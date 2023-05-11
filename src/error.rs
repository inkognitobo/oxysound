//! Main crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Request failed {0}")]
    Request(#[from] reqwest::Error),

    #[error("Response didn't yield enough items (expected: {0}, found: {1}")]
    NotEnoughResponseItems(u8, u8),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to serialize/deserialize")]
    Serialize(#[from] serde_json::Error),

    #[error("Failed loading var from env")]
    Var(#[from] std::env::VarError),
}
