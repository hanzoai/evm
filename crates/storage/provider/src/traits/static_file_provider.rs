use alloy_primitives::BlockNumber;
use hanzo_evm_errors::ProviderResult;
use hanzo_evm_static_file_types::StaticFileSegment;
use hanzo_evm_storage_api::NodePrimitivesProvider;

use crate::providers::{StaticFileProvider, StaticFileProviderRWRefMut};

/// Static file provider factory.
pub trait StaticFileProviderFactory: NodePrimitivesProvider {
    /// Create new instance of static file provider.
    fn static_file_provider(&self) -> StaticFileProvider<Self::Primitives>;

    /// Returns a mutable reference to a
    /// [`StaticFileProviderRW`](`crate::providers::StaticFileProviderRW`) of a
    /// [`StaticFileSegment`].
    fn get_static_file_writer(
        &self,
        block: BlockNumber,
        segment: StaticFileSegment,
    ) -> ProviderResult<StaticFileProviderRWRefMut<'_, Self::Primitives>>;
}
