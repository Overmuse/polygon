use crate::errors::{Error, Result};
use futures::{ready, Sink, SinkExt, Stream, StreamExt};
use std::collections::VecDeque;
use std::pin::Pin;
use std::task::{Context, Poll};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tracing::info;

pub mod types;
pub use types::*;

type TungsteniteResult = std::result::Result<Message, tokio_tungstenite::tungstenite::Error>;

pub struct WebSocket<T> {
    inner: T,
    buffer: VecDeque<PolygonMessage>,
}

impl<T: Stream<Item = TungsteniteResult> + Unpin> Stream for WebSocket<T> {
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
impl<T: Sink<Message> + Unpin, S: Into<String>> Sink<S> for WebSocket<T> {
    type Error = T::Error;

    fn poll_ready(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: S) -> std::result::Result<(), Self::Error> {
        let inner_item = Message::Text(item.into());
        Pin::new(&mut self.inner).start_send(inner_item)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<std::result::Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_close(cx)
    }
}

impl<T: Sink<Message> + Unpin> WebSocket<T> {
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
        let subscription_str = serde_json::to_string(&subscription_message)
            .map_err(|_| Error::Serialize(subscription_message))?;

        self.send(&subscription_str)
            .await
            .map_err(|_| Error::Sending(subscription_str))?;
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
    pub fn new(url: String, auth_token: String, events: Vec<String>, assets: Vec<String>) -> Self {
        Self {
            url,
            auth_token,
            events,
            assets,
        }
    }

    pub async fn connect(
        self,
    ) -> Result<WebSocket<impl Stream<Item = TungsteniteResult> + Sink<Message> + Unpin>> {
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
        ws.send(&serde_json::to_string(&auth_message).map_err(|_| Error::Serialize(auth_message))?)
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

#[cfg(test)]
mod test {
    use super::{Connection, PolygonMessage, PolygonStatus};
    use futures::{SinkExt, StreamExt};
    use tokio::{
        io::{AsyncRead, AsyncWrite},
        net::TcpListener,
    };
    use tokio_tungstenite::tungstenite::Message;
    use tokio_tungstenite::{accept_async, WebSocketStream};

    async fn run_connection<S>(connection: WebSocketStream<S>)
    where
        S: AsyncRead + AsyncWrite + Unpin,
    {
        let mut connection = connection;
        let connection_message = Message::Text(
            r#"[{"ev":"status","status":"connected","message":"Connected Successfully"}]"#.into(),
        );
        connection
            .send(connection_message)
            .await
            .expect("Failed to send connection_message");
        let auth_request = connection.next().await.unwrap().unwrap();
        assert_eq!(
            auth_request,
            Message::Text(r#"{"action":"auth","params":"test"}"#.into())
        );
        let auth_response = Message::Text(
            r#"[{"ev":"status","status":"auth_success","message":"authenticated"}]"#.into(),
        );
        connection
            .send(auth_response)
            .await
            .expect("Failed to send auth_response");
        let subscription_request = connection.next().await.unwrap().unwrap();
        assert_eq!(
            subscription_request,
            Message::Text(r#"{"action":"subscribe","params":"T.AAPL,T.TSLA,Q.AAPL,Q.TSLA,A.AAPL,A.TSLA,AM.AAPL,AM.TSLA"}"#.into())
        );
        let subscription_response = r#"[
            {"ev":"status","status":"success","message":"subscribed to: T.AAPL"},
            {"ev":"status","status":"success","message":"subscribed to: T.TSLA"},
            {"ev":"status","status":"success","message":"subscribed to: Q.AAPL"},
            {"ev":"status","status":"success","message":"subscribed to: Q.TSLA"},
            {"ev":"status","status":"success","message":"subscribed to: A.AAPL"},
            {"ev":"status","status":"success","message":"subscribed to: A.TSLA"},
            {"ev":"status","status":"success","message":"subscribed to: AM.AAPL"},
            {"ev":"status","status":"success","message":"subscribed to: AM.TSLA"}
        ]"#;
        connection
            .send(Message::Text(subscription_response.into()))
            .await
            .expect("Failed to send subscription response");
    }

    #[tokio::test]
    async fn test_connection() {
        let (con_tx, con_rx) = futures_channel::oneshot::channel();
        tokio::spawn(async move {
            let listener = TcpListener::bind("127.0.0.1:12345").await.unwrap();
            // Send message when server is ready to start the test
            con_tx.send(()).unwrap();
            let (connection, _) = listener.accept().await.expect("No connections to accept");
            let stream = accept_async(connection).await;
            let stream = stream.expect("Failed to handshake with connection");
            run_connection(stream).await;
        });

        con_rx.await.expect("Server not ready");
        let connection = Connection::new(
            "ws://localhost:12345".into(),
            "test".into(),
            vec!["T".into(), "Q".into(), "A".into(), "AM".into()],
            vec!["AAPL".into(), "TSLA".into()],
        );

        let mut ws = connection.connect().await.unwrap();
        let subscription_response = ws.next().await.unwrap().unwrap();
        // only receive one message even though multiple were sent at once
        assert_eq!(
            subscription_response,
            PolygonMessage::Status {
                status: PolygonStatus::Success,
                message: "subscribed to: T.AAPL".into()
            }
        );
        // The remaining messages are still in the buffer
        assert_eq!(ws.buffer.len(), 7);

        let subscription_response = ws.next().await.unwrap().unwrap();
        // this time the message gets pulled from the buffer
        assert_eq!(
            subscription_response,
            PolygonMessage::Status {
                status: PolygonStatus::Success,
                message: "subscribed to: T.TSLA".into()
            }
        );
        // buffer has decreased
        assert_eq!(ws.buffer.len(), 6);
    }
}
