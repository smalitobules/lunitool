use thiserror::Error;

/// Custom error type for lunitool
#[derive(Error, Debug)]
pub enum LunitoolError {
    /// Input/output error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// System command error
    #[error("Command error: {0}")]
    Command(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Language error
    #[error("Language error: {0}")]
    Language(String),

    /// UI error
    #[error("UI error: {0}")]
    Ui(String),

    /// Generic error
    #[error("{0}")]
    Generic(String),
}

/// Result type for lunitool
pub type LunitoolResult<T> = Result<T, LunitoolError>;