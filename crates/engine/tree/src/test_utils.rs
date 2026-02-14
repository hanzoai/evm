use alloy_primitives::B256;
use hanzo_evm_chainspec::ChainSpec;
use hanzo_evm_ethereum_primitives::BlockBody;
use hanzo_evm_network_p2p::test_utils::TestFullBlockClient;
use hanzo_evm_primitives_traits::SealedHeader;
use hanzo_evm_provider::test_utils::{
    create_test_provider_factory_with_chain_spec, MockNodeTypesWithDB,
};
use hanzo_evm_prune_types::PruneModes;
use hanzo_evm_stages::{test_utils::TestStages, ExecOutput, StageError};
use hanzo_evm_stages_api::Pipeline;
use hanzo_evm_static_file::StaticFileProducer;
use std::{collections::VecDeque, ops::Range, sync::Arc};
use tokio::sync::watch;

/// Test pipeline builder.
#[derive(Default, Debug)]
pub struct TestPipelineBuilder {
    pipeline_exec_outputs: VecDeque<Result<ExecOutput, StageError>>,
}

impl TestPipelineBuilder {
    /// Create a new [`TestPipelineBuilder`].
    pub const fn new() -> Self {
        Self { pipeline_exec_outputs: VecDeque::new() }
    }

    /// Set the pipeline execution outputs to use for the test consensus engine.
    pub fn with_pipeline_exec_outputs(
        mut self,
        pipeline_exec_outputs: VecDeque<Result<ExecOutput, StageError>>,
    ) -> Self {
        self.pipeline_exec_outputs = pipeline_exec_outputs;
        self
    }

    /// Set the executor results to use for the test consensus engine.
    #[deprecated(
        note = "no-op: executor results are not used and will be removed in a future release"
    )]
    pub fn with_executor_results(
        self,
        executor_results: Vec<hanzo_evm_provider::ExecutionOutcome>,
    ) -> Self {
        let _ = executor_results;
        self
    }

    /// Builds the pipeline.
    pub fn build(self, chain_spec: Arc<ChainSpec>) -> Pipeline<MockNodeTypesWithDB> {
        hanzo_evm_tracing::init_test_tracing();

        // Setup pipeline
        let (tip_tx, _tip_rx) = watch::channel(B256::default());
        let pipeline = Pipeline::<MockNodeTypesWithDB>::builder()
            .add_stages(TestStages::new(self.pipeline_exec_outputs, Default::default()))
            .with_tip_sender(tip_tx);

        let provider_factory = create_test_provider_factory_with_chain_spec(chain_spec);

        let static_file_producer =
            StaticFileProducer::new(provider_factory.clone(), PruneModes::default());

        pipeline.build(provider_factory, static_file_producer)
    }
}

/// Starting from the given genesis header, inserts headers from the given
/// range in the given test full block client.
pub fn insert_headers_into_client(
    client: &TestFullBlockClient,
    genesis_header: SealedHeader,
    range: Range<usize>,
) {
    let mut sealed_header = genesis_header;
    let body = BlockBody::default();
    for _ in range {
        let (mut header, hash) = sealed_header.split();
        // update to the next header
        header.parent_hash = hash;
        header.number += 1;
        header.timestamp += 1;
        sealed_header = SealedHeader::seal_slow(header);
        client.insert(sealed_header.clone(), body.clone());
    }
}
