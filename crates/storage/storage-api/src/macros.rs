//! Helper macros for implementing traits for various `StateProvider`
//! implementations

/// A macro that delegates trait implementations to the `as_ref` function of the type.
///
/// Used to implement provider traits.
#[macro_export]
macro_rules! delegate_impls_to_as_ref {
    (for $target:ty => $($trait:ident $(where [$($generics:tt)*])? {  $(fn $func:ident$(<$($generic_arg:ident: $generic_arg_ty:path),*>)?(&self, $($arg:ident: $argty:ty),*) -> $ret:path;)* })* ) => {

        $(
          impl<'a, $($($generics)*)?> $trait for $target {
              $(
                  fn $func$(<$($generic_arg: $generic_arg_ty),*>)?(&self, $($arg: $argty),*) -> $ret {
                    self.as_ref().$func($($arg),*)
                  }
              )*
          }
        )*
    };
}

pub use delegate_impls_to_as_ref;

/// Delegates the provider trait implementations to the `as_ref` function of the type:
///
/// [`AccountReader`](crate::AccountReader)
/// [`BlockHashReader`](crate::BlockHashReader)
/// [`StateProvider`](crate::StateProvider)
#[macro_export]
macro_rules! delegate_provider_impls {
    ($target:ty $(where [$($generics:tt)*])?) => {
        $crate::macros::delegate_impls_to_as_ref!(
            for $target =>
            AccountReader $(where [$($generics)*])? {
                fn basic_account(&self, address: &alloy_primitives::Address) -> hanzo_evm_storage_api::errors::provider::ProviderResult<Option<hanzo_evm_primitives_traits::Account>>;
            }
            BlockHashReader $(where [$($generics)*])? {
                fn block_hash(&self, number: u64) -> hanzo_evm_storage_api::errors::provider::ProviderResult<Option<alloy_primitives::B256>>;
                fn canonical_hashes_range(&self, start: alloy_primitives::BlockNumber, end: alloy_primitives::BlockNumber) -> hanzo_evm_storage_api::errors::provider::ProviderResult<Vec<alloy_primitives::B256>>;
            }
            StateProvider $(where [$($generics)*])? {
                fn storage(&self, account: alloy_primitives::Address, storage_key: alloy_primitives::StorageKey) -> reth_storage_api::errors::provider::ProviderResult<Option<alloy_primitives::StorageValue>>;
                fn storage_by_hashed_key(&self, address: alloy_primitives::Address, hashed_storage_key: alloy_primitives::StorageKey) -> reth_storage_api::errors::provider::ProviderResult<Option<alloy_primitives::StorageValue>>;
            }
            BytecodeReader $(where [$($generics)*])? {
                fn bytecode_by_hash(&self, code_hash: &alloy_primitives::B256) -> hanzo_evm_storage_api::errors::provider::ProviderResult<Option<hanzo_evm_primitives_traits::Bytecode>>;
            }
            StateRootProvider $(where [$($generics)*])? {
                fn state_root(&self, state: hanzo_evm_trie::HashedPostState) -> hanzo_evm_storage_api::errors::provider::ProviderResult<alloy_primitives::B256>;
                fn state_root_from_nodes(&self, input: hanzo_evm_trie::TrieInput) -> hanzo_evm_storage_api::errors::provider::ProviderResult<alloy_primitives::B256>;
                fn state_root_with_updates(&self, state: hanzo_evm_trie::HashedPostState) -> hanzo_evm_storage_api::errors::provider::ProviderResult<(alloy_primitives::B256, hanzo_evm_trie::updates::TrieUpdates)>;
                fn state_root_from_nodes_with_updates(&self, input: hanzo_evm_trie::TrieInput) -> hanzo_evm_storage_api::errors::provider::ProviderResult<(alloy_primitives::B256, hanzo_evm_trie::updates::TrieUpdates)>;
            }
            StorageRootProvider $(where [$($generics)*])? {
                fn storage_root(&self, address: alloy_primitives::Address, storage: hanzo_evm_trie::HashedStorage) -> hanzo_evm_storage_api::errors::provider::ProviderResult<alloy_primitives::B256>;
                fn storage_proof(&self, address: alloy_primitives::Address, slot: alloy_primitives::B256, storage: hanzo_evm_trie::HashedStorage) -> hanzo_evm_storage_api::errors::provider::ProviderResult<hanzo_evm_trie::StorageProof>;
                fn storage_multiproof(&self, address: alloy_primitives::Address, slots: &[alloy_primitives::B256], storage: hanzo_evm_trie::HashedStorage) -> hanzo_evm_storage_api::errors::provider::ProviderResult<hanzo_evm_trie::StorageMultiProof>;
            }
            StateProofProvider $(where [$($generics)*])? {
                fn proof(&self, input: hanzo_evm_trie::TrieInput, address: alloy_primitives::Address, slots: &[alloy_primitives::B256]) -> hanzo_evm_storage_api::errors::provider::ProviderResult<hanzo_evm_trie::AccountProof>;
                fn multiproof(&self, input: hanzo_evm_trie::TrieInput, targets: hanzo_evm_trie::MultiProofTargets) -> hanzo_evm_storage_api::errors::provider::ProviderResult<hanzo_evm_trie::MultiProof>;
                fn witness(&self, input: hanzo_evm_trie::TrieInput, target: hanzo_evm_trie::HashedPostState) -> hanzo_evm_storage_api::errors::provider::ProviderResult<Vec<alloy_primitives::Bytes>>;
            }
            HashedPostStateProvider $(where [$($generics)*])? {
                fn hashed_post_state(&self, bundle_state: &revm_database::BundleState) -> hanzo_evm_trie::HashedPostState;
            }
        );
    }
}

pub use delegate_provider_impls;
