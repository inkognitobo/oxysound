use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadPlaylistError {
    #[error("failed to read/create file")]
    IOError(#[from] std::io::Error),
    #[error("failed to deserialize json")]
    DeserializeError(#[from] serde_json::Error),
}
