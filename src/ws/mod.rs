use crate::errors::{Error, Result};
use futures::{ready, SinkExt, Stream, StreamExt};
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};
use tracing::info;

pub mod types;
pub use types::*;

pub struct WebSocket {
    inner: WebSocketStream<MaybeTlsStream<TcpStream>>,
    buffer: VecDeque<PolygonMessage>,
}

impl Stream for WebSocket {
    type Item = Result<PolygonMessage>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        if !self.buffer.is_empty() {
            let message = self.buffer.pop_front().expect("Guaranteed to be non-empty");
            return Poll::Ready(Some(Ok(message)));
        }
        match ready!(Pin::new(&mut self.inner).poll_next(cx)) {
            Some(Ok(item)) => {
                match item {
                    Message::Text(txt) => {
                        let parsed: Result<Vec<PolygonMessage>> =
                            serde_json::from_str(&txt).map_err(Error::Serde);
                        match parsed {
                            Ok(messages) => {
                                let to_buffer = &messages[1..];
                                for msg in to_buffer {
                                    self.buffer.push_back(msg.clone());
                                }
                                Poll::Ready(Some(Ok(messages[0].clone())))
                            }
                            Err(e) => Poll::Ready(Some(Err(e))),
                        }
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
            url: "wss://socket.polygon.io/stocks".to_string(),
            auth_token,
            events,
            assets,
        }
    }

    pub async fn connect(self) -> Result<WebSocket> {
        let (client, _) = connect_async(&self.url).await?;
        let mut ws = WebSocket {
            inner: client,
            buffer: VecDeque::new(),
        };
        let parsed = ws.next().await.ok_or(Error::StreamClosed)??;
        if let PolygonMessage::Status { status, message } = parsed {
            if let PolygonStatus::Connected = status {
                info!("Connected successfully");
            } else {
                return Err(Error::ConnectionFailure(message));
            }
        }
        let auth_message = PolygonAction {
            action: "auth".to_string(),
            params: self.auth_token.clone(),
        };
        ws.send_message(
            &serde_json::to_string(&auth_message).map_err(|_| Error::Serialize(auth_message))?,
        )
        .await?;
        let parsed = ws.next().await.ok_or(Error::StreamClosed)??;
        if let PolygonMessage::Status { status, message } = parsed {
            if let PolygonStatus::AuthSuccess = status {
                info!("Authorized successfully");
            } else {
                return Err(Error::ConnectionFailure(message));
            }
        }
        ws.subscribe(self.events, self.assets).await?;
        Ok(ws)
    }
}
