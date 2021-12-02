use serde_json::Value;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NFTStorageError {
    #[error("{0}")]
    InvalidRequest(#[from] reqwest::Error),
    #[error("Unable to parse json, {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("{0}")]
    ApiError(Value),
    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),
}
