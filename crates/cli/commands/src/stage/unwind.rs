//! Unwinding a certain block range

use crate::{
    common::{AccessRights, CliNodeTypes, Environment, EnvironmentArgs},
    stage::CliNodeComponents,
};
use alloy_eips::BlockHashOrNumber;
use alloy_primitives::B256;
use clap::{Parser, Subcommand};
use hanzo_evm_chainspec::{ChainSpecProvider, EthChainSpec, EthereumHardforks};
use hanzo_evm_cli::chainspec::ChainSpecParser;
use hanzo_evm_config::Config;
use hanzo_evm_consensus::noop::NoopConsensus;
use hanzo_evm_db::DatabaseEnv;
use hanzo_evm_downloaders::{bodies::noop::NoopBodiesDownloader, headers::noop::NoopHeaderDownloader};
use hanzo_evm_execution::ConfigureEvm;
use hanzo_evm_exex::ExExManagerHandle;
use hanzo_evm_provider::{providers::ProviderNodeTypes, BlockNumReader, ProviderFactory};
use hanzo_evm_stages::{
    sets::{DefaultStages, OfflineStages},
    stages::ExecutionStage,
    ExecutionStageThresholds, Pipeline, StageSet,
};
use hanzo_evm_static_file::StaticFileProducer;
use std::sync::Arc;
use tokio::sync::watch;
use tracing::info;

/// `evm stage unwind` command
#[derive(Debug, Parser)]
pub struct Command<C: ChainSpecParser> {
    #[command(flatten)]
    env: EnvironmentArgs<C>,

    #[command(subcommand)]
    command: Subcommands,

    /// If this is enabled, then all stages except headers, bodies, and sender recovery will be
    /// unwound.
    #[arg(long)]
    offline: bool,
}

impl<C: ChainSpecParser<ChainSpec: EthChainSpec + EthereumHardforks>> Command<C> {
    /// Execute `db stage unwind` command
    pub async fn execute<N: CliNodeTypes<ChainSpec = C::ChainSpec>, F, Comp>(
        self,
        components: F,
        runtime: reth_tasks::Runtime,
    ) -> eyre::Result<()>
    where
        Comp: CliNodeComponents<N>,
        F: FnOnce(Arc<C::ChainSpec>) -> Comp,
    {
        let Environment { provider_factory, config, .. } =
            self.env.init::<N>(AccessRights::RW, runtime)?;

        let target = self.command.unwind_target(provider_factory.clone())?;

        let components = components(provider_factory.chain_spec());

        if self.offline {
            info!(target: "evm::cli", "Performing an unwind for offline-only data!");
        }

        let highest_static_file_block = provider_factory.provider()?.last_block_number()?;
        info!(target: "evm::cli", ?target, ?highest_static_file_block, prune_config=?config.prune,  "Executing a pipeline unwind.");

        // This will build an offline-only pipeline if the `offline` flag is enabled
        let mut pipeline =
            self.build_pipeline(config, provider_factory, components.hanzo_evm_config().clone())?;

        // Move all applicable data from database to static files.
        pipeline.move_to_static_files()?;

        pipeline.unwind(target, None)?;

        info!(target: "evm::cli", ?target, "Unwound blocks");

        Ok(())
    }

