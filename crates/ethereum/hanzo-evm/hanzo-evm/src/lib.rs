//! Ethereum meta crate that provides access to commonly used evm dependencies.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/hanzoai/evm/main/assets/evm-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/hanzoai/evm/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

/// Re-exported ethereum types
#[doc(inline)]
pub use hanzo_evm_ethereum_primitives::*;

/// Re-exported evm primitives
pub mod primitives {
    #[doc(inline)]
    pub use hanzo_evm_primitives_traits::*;
}

/// Re-exported cli types
#[cfg(feature = "cli")]
pub mod cli {
    #[doc(inline)]
    pub use hanzo_evm_cli_util::*;
    #[doc(inline)]
    pub use hanzo_evm_ethereum_cli::*;
}

/// Re-exported pool types
#[cfg(feature = "pool")]
pub use hanzo_evm_transaction_pool as pool;

/// Re-exported consensus types
#[cfg(feature = "consensus")]
pub mod consensus {
    #[doc(inline)]
    pub use hanzo_evm_consensus::*;
    pub use hanzo_evm_consensus_common::*;
    pub use hanzo_evm_ethereum_consensus::*;
}

/// Re-exported from `hanzo_evm_chainspec`
pub mod chainspec {
    #[doc(inline)]
    pub use hanzo_evm_chainspec::*;
}

/// Re-exported evm types
#[cfg(feature = "evm")]
pub mod evm {
    #[doc(inline)]
    pub use hanzo_evm_eth_execution::*;

    #[doc(inline)]
    pub use hanzo_evm_execution as primitives;

    #[doc(inline)]
    pub use hanzo_evm_revm as revm;
}

/// Re-exported exex types
#[cfg(feature = "exex")]
pub use hanzo_evm_exex as exex;

/// Re-exported from `tasks`.
#[cfg(feature = "tasks")]
pub mod tasks {
    pub use hanzo_evm_tasks::*;
}

/// Re-exported evm network types
#[cfg(feature = "network")]
pub mod network {
    #[doc(inline)]
    pub use hanzo_evm_eth_wire as eth_wire;
    #[doc(inline)]
    pub use hanzo_evm_network::*;
    #[doc(inline)]
    pub use hanzo_evm_network_api as api;
}

/// Re-exported evm provider types
#[cfg(feature = "provider")]
pub mod provider {
    #[doc(inline)]
    pub use hanzo_evm_provider::*;

    #[doc(inline)]
    pub use hanzo_evm_db as db;
}

/// Re-exported codec crate
#[cfg(feature = "provider")]
pub use hanzo_evm_codecs as codec;

/// Re-exported evm storage api types
#[cfg(feature = "storage-api")]
pub mod storage {
    #[doc(inline)]
    pub use hanzo_evm_storage_api::*;
}

/// Re-exported ethereum node
#[cfg(feature = "node-api")]
pub mod node {
    #[doc(inline)]
    pub use hanzo_evm_node_api as api;
    #[cfg(feature = "node")]
    pub use hanzo_evm_node_builder as builder;
    #[doc(inline)]
    pub use hanzo_evm_node_core as core;
    #[cfg(feature = "node")]
    pub use hanzo_evm_node_ethereum::*;
}

/// Re-exported ethereum engine types
#[cfg(feature = "node")]
pub mod engine {
    #[doc(inline)]
    pub use hanzo_evm_engine_local as local;
    #[doc(inline)]
    pub use hanzo_evm_node_ethereum::engine::*;
}

/// Re-exported evm trie types
#[cfg(feature = "trie")]
pub mod trie {
    #[doc(inline)]
    pub use hanzo_evm_trie::*;

    #[cfg(feature = "trie-db")]
    #[doc(inline)]
    pub use hanzo_evm_trie_db::*;
}

/// Re-exported rpc types
#[cfg(feature = "rpc")]
pub mod rpc {
    #[doc(inline)]
    pub use hanzo_evm_rpc::*;

    #[doc(inline)]
    pub use hanzo_evm_rpc_api as api;
    #[doc(inline)]
    pub use hanzo_evm_rpc_builder as builder;

    /// Re-exported eth types
    #[allow(ambiguous_glob_reexports)]
    pub mod eth {
        #[doc(inline)]
        pub use alloy_rpc_types_eth as primitives;
        #[doc(inline)]
        pub use hanzo_evm_rpc_eth_types::*;

        pub use hanzo_evm_rpc::eth::*;
    }

    /// Re-exported types
    pub mod types {
        #[doc(inline)]
        pub use alloy_rpc_types_engine as engine;
    }
}
