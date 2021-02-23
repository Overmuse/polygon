use serde::{Deserialize, Serialize};
use serde_repr::*;

fn default_conditions() -> Vec<u8> {
    Vec::new()
}

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
pub enum Tape {
    A = 1,
    B = 2,
    C = 3,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BidQuote {
    #[serde(rename = "bx")]
    exchange_id: u8,
    #[serde(rename = "bp")]
    price: f64,
    #[serde(rename = "bs")]
    size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AskQuote {
    #[serde(rename = "ax")]
    exchange_id: u8,
    #[serde(rename = "ap")]
    price: f64,
    #[serde(rename = "as")]
    size: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "ev")]
pub enum PolygonMessage {
    #[serde(rename = "status")]
    Status { status: String, message: String },
    #[serde(rename = "T")]
    Trade {
        #[serde(rename = "sym")]
        symbol: String,
        #[serde(rename = "i")]
        trade_id: String,
        #[serde(rename = "x")]
        exchange_id: u8,
        #[serde(rename = "p")]
        price: f64,
        #[serde(rename = "s")]
        size: u32,
        #[serde(rename = "c", default = "default_conditions")]
        conditions: Vec<u8>,
        #[serde(rename = "t")]
        timestamp: u64,
        #[serde(rename = "z")]
        tape: Tape,
    },
    #[serde(rename = "Q")]
    Quote {
        #[serde(rename = "sym")]
        symbol: String,
        #[serde(flatten)]
        bid_quote: Option<BidQuote>,
        #[serde(flatten)]
        ask_quote: Option<AskQuote>,
        #[serde(rename = "c")]
        condition: Option<u8>,
        #[serde(rename = "t")]
        timestamp: u64,
    },
    #[serde(rename = "AM")]
    MinuteAggregate {
        #[serde(rename = "sym")]
        symbol: String,
        #[serde(rename = "v")]
        volume: u32,
        #[serde(rename = "av")]
        accumulated_volume: u32,
        #[serde(rename = "op")]
        day_open: Option<f64>,
        #[serde(rename = "vw")]
        vwap: f64,
        #[serde(rename = "o")]
        open: f64,
        #[serde(rename = "c")]
        close: f64,
        #[serde(rename = "h")]
        high: f64,
        #[serde(rename = "l")]
        low: f64,
        #[serde(rename = "a")]
        average: f64,
        #[serde(rename = "s")]
        start_timestamp: u64,
        #[serde(rename = "e")]
        end_timestamp: u64,
    },
    #[serde(rename = "A")]
    SecondAggregate {
        #[serde(rename = "sym")]
        symbol: String,
        #[serde(rename = "v")]
        volume: u32,
        #[serde(rename = "av")]
        accumulated_volume: u32,
        #[serde(rename = "op")]
        day_open: Option<f64>,
        #[serde(rename = "vw")]
        vwap: f64,
        #[serde(rename = "o")]
        open: f64,
        #[serde(rename = "c")]
        close: f64,
        #[serde(rename = "h")]
        high: f64,
        #[serde(rename = "l")]
        low: f64,
        #[serde(rename = "a")]
        average: f64,
        #[serde(rename = "s")]
        start_timestamp: u64,
        #[serde(rename = "e")]
        end_timestamp: u64,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trade() {
        let t = PolygonMessage::Trade {
            symbol: "AAPL".to_string(),
            trade_id: "53165".to_string(),
            exchange_id: 4,
            price: 321.85,
            size: 12399,
            conditions: vec![13],
            timestamp: 1591041629938,
            tape: Tape::C,
        };
        assert_eq!(
            format!("{}", serde_json::to_string(&t).unwrap()),
            r#"{"ev":"T","sym":"AAPL","i":"53165","x":4,"p":321.85,"s":12399,"c":[13],"t":1591041629938,"z":3}"#
        );
    }
}
