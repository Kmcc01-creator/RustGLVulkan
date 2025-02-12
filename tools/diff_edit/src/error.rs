
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DiffError {
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("LLM API error: {0}")]
    LLMError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,

    #[error("Parse error: {0}")]
    ParseError(String),
}