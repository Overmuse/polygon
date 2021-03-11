#[cfg(feature = "ws")]
use crate::ws::PolygonAction;
use thiserror::Error;

#[cfg(feature = "ws")]
use tokio_tungstenite::tungstenite;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "rest")]
    #[error("Missing environment variable: {0}")]
    MissingEnv(#[from] std::env::VarError),

    #[cfg(feature = "rest")]
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde error: {0}")]
    Serde(#[from] serde_json::Error),

    #[cfg(feature = "rest")]
    #[error("Invalid request. Received status {0}. Message: {1}")]
    ClientError(reqwest::StatusCode, String),

    #[cfg(feature = "rest")]
    #[error("Server error. Received status {0}. Message: {1}")]
    ServerError(reqwest::StatusCode, String),

    #[cfg(feature = "ws")]
    #[error("Tungstenite error: {0}")]
    Tungstenite(#[from] tungstenite::Error),

    #[cfg(feature = "ws")]
    #[error("WebSocket stream has been closed")]
    StreamClosed,

    #[cfg(feature = "ws")]
    #[error("Failed to connect: {0}")]
    ConnectionFailure(String),

    #[cfg(feature = "ws")]
    #[error("Failed to serialize message: {:?}", .0)]
    Serialize(PolygonAction),

    #[cfg(feature = "ws")]
    #[error("Failed to send message: {0}")]
    Sending(String),
}

pub type Result<T> = std::result::Result<T, Error>;
