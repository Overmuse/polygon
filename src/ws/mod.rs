use crate::errors::Result;
use serde::{Serialize, Deserialize};
use tungstenite::Message;
use tungstenite::client::{connect, AutoStream};
use tungstenite::protocol::WebSocket;

#[derive(Serialize)]
struct PolygonAction {
    action: String,
    params: String,
}

#[derive(Deserialize, Debug)]
struct PolygonResponse {
    ev: String,
    status: String,
    message: String,
}

pub struct Connection {
    client: Option<WebSocket<AutoStream>>,
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

    fn send_message(&mut self, msg: &str) -> Result<()> {
        if let Some(client) = &mut self.client {
            client.write_message(Message::Text(msg.to_string()))?;
        } else {
            //error!("Tried to send a message before connecting to websocket!")
        }
        Ok(())
    }

    pub fn connect(&mut self) -> Result<()> {
        let auth_message = PolygonAction {
            action: "auth".to_string(),
            params: self.auth_token.clone(),
        };
        let (mut client, _) = connect(&self.url)?;
        let resp = client.read_message()?;
        let parsed: Vec<PolygonResponse> = serde_json::from_str(resp.to_text()?)?;
        println!("{:?}", parsed);
        self.client = Some(client);
        self.send_message(&serde_json::to_string(&auth_message)?)?;
        let resp = self.client.as_mut().unwrap().read_message()?;
        let parsed: Vec<PolygonResponse> = serde_json::from_str(resp.to_text()?)?;
        println!("{:?}", parsed);
        Ok(())
    }

    pub fn subscribe(&mut self) -> Result<()> {
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

        self.send_message(&serde_json::to_string(&subscription_message)?)?;
        let resp = self.client.as_mut().unwrap().read_message()?;
        let parsed: Vec<PolygonResponse> = serde_json::from_str(resp.to_text()?)?;
        println!("{:?}", parsed);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn connect() {
        let mut c = Connection::new("AKJJIS846R9E4H9NQLHJ".into(), vec!["T".to_string()], vec!["AAPL".to_string()]);
        c.connect().unwrap();
        c.subscribe().unwrap();
    }
}
