//! Error types for Czech File Knife

use thiserror::Error;

/// Result type alias
pub type CfkResult<T> = Result<T, CfkError>;

/// Main error type
#[derive(Error, Debug)]
pub enum CfkError {
    #[error("Path not found: {0}")]
    NotFound(String),

    #[error("Already exists: {0}")]
    AlreadyExists(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Not a directory: {0}")]
    NotADirectory(String),

    #[error("Not a file: {0}")]
    NotAFile(String),

    #[error("Directory not empty: {0}")]
    DirectoryNotEmpty(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Authentication required: {0}")]
    AuthRequired(String),

    #[error("Authentication failed: {0}")]
    AuthFailed(String),

    #[error("Token expired")]
    TokenExpired,

    #[error("Rate limited: retry after {retry_after_secs:?}s")]
    RateLimited { retry_after_secs: Option<u64> },

    #[error("Provider API error ({provider}): {message}")]
    ProviderApi { provider: String, message: String },

    #[error("Quota exceeded: {0}")]
    QuotaExceeded(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Cache error: {0}")]
    Cache(String),

    #[error("Backend not found: {0}")]
    BackendNotFound(String),

    #[error("Offline and no cached version")]
    OfflineNoCache,

    #[error("Checksum mismatch")]
    ChecksumMismatch,

    #[error("Timeout")]
    Timeout,

    #[error("Cancelled")]
    Cancelled,

    #[error("{0}")]
    Other(String),
}

impl CfkError {
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            CfkError::Network(_)
                | CfkError::RateLimited { .. }
                | CfkError::Timeout
                | CfkError::TokenExpired
        )
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(
            self,
            CfkError::AuthRequired(_) | CfkError::AuthFailed(_) | CfkError::TokenExpired
        )
    }
}
