use thiserror::Error;

#[cfg(feature = "ws")]
use tokio_tungstenite::tungstenite;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Client has not yet been initialized.")]
    UninitializedClient,
    #[error("WebSocket stream has been closed")]
    StreamClosed,
    #[error("Failed to connect: {0}")]
    ConnectionFailure(String),
    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),
    #[cfg(feature = "ws")]
    #[error("Tungstenite error: {0}")]
    Tungstenite(#[from] tungstenite::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
