use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Debug)]
pub struct PolygonAction {
    pub action: String,
    pub params: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PolygonStatus {
    Connected,
    Success,
    AuthSuccess,
    AuthFailed,
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

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Tape {
    A = 1,
    B = 2,
    C = 3,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BidQuote {
    #[serde(rename = "bx")]
    exchange_id: u8,
    #[serde(rename = "bp")]
    price: f64,
    #[serde(rename = "bs")]
    size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AskQuote {
    #[serde(rename = "ax")]
    exchange_id: u8,
    #[serde(rename = "ap")]
    price: f64,
    #[serde(rename = "as")]
    size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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
        #[serde(rename = "x")]
        exchange_id: u8,
        #[serde(rename = "i")]
        trade_id: String,
        #[serde(rename = "z")]
        tape: Tape,
        #[serde(rename = "p")]
        price: f64,
        #[serde(rename = "s")]
        size: u32,
        #[serde(
            rename = "c",
            default = "default_conditions",
            skip_serializing_if = "Vec::is_empty"
        )]
        conditions: Vec<u8>,
        #[serde(rename = "t")]
        timestamp: u64,
    },
    #[serde(rename = "Q")]
    Quote {
        #[serde(rename = "sym")]
        symbol: String,
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        bid_quote: Option<BidQuote>,
        #[serde(flatten, skip_serializing_if = "Option::is_none")]
        ask_quote: Option<AskQuote>,
        #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
        average_trade_size: Option<u32>,
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
        #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
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
        #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
        average_trade_size: Option<u32>,
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
        let json = r#"{"ev":"T","sym":"MSFT","x":4,"i":"12345","z":3,"p":114.125,"s":100,"c":[0,12],"t":1536036818784}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::Trade {
                symbol: "MSFT".into(),
                trade_id: "12345".into(),
                exchange_id: 4,
                price: 114.125,
                size: 100,
                conditions: vec![0, 12],
                timestamp: 1536036818784,
                tape: Tape::C
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn can_parse_trade_without_conditions_field() {
        let json = r#"{"ev":"T","sym":"MSFT","x":4,"i":"12345","z":3,"p":114.125,"s":100,"t":1536036818784}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::Trade {
                symbol: "MSFT".into(),
                trade_id: "12345".into(),
                exchange_id: 4,
                price: 114.125,
                size: 100,
                conditions: vec![],
                timestamp: 1536036818784,
                tape: Tape::C
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn serde_quote() {
        let json = r#"{"ev":"Q","sym":"MSFT","bx":4,"bp":114.125,"bs":100,"ax":7,"ap":114.128,"as":160,"c":0,"t":1536036818784}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::Quote {
                symbol: "MSFT".into(),
                ask_quote: Some(AskQuote {
                    exchange_id: 7,
                    price: 114.128,
                    size: 160
                }),
                bid_quote: Some(BidQuote {
                    exchange_id: 4,
                    price: 114.125,
                    size: 100
                }),
                condition: Some(0),
                timestamp: 1536036818784,
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn can_parse_quote_with_only_one_exchange_and_no_conditions() {
        let json = r#"{"ev":"Q","sym":"MSFT","bx":4,"bp":114.125,"bs":100,"t":1536036818784}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::Quote {
                symbol: "MSFT".into(),
                ask_quote: None,
                bid_quote: Some(BidQuote {
                    exchange_id: 4,
                    price: 114.125,
                    size: 100
                }),
                condition: None,
                timestamp: 1536036818784,
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn serde_second_aggregate() {
        let json = r#"{"ev":"A","sym":"SPCE","v":200,"av":8642007,"op":25.66,"vw":25.3981,"o":25.39,"c":25.39,"h":25.39,"l":25.39,"a":25.3714,"z":50,"s":1610144868000,"e":1610144869000}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::SecondAggregate {
                symbol: "SPCE".into(),
                volume: 200,
                accumulated_volume: 8642007,
                day_open: Some(25.66),
                vwap: 25.3981,
                open: 25.39,
                close: 25.39,
                high: 25.39,
                low: 25.39,
                average: 25.3714,
                average_trade_size: Some(50),
                start_timestamp: 1610144868000,
                end_timestamp: 1610144869000,
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn can_parse_aggregate_with_missing_fields() {
        let json = r#"{"ev":"A","sym":"SPCE","v":200,"av":8642007,"vw":25.3981,"o":25.39,"c":25.39,"h":25.39,"l":25.39,"a":25.3714,"s":1610144868000,"e":1610144869000}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::SecondAggregate {
                symbol: "SPCE".into(),
                volume: 200,
                accumulated_volume: 8642007,
                day_open: None,
                vwap: 25.3981,
                open: 25.39,
                close: 25.39,
                high: 25.39,
                low: 25.39,
                average: 25.3714,
                average_trade_size: None,
                start_timestamp: 1610144868000,
                end_timestamp: 1610144869000,
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn serde_minute_aggregate() {
        let json = r#"{"ev":"AM","sym":"GTE","v":4110,"av":9470157,"op":0.4372,"vw":0.4488,"o":0.4488,"c":0.4486,"h":0.4489,"l":0.4486,"a":0.4352,"z":685,"s":1610144640000,"e":1610144700000}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::MinuteAggregate {
                symbol: "GTE".into(),
                volume: 4110,
                accumulated_volume: 9470157,
                day_open: Some(0.4372),
                vwap: 0.4488,
                open: 0.4488,
                close: 0.4486,
                high: 0.4489,
                low: 0.4486,
                average: 0.4352,
                average_trade_size: Some(685),
                start_timestamp: 1610144640000,
                end_timestamp: 1610144700000,
            }
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }
}
