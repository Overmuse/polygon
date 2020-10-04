use serde_json;
#[cfg(feature = "ws")]
use tungstenite;


#[derive(Debug)]
pub enum Error {
    Serde(serde_json::Error),
    #[cfg(feature = "ws")]
    Tungstenite(tungstenite::Error),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

#[cfg(feature = "ws")]
impl From<tungstenite::Error> for Error {
    fn from(e: tungstenite::Error) -> Self {
        Self::Tungstenite(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
