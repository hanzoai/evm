use hanzo_evm_errors::{DatabaseError, EvmError};
use hanzo_evm_provider::ProviderError;
use hanzo_evm_prune_types::PruneSegmentError;
use thiserror::Error;

/// Errors that can occur during pruning.
#[derive(Error, Debug)]
pub enum PrunerError {
    #[error(transparent)]
    PruneSegment(#[from] PruneSegmentError),

    #[error("inconsistent data: {0}")]
    InconsistentData(&'static str),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    Provider(#[from] ProviderError),
}

impl From<PrunerError> for EvmError {
    fn from(err: PrunerError) -> Self {
        match err {
            PrunerError::PruneSegment(_) | PrunerError::InconsistentData(_) => Self::other(err),
            PrunerError::Database(err) => Self::Database(err),
            PrunerError::Provider(err) => Self::Provider(err),
        }
    }
}
