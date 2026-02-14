//! High level error types for the evm in general.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/hanzoai/evm/main/assets/evm-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/hanzoai/evm/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]

extern crate alloc;

mod error;
pub use error::{EvmError, EvmResult};

pub use hanzo_evm_consensus::ConsensusError;
pub use hanzo_evm_execution_errors::{BlockExecutionError, BlockValidationError};
pub use hanzo_evm_storage_errors::{
    db::DatabaseError,
    provider::{ProviderError, ProviderResult},
};
