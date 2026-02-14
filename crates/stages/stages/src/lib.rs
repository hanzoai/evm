//! Staged syncing primitives for evm.
//!
//! This crate contains the syncing primitives [`Pipeline`] and [`Stage`], as well as all stages
//! that evm uses to sync.
//!
//! A pipeline can be configured using [`Pipeline::builder()`].
//!
//! For ease of use, this crate also exposes a set of [`StageSet`]s, which are collections of stages
//! that perform specific functions during sync. Stage sets can be customized; it is possible to
//! add, disable and replace stages in the set.
//!
//! # Examples
//!
//! ```
//! # use std::sync::Arc;
//! # use hanzo_evm_downloaders::bodies::bodies::BodiesDownloaderBuilder;
//! # use hanzo_evm_downloaders::headers::reverse_headers::ReverseHeadersDownloaderBuilder;
//! # use hanzo_evm_network_p2p::test_utils::{TestBodiesClient, TestHeadersClient};
//! # use alloy_primitives::B256;
//! # use hanzo_evm_chainspec::MAINNET;
//! # use hanzo_evm_prune_types::PruneModes;
//! # use hanzo_evm_network_peers::PeerId;
//! # use hanzo_evm_stages::Pipeline;
//! # use hanzo_evm_stages::sets::DefaultStages;
//! # use tokio::sync::watch;
//! # use hanzo_evm_eth_execution::EthEvmConfig;
//! # use hanzo_evm_provider::ProviderFactory;
//! # use hanzo_evm_provider::StaticFileProviderFactory;
//! # use hanzo_evm_provider::test_utils::{create_test_provider_factory, MockNodeTypesWithDB};
//! # use hanzo_evm_static_file::StaticFileProducer;
//! # use hanzo_evm_config::config::StageConfig;
//! # use hanzo_evm_consensus::Consensus;
//! # use hanzo_evm_consensus::test_utils::TestConsensus;
//! # use hanzo_evm_consensus::FullConsensus;
//! #
//! # let chain_spec = MAINNET.clone();
//! # let consensus: Arc<dyn FullConsensus<hanzo_evm_ethereum_primitives::EthPrimitives>> = Arc::new(TestConsensus::default());
//! # let headers_downloader = ReverseHeadersDownloaderBuilder::default().build(
//! #    Arc::new(TestHeadersClient::default()),
//! #    consensus.clone()
//! # );
//! # let provider_factory = create_test_provider_factory();
//! # let bodies_downloader = BodiesDownloaderBuilder::default().build(
//! #    Arc::new(TestBodiesClient { responder: |_| Ok((PeerId::ZERO, vec![]).into()) }),
//! #    consensus.clone(),
//! #    provider_factory.clone()
//! # );
//! # let (tip_tx, tip_rx) = watch::channel(B256::default());
//! # let executor_provider = EthEvmConfig::mainnet();
//! # let static_file_producer = StaticFileProducer::new(
//! #    provider_factory.clone(),
//! #    PruneModes::default()
//! # );
//! # let era_import_source = None;
//! // Create a pipeline that can fully sync
//! # let pipeline =
//! Pipeline::<MockNodeTypesWithDB>::builder()
//!     .with_tip_sender(tip_tx)
//!     .add_stages(DefaultStages::new(
//!         provider_factory.clone(),
//!         tip_rx,
//!         consensus,
//!         headers_downloader,
//!         bodies_downloader,
//!         executor_provider,
//!         StageConfig::default(),
//!         PruneModes::default(),
//!         era_import_source,
//!     ))
//!     .build(provider_factory, static_file_producer);
//! ```
//!
//! ## Feature Flags
//!
//! - `test-utils`: Export utilities for testing

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/hanzoai/evm/main/assets/evm-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/hanzoai/evm/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

#[expect(missing_docs)]
#[cfg(any(test, feature = "test-utils"))]
pub mod test_utils;

/// A re-export of common structs and traits.
pub mod prelude;

/// Implementations of stages.
pub mod stages;

pub mod sets;

// re-export the stages API
pub use hanzo_evm_stages_api::*;
