use crate::{
    providers::{NodeTypesForProvider, ProviderNodeTypes, RocksDBBuilder, StaticFileProvider},
    HashingWriter, ProviderFactory, TrieWriter,
};
use alloy_primitives::B256;
use hanzo_evm_chainspec::{ChainSpec, MAINNET};
use hanzo_evm_db::{test_utils::TempDatabase, DatabaseEnv};
use hanzo_evm_errors::ProviderResult;
use hanzo_evm_ethereum_engine_primitives::EthEngineTypes;
use hanzo_evm_node_types::NodeTypesWithDBAdapter;
use hanzo_evm_primitives_traits::{Account, StorageEntry};
use hanzo_evm_trie::StateRoot;
use hanzo_evm_trie_db::DatabaseStateRoot;
use std::sync::Arc;

pub mod blocks;
mod mock;
mod noop;

pub use mock::{ExtendedAccount, MockEthProvider};
pub use noop::NoopProvider;
pub use hanzo_evm_chain_state::test_utils::TestCanonStateSubscriptions;

/// Mock [`hanzo_evm_node_types::NodeTypes`] for testing.
pub type MockNodeTypes = hanzo_evm_node_types::AnyNodeTypesWithEngine<
    hanzo_evm_ethereum_primitives::EthPrimitives,
    hanzo_evm_ethereum_engine_primitives::EthEngineTypes,
    hanzo_evm_chainspec::ChainSpec,
    crate::EthStorage,
    EthEngineTypes,
>;

/// Mock [`hanzo_evm_node_types::NodeTypesWithDB`] for testing.
pub type MockNodeTypesWithDB<DB = Arc<TempDatabase<DatabaseEnv>>> =
    NodeTypesWithDBAdapter<MockNodeTypes, DB>;

/// Creates test provider factory with mainnet chain spec.
pub fn create_test_provider_factory() -> ProviderFactory<MockNodeTypesWithDB> {
    create_test_provider_factory_with_chain_spec(MAINNET.clone())
}

/// Creates test provider factory with provided chain spec.
pub fn create_test_provider_factory_with_chain_spec(
    chain_spec: Arc<ChainSpec>,
) -> ProviderFactory<MockNodeTypesWithDB> {
    create_test_provider_factory_with_node_types::<MockNodeTypes>(chain_spec)
}

/// Creates test provider factory with provided chain spec.
pub fn create_test_provider_factory_with_node_types<N: NodeTypesForProvider>(
    chain_spec: Arc<N::ChainSpec>,
) -> ProviderFactory<NodeTypesWithDBAdapter<N, Arc<TempDatabase<DatabaseEnv>>>> {
    // Create a single temp directory that contains all data dirs (db, static_files, rocksdb).
    // TempDatabase will clean up the entire directory on drop.
    let datadir_path = hanzo_evm_db::test_utils::tempdir_path();

    let static_files_path = datadir_path.join("static_files");
    let rocksdb_path = datadir_path.join("rocksdb");

    // Create static_files directory
    std::fs::create_dir_all(&static_files_path).expect("failed to create static_files dir");

    // Create database with the datadir path so TempDatabase cleans up everything on drop
    let db = hanzo_evm_db::test_utils::create_test_rw_db_with_datadir(&datadir_path);

    ProviderFactory::new(
        db,
        chain_spec,
        StaticFileProvider::read_write(static_files_path).expect("static file provider"),
        RocksDBBuilder::new(&rocksdb_path)
            .with_default_tables()
            .build()
            .expect("failed to create test RocksDB provider"),
        reth_tasks::Runtime::test(),
    )
    .expect("failed to create test provider factory")
}

/// Inserts the genesis alloc from the provided chain spec into the trie.
pub fn insert_genesis<N: ProviderNodeTypes<ChainSpec = ChainSpec>>(
    provider_factory: &ProviderFactory<N>,
    chain_spec: Arc<N::ChainSpec>,
) -> ProviderResult<B256> {
    let provider = provider_factory.provider_rw()?;

    // Hash accounts and insert them into hashing table.
    let genesis = chain_spec.genesis();
    let alloc_accounts =
        genesis.alloc.iter().map(|(addr, account)| (*addr, Some(Account::from(account))));
    provider.insert_account_for_hashing(alloc_accounts).unwrap();

    let alloc_storage = genesis.alloc.clone().into_iter().filter_map(|(addr, account)| {
        // Only return `Some` if there is storage.
        account.storage.map(|storage| {
            (
                addr,
                storage.into_iter().map(|(key, value)| StorageEntry { key, value: value.into() }),
            )
        })
    });
    provider.insert_storage_for_hashing(alloc_storage)?;

    let (root, updates) = StateRoot::from_tx(provider.tx_ref()).root_with_updates()?;
    provider.write_trie_updates(updates).unwrap();

    provider.commit()?;

    Ok(root)
}
