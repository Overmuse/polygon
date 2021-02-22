use crate::errors::{Error, Result};
use futures::{ready, SinkExt, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

pub mod types;
pub use types::*;

#[derive(Serialize, Debug)]
pub struct PolygonAction {
    action: String,
    params: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum PolygonStatus {
    Connected,
    Success,
    AuthSuccess,
    AuthFailed,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
struct PolygonResponse {
    ev: String,
    status: PolygonStatus,
    message: String,
}

pub struct WebSocket {
    inner: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl Stream for WebSocket {
    type Item = Result<Vec<PolygonMessage>>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match ready!(Pin::new(&mut self.inner).poll_next(cx)) {
            Some(Ok(item)) => {
                match item {
                    Message::Text(txt) => {
                        let parsed: Result<Vec<PolygonMessage>> =
                            serde_json::from_str(&txt).map_err(|_| Error::Parse(txt));
                        Poll::Ready(Some(parsed))
                    }
                    _ => {
                        // Non Text message received, immediately schedule re-poll
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
            }
            Some(Err(e)) => Poll::Ready(Some(Err(Error::Tungstenite(e)))),
            None => Poll::Ready(None),
        }
    }
}

impl WebSocket {
    async fn send_message(&mut self, msg: &str) -> Result<()> {
        self.inner.send(Message::Text(msg.to_string())).await?;
        Ok(())
    }

    async fn read_message(&mut self) -> Result<Vec<PolygonResponse>> {
        let resp = self.inner.next().await.ok_or(Error::StreamClosed)??;
        let txt = resp.to_text()?;
        let parsed: Vec<PolygonResponse> =
            serde_json::from_str(txt).map_err(|_| Error::Parse(txt.to_string()))?;
        Ok(parsed)
    }

    pub async fn subscribe(&mut self, events: Vec<String>, assets: Vec<String>) -> Result<()> {
        let subscriptions: Vec<_> = events
            .iter()
            .flat_map(|x| std::iter::repeat(x).zip(assets.iter()))
            .map(|(x, y)| format!("{}.{}", x, y))
            .collect();
        let subscription_message = PolygonAction {
            action: "subscribe".to_string(),
            params: subscriptions.join(","),
        };

        self.send_message(
            &serde_json::to_string(&subscription_message)
                .map_err(|_| Error::Serialize(subscription_message))?,
        )
        .await?;
        Ok(())
    }
}

pub struct Connection {
    url: String,
    auth_token: String,
    events: Vec<String>,
    assets: Vec<String>,
}

impl Connection {
    pub fn new(auth_token: String, events: Vec<String>, assets: Vec<String>) -> Self {
        Self {
            url: "wss://alpaca.socket.polygon.io/stocks".to_string(),
            auth_token,
            events,
            assets,
        }
    }

    pub async fn connect(self) -> Result<WebSocket> {
        let auth_message = PolygonAction {
            action: "auth".to_string(),
            params: self.auth_token.clone(),
        };
        let (client, _) = connect_async(&self.url).await?;
        let mut ws = WebSocket { inner: client };
        let parsed = ws.read_message().await?;
        if let PolygonStatus::Connected = parsed[0].status {
        } else {
            return Err(Error::ConnectionFailure(parsed[0].message.clone()));
        }
        ws.send_message(
            &serde_json::to_string(&auth_message).map_err(|_| Error::Serialize(auth_message))?,
        )
        .await?;
        let parsed = ws.read_message().await?;
        if let PolygonStatus::AuthSuccess = parsed[0].status {
        } else {
            return Err(Error::ConnectionFailure(parsed[0].message.clone()));
        }
        ws.subscribe(self.events, self.assets).await?;
        Ok(ws)
    }
}

#[cfg(test)]
mod test {
    use super::*;
}
