//! `StaticFile` segment implementations and utilities.

mod receipts;
pub use receipts::Receipts;

use alloy_primitives::BlockNumber;
use hanzo_evm_provider::StaticFileProviderFactory;
use hanzo_evm_static_file_types::StaticFileSegment;
use hanzo_evm_storage_errors::provider::ProviderResult;
use std::ops::RangeInclusive;

/// A segment represents moving some portion of the data to static files.
pub trait Segment<Provider: StaticFileProviderFactory>: Send + Sync {
    /// Returns the [`StaticFileSegment`].
    fn segment(&self) -> StaticFileSegment;

    /// Move data to static files for the provided block range.
    /// [`StaticFileProvider`](hanzo_evm_provider::providers::StaticFileProvider) will handle
    /// the management of and writing to files.
    fn copy_to_static_files(
        &self,
        provider: Provider,
        block_range: RangeInclusive<BlockNumber>,
    ) -> ProviderResult<()>;
}
