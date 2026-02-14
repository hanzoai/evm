//! Hanzo EVM RPC server types.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/hanzoai/evm/main/assets/evm-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/hanzoai/evm/issues/"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]

/// Common RPC constants.
pub mod constants;
pub mod result;

mod module;
pub use module::{
    DefaultRpcModuleValidator, LenientRpcModuleValidator, EvmRpcModule, RpcModuleSelection,
    RpcModuleValidator,
};

pub use result::ToRpcResult;
