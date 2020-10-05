use crate::errors::{Error, Result};
use serde::{Serialize, Deserialize};
use futures_util::{stream::StreamExt, sink::SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tokio_tungstenite::tungstenite::Message;

#[derive(Serialize)]
struct PolygonAction {
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

pub struct Connection {
    client: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
    url: String,
    auth_token: String,
    events: Vec<String>,
    assets: Vec<String>,
}

impl Connection {
    pub fn new(auth_token: String, events: Vec<String>, assets: Vec<String>) -> Self {
        Self {
            client: None,
            url: "wss://alpaca.socket.polygon.io/stocks".to_string(),
            auth_token,
            events,
            assets,
        }
    }

    async fn send_message(&mut self, msg: &str) -> Result<()> {
        if let Some(client) = &mut self.client {
            client.send(Message::Text(msg.to_string())).await?;
            Ok(())
        } else {
            Err(Error::UninitializedClient)
        }
    }

    async fn read_message(&mut self) -> Result<Vec<PolygonResponse>> {
        if let Some(client) = &mut self.client {
            let resp = client.next().await.ok_or_else(|| Error::StreamClosed)??;
            let parsed: Vec<PolygonResponse> = serde_json::from_str(resp.to_text()?)?;
            Ok(parsed)
        } else {
            Err(Error::UninitializedClient)
        }
    }

    pub async fn connect(&mut self) -> Result<()> {
        let auth_message = PolygonAction {
            action: "auth".to_string(),
            params: self.auth_token.clone(),
        };
        let (client, _) = connect_async(&self.url).await?;
        self.client = Some(client);
        let parsed = self.read_message().await?;
        if let PolygonStatus::Connected = parsed[0].status {
        } else {
            return Err(Error::ConnectionFailure(parsed[0].message.clone()))
        }
        println!("{:?}", parsed);
        self.send_message(&serde_json::to_string(&auth_message)?).await?;
        let parsed = self.read_message().await?;
        if let PolygonStatus::AuthSuccess = parsed[0].status {
        } else {
            return Err(Error::ConnectionFailure(parsed[0].message.clone()))
        }
        println!("{:?}", parsed);
        Ok(())
    }

    pub async fn subscribe(&mut self) -> Result<()> {
        let subscriptions: Vec<_> = self
            .events
            .iter()
            .flat_map(|x| std::iter::repeat(x).zip(self.assets.iter()))
            .map(|(x, y)| format!("{}.{}", x, y))
            .collect();
        let subscription_message = PolygonAction {
            action: "subscribe".to_string(),
            params: subscriptions.join(","),
        };

        self.send_message(&serde_json::to_string(&subscription_message)?).await?;
        let parsed = self.read_message().await?;
        println!("{:?}", parsed);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn connect() {
        let mut c = Connection::new("AKJJIS846R9E4H9NQLHJ".into(), vec!["T".to_string()], vec!["AAPL".to_string()]);
        c.connect().await.unwrap();
        c.subscribe().await.unwrap();
    }
}
