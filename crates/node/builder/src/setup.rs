//! Helpers for setting up parts of the node.

use std::sync::Arc;

use crate::BlockTy;
use alloy_primitives::{BlockNumber, B256};
use hanzo_evm_config::{config::StageConfig, PruneConfig};
use hanzo_evm_consensus::FullConsensus;
use hanzo_evm_downloaders::{
    bodies::bodies::BodiesDownloaderBuilder,
    headers::reverse_headers::ReverseHeadersDownloaderBuilder,
};
use hanzo_evm_execution::ConfigureEvm;
use hanzo_evm_exex::ExExManagerHandle;
use hanzo_evm_network_p2p::{
    bodies::downloader::BodyDownloader, headers::downloader::HeaderDownloader, BlockClient,
};
use hanzo_evm_node_api::HeaderTy;
use hanzo_evm_provider::{providers::ProviderNodeTypes, ProviderFactory};
use hanzo_evm_stages::{
    prelude::DefaultStages,
    stages::{EraImportSource, ExecutionStage},
    Pipeline, StageSet,
};
use hanzo_evm_static_file::StaticFileProducer;
use hanzo_evm_tasks::TaskExecutor;
use hanzo_evm_tracing::tracing::debug;
use tokio::sync::watch;

/// Constructs a [Pipeline] that's wired to the network
#[expect(clippy::too_many_arguments)]
pub fn build_networked_pipeline<N, Client, Evm>(
    config: &StageConfig,
    client: Client,
    consensus: Arc<dyn FullConsensus<N::Primitives>>,
    provider_factory: ProviderFactory<N>,
    task_executor: &TaskExecutor,
    metrics_tx: hanzo_evm_stages::MetricEventsSender,
    prune_config: PruneConfig,
    max_block: Option<BlockNumber>,
    static_file_producer: StaticFileProducer<ProviderFactory<N>>,
    hanzo_evm_config: Evm,
    exex_manager_handle: ExExManagerHandle<N::Primitives>,
    era_import_source: Option<EraImportSource>,
) -> eyre::Result<Pipeline<N>>
where
    N: ProviderNodeTypes,
    Client: BlockClient<Block = BlockTy<N>> + 'static,
    Evm: ConfigureEvm<Primitives = N::Primitives> + 'static,
{
    // building network downloaders using the fetch client
    let header_downloader = ReverseHeadersDownloaderBuilder::new(config.headers)
        .build(client.clone(), consensus.clone())
        .into_task_with(task_executor);

    let body_downloader = BodiesDownloaderBuilder::new(config.bodies)
        .build(client, consensus.clone(), provider_factory.clone())
        .into_task_with(task_executor);

    let pipeline = build_pipeline(
        provider_factory,
        config,
        header_downloader,
        body_downloader,
        consensus,
        max_block,
        metrics_tx,
        prune_config,
        static_file_producer,
        hanzo_evm_config,
        exex_manager_handle,
        era_import_source,
    )?;

    Ok(pipeline)
}

/// Builds the [Pipeline] with the given [`ProviderFactory`] and downloaders.
#[expect(clippy::too_many_arguments)]
pub fn build_pipeline<N, H, B, Evm>(
    provider_factory: ProviderFactory<N>,
    stage_config: &StageConfig,
    header_downloader: H,
    body_downloader: B,
    consensus: Arc<dyn FullConsensus<N::Primitives>>,
    max_block: Option<u64>,
    metrics_tx: hanzo_evm_stages::MetricEventsSender,
    prune_config: PruneConfig,
    static_file_producer: StaticFileProducer<ProviderFactory<N>>,
    hanzo_evm_config: Evm,
    exex_manager_handle: ExExManagerHandle<N::Primitives>,
    era_import_source: Option<EraImportSource>,
) -> eyre::Result<Pipeline<N>>
where
    N: ProviderNodeTypes,
    H: HeaderDownloader<Header = HeaderTy<N>> + 'static,
    B: BodyDownloader<Block = BlockTy<N>> + 'static,
    Evm: ConfigureEvm<Primitives = N::Primitives> + 'static,
{
    let mut builder = Pipeline::<N>::builder();

    if let Some(max_block) = max_block {
        debug!(target: "evm::cli", max_block, "Configuring builder to use max block");
        builder = builder.with_max_block(max_block)
    }

    let (tip_tx, tip_rx) = watch::channel(B256::ZERO);

    let pipeline = builder
        .with_tip_sender(tip_tx)
        .with_metrics_tx(metrics_tx)
        .add_stages(
            DefaultStages::new(
                provider_factory.clone(),
                tip_rx,
                Arc::clone(&consensus),
                header_downloader,
                body_downloader,
                hanzo_evm_config.clone(),
                stage_config.clone(),
                prune_config.segments,
                era_import_source,
            )
            .set(ExecutionStage::new(
                hanzo_evm_config,
                consensus,
                stage_config.execution.into(),
                stage_config.execution_external_clean_threshold(),
                exex_manager_handle,
            )),
        )
        .build(provider_factory, static_file_producer);

    Ok(pipeline)
}
