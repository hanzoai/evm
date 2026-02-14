use hanzo_evm_db_api::table::Table;

/// The trait for fetching provider statistics.
#[auto_impl::auto_impl(&)]
pub trait StatsReader {
    /// Fetch the number of entries in the corresponding [Table]. Depending on the provider, it may
    /// route to different data sources other than [Table].
    fn count_entries<T: Table>(&self) -> hanzo_evm_storage_errors::provider::ProviderResult<usize>;
}