    fn build_pipeline<N: ProviderNodeTypes<ChainSpec = C::ChainSpec>>(
        self,
        config: Config,
        provider_factory: ProviderFactory<N>,
        hanzo_evm_config: impl ConfigureEvm<Primitives = N::Primitives> + 'static,
    ) -> Result<Pipeline<N>, eyre::Error> {
        let stage_conf = &config.stages;
        let prune_modes = config.prune.segments.clone();

        let (tip_tx, tip_rx) = watch::channel(B256::ZERO);

        let builder = if self.offline {
            Pipeline::<N>::builder().add_stages(
                OfflineStages::new(
                    hanzo_evm_config,
                    NoopConsensus::arc(),
                    config.stages,
                    prune_modes.clone(),
                )
                .builder()
                .disable(hanzo_evm_stages::StageId::SenderRecovery),
            )
        } else {
            Pipeline::<N>::builder().with_tip_sender(tip_tx).add_stages(
                DefaultStages::new(
                    provider_factory.clone(),
                    tip_rx,
                    Arc::new(NoopConsensus::default()),
                    NoopHeaderDownloader::default(),
                    NoopBodiesDownloader::default(),
                    hanzo_evm_config.clone(),
                    stage_conf.clone(),
                    prune_modes.clone(),
                    None,
                )
                .set(ExecutionStage::new(
                    hanzo_evm_config,
                    Arc::new(NoopConsensus::default()),
                    ExecutionStageThresholds {
                        max_blocks: None,
                        max_changes: None,
                        max_cumulative_gas: None,
                        max_duration: None,
                    },
                    stage_conf.execution_external_clean_threshold(),
                    ExExManagerHandle::empty(),
                )),
            )
        };

        let pipeline = builder.build(
            provider_factory.clone(),
            StaticFileProducer::new(provider_factory, prune_modes),
        );
        Ok(pipeline)
    }
}

impl<C: ChainSpecParser> Command<C> {
    /// Return the underlying chain being used to run this command
    pub fn chain_spec(&self) -> Option<&Arc<C::ChainSpec>> {
        Some(&self.env.chain)
    }
}

/// `evm stage unwind` subcommand
#[derive(Subcommand, Debug, Eq, PartialEq)]
enum Subcommands {
    /// Unwinds the database from the latest block, until the given block number or hash has been
    /// reached, that block is not included.
    #[command(name = "to-block")]
    ToBlock { target: BlockHashOrNumber },
    /// Unwinds the database from the latest block, until the given number of blocks have been
    /// reached.
    #[command(name = "num-blocks")]
    NumBlocks { amount: u64 },
}

impl Subcommands {
    /// Returns the block to unwind to. The returned block will stay in database.
    fn unwind_target<N: ProviderNodeTypes<DB = DatabaseEnv>>(
        &self,
        factory: ProviderFactory<N>,
    ) -> eyre::Result<u64> {
        let provider = factory.provider()?;
        let last = provider.last_block_number()?;
        let target = match self {
            Self::ToBlock { target } => match target {
                BlockHashOrNumber::Hash(hash) => provider
                    .block_number(*hash)?
                    .ok_or_else(|| eyre::eyre!("Block hash not found in database: {hash:?}"))?,
                BlockHashOrNumber::Number(num) => *num,
            },
            Self::NumBlocks { amount } => last.saturating_sub(*amount),
        };
        if target > last {
            eyre::bail!(
                "Target block number {target} is higher than the latest block number {last}"
            )
        }
        Ok(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hanzo_evm_chainspec::SEPOLIA;
    use hanzo_evm_ethereum_cli::chainspec::EthereumChainSpecParser;

    #[test]
    fn parse_unwind() {
        let cmd = Command::<EthereumChainSpecParser>::parse_from([
            "evm",
            "--datadir",
            "dir",
            "to-block",
            "100",
        ]);
        assert_eq!(cmd.command, Subcommands::ToBlock { target: BlockHashOrNumber::Number(100) });

        let cmd = Command::<EthereumChainSpecParser>::parse_from([
            "evm",
            "--datadir",
            "dir",
            "num-blocks",
            "100",
        ]);
        assert_eq!(cmd.command, Subcommands::NumBlocks { amount: 100 });
    }

    #[test]
    fn parse_unwind_chain() {
        let cmd = Command::<EthereumChainSpecParser>::parse_from([
            "evm", "--chain", "sepolia", "to-block", "100",
        ]);
        assert_eq!(cmd.command, Subcommands::ToBlock { target: BlockHashOrNumber::Number(100) });
        assert_eq!(cmd.env.chain.chain_id(), SEPOLIA.chain_id());
    }
}
