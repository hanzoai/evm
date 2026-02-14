//! Hanzo EVM compatibility and utils for RPC types
//!
//! This crate various helper functions to convert between evm primitive types and rpc types.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/hanzoai/evm/main/assets/evm-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/hanzoai/evm/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod block;
pub mod receipt;
mod rpc;
pub mod transaction;

pub use block::TryFromBlockResponse;
pub use receipt::TryFromReceiptResponse;
pub use rpc::*;
pub use transaction::{
    RpcConvert, RpcConverter, TransactionConversionError, TryFromTransactionResponse, TryIntoSimTx,
    TxInfoMapper,
};

pub use alloy_evm::rpc::{CallFees, CallFeesError, EthTxEnvError, TryIntoTxEnv};
