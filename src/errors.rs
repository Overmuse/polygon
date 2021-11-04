#[cfg(feature = "ws")]
use crate::ws::PolygonAction;
use thiserror::Error;

#[cfg(feature = "ws")]
use tokio_tungstenite::tungstenite;

#[derive(Debug, Error)]
pub enum Error {
    #[cfg(feature = "rest")]
    #[error("Missing environment variable: {variable}")]
    MissingEnv {
        #[source]
        source: std::env::VarError,
        variable: String,
    },

    #[error("Serde error: {error}\nMsg: {msg}")]
    Serde {
        error: serde_json::Error,
        msg: String,
    },

    #[cfg(feature = "rest")]
    #[error(transparent)]
    Rest(rest_client::Error),

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
