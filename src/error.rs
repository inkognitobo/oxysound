//! Main crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Required values missing from config: {0}\nShould be configured here: {1}")]
    MissingConfig(String, String),

    #[error("Couldn't convert `PathBuf` to `String`: {0}")]
    StringFromPathBuf(String),

    #[error("Response didn't yield enough items (expected: {0}, found: {1}")]
    NotEnoughResponseItems(u8, u8),

    #[error("Request failed {0}")]
    Request(#[from] reqwest::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("Failed to serialize/deserialize JSON")]
    Json(#[from] serde_json::Error),

    #[error("Failed loading config")]
    Config(#[from] confy::ConfyError),
}
