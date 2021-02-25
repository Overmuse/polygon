use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Debug)]
pub struct PolygonAction {
    pub action: String,
    pub params: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PolygonStatus {
    Connected,
    Success,
    AuthSuccess,
    AuthFailed,
    MaxConnections,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub struct PolygonResponse {
    ev: String,
    pub status: PolygonStatus,
    pub message: String,
}

fn default_conditions() -> Vec<u8> {
    Vec::new()
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone)]
#[repr(u8)]
pub enum Tape {
    A = 1,
    B = 2,
    C = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BidQuote {
    #[serde(rename = "bx")]
    exchange_id: u8,
    #[serde(rename = "bp")]
    price: f64,
    #[serde(rename = "bs")]
    size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AskQuote {
    #[serde(rename = "ax")]
    exchange_id: u8,
    #[serde(rename = "ap")]
    price: f64,
    #[serde(rename = "as")]
    size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "ev")]
pub enum PolygonMessage {
    #[serde(rename = "status")]
    Status {
        status: PolygonStatus,
        message: String,
    },
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
    fn serde_trade() {
        let json = r#"{
	    "ev": "T",
	    "sym": "MSFT",
	    "x": 4,
	    "i": "12345",
	    "z": 3,
	    "p": 114.125,
	    "s": 100,
	    "c": [
	    	0,
	     	12
	    ],
	    "t": 1536036818784
	}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
        if let PolygonMessage::Trade { .. } = deserialized {
        } else {
            panic!("Not a Trade")
        }
    }

    #[test]
    fn can_parse_trade_without_conditions_field() {
        let json = r#"{
	    "ev": "T",
	    "sym": "MSFT",
	    "x": 4,
	    "i": "12345",
	    "z": 3,
	    "p": 114.125,
	    "s": 100,
	    "t": 1536036818784
	}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
        if let PolygonMessage::Trade { .. } = deserialized {
        } else {
            panic!("Not a Trade")
        }
    }

    #[test]
    fn serde_quote() {
        let json = r#"{
	    "ev": "Q",
	    "sym": "MSFT",
	    "bx": 4,
            "bp": 114.125,
            "bs": 100,
            "ax": 7,
            "ap": 114.128,
            "as": 160,
	    "c":  0,
	    "t": 1536036818784
	}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
        if let PolygonMessage::Quote { .. } = deserialized {
        } else {
            panic!("Not a Quote")
        }
    }

    #[test]
    fn can_parse_quote_with_only_one_exchange_and_no_conditions() {
        let json = r#"{
	    "ev": "Q",
	    "sym": "MSFT",
	    "bx": 4,
            "bp": 114.125,
            "bs": 100,
	    "t": 1536036818784
	}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
        if let PolygonMessage::Quote { .. } = deserialized {
        } else {
            panic!("Not a Quote")
        }
    }

    #[test]
    fn serde_second_aggregate() {
        let json = r#"{
            "ev": "A",
            "sym": "SPCE",
            "v": 200,
            "av": 8642007,
            "op": 25.66,
            "vw": 25.3981,
            "o": 25.39,
            "c": 25.39,
            "h": 25.39,
            "l": 25.39,
            "a": 25.3714,
            "z": 50,
            "s": 1610144868000,
            "e": 1610144869000
        }"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
        if let PolygonMessage::SecondAggregate { .. } = deserialized {
        } else {
            panic!("Not a SecondAggregate")
        }
    }

    #[test]
    fn can_parse_aggregate_without_day_open_field() {
        let json = r#"{
            "ev": "A",
            "sym": "SPCE",
            "v": 200,
            "av": 9470157,
            "vw": 25.3981,
            "o": 25.39,
            "c": 25.39,
            "h": 25.39,
            "l": 25.39,
            "a": 25.3714,
            "z": 50,
            "s": 1610144868000,
            "e": 1610144869000
        }"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
        if let PolygonMessage::SecondAggregate { .. } = deserialized {
        } else {
            panic!("Not a SecondAggregate")
        }
    }

    #[test]
    fn serde_minute_aggregate() {
        let json = r#"{
            "ev": "AM",
            "sym": "GTE",
            "v": 4110,
            "av": 9470157,
            "op": 0.4372,
            "vw": 0.4488,
            "o": 0.4488,
            "c": 0.4486,
            "h": 0.4489,
            "l": 0.4486,
            "a": 0.4352,
            "z": 685,
            "s": 1610144640000,
            "e": 1610144700000
        }"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        let _serialized = serde_json::to_string(&deserialized).unwrap();
        if let PolygonMessage::MinuteAggregate { .. } = deserialized {
        } else {
            panic!("Not a MinuteAggregate")
        }
    }
}
