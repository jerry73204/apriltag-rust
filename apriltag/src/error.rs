//! Defines the error type for the crate.

/// The error type for the crate.
#[derive(Debug, Clone, thiserror::Error)]
pub enum Error {
    #[error("unable to parse family string '{{0}}'")]
    ParseFamilyStringError(String),

    #[error("Unable to create an image: {reason}")]
    CreateImageError { reason: String },

    #[error("Unable to create a detector: {reason}")]
    CreateDetectorError { reason: String },
}
