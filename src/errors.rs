use crate::ws::{PolygonAction, PolygonResponse};
use thiserror::Error;

#[cfg(feature = "ws")]
use tokio_tungstenite::tungstenite;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "ws")]
    #[error("Tungstenite error: {0}")]
    Tungstenite(#[from] tungstenite::Error),
    #[cfg(feature = "ws")]
    #[error("WebSocket stream has been closed")]
    StreamClosed,
    #[cfg(feature = "ws")]
    #[error("Failed to connect: {:?}", .0)]
    ConnectionFailure(PolygonResponse),
    #[error("Failed to parse message: {0}")]
    Parse(String),
    #[error("Failed to serialize message: {:?}", .0)]
    Serialize(PolygonAction),
}

pub type Result<T> = std::result::Result<T, Error>;
