use super::{aggregates::*, quotes::*, trades::*};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Debug)]
pub struct PolygonAction {
    pub action: Cow<'static, str>,
    pub params: Cow<'static, str>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum PolygonStatus {
    Connected,
    Success,
    AuthSuccess,
    AuthFailed,
    MaxConnections,
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
    Trade(Trade),
    #[serde(rename = "Q")]
    Quote(Quote),
    #[serde(rename = "AM")]
    Minute(Aggregate),
    #[serde(rename = "A")]
    Second(Aggregate),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn serde_trade() {
        let json = r#"{"ev":"T","sym":"MSFT","x":4,"i":"12345","z":3,"p":114.125,"s":100,"c":[0,12],"t":1536036818784}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::Trade(Trade {
                symbol: "MSFT".into(),
                trade_id: "12345".into(),
                exchange_id: 4,
                price: dec!(114.125),
                size: 100,
                conditions: vec![TradeCondition::RegularSale, TradeCondition::FormT],
                timestamp: 1536036818784,
                tape: Tape::C
            })
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
            PolygonMessage::Trade(Trade {
                symbol: "MSFT".into(),
                trade_id: "12345".into(),
                exchange_id: 4,
                price: dec!(114.125),
                size: 100,
                conditions: vec![],
                timestamp: 1536036818784,
                tape: Tape::C
            })
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }

    #[test]
    fn can_parse_trade_without_size_field() {
        let json = r#"{"ev":"T","sym":"CBOE","x":19,"i":"52983525035591","z":1,"p":104.28,"c":[38],"t":1618963200252}"#;

        let deserialized: PolygonMessage = serde_json::from_str(json).unwrap();
        assert_eq!(
            deserialized,
            PolygonMessage::Trade(Trade {
                symbol: "CBOE".into(),
                trade_id: "52983525035591".into(),
                exchange_id: 19,
                price: dec!(104.28),
                size: 0,
                conditions: vec![TradeCondition::CorrectedConsolidatedClose],
                timestamp: 1618963200252,
                tape: Tape::A
            })
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
            PolygonMessage::Quote(Quote {
                symbol: "MSFT".into(),
                ask_quote: Some(AskQuote {
                    exchange_id: 7,
                    price: dec!(114.128),
                    size: 160
                }),
                bid_quote: Some(BidQuote {
                    exchange_id: 4,
                    price: dec!(114.125),
                    size: 100
                }),
                condition: Some(QuoteCondition::Regular),
                timestamp: 1536036818784,
            })
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
            PolygonMessage::Quote(Quote {
                symbol: "MSFT".into(),
                ask_quote: None,
                bid_quote: Some(BidQuote {
                    exchange_id: 4,
                    price: dec!(114.125),
                    size: 100
                }),
                condition: None,
                timestamp: 1536036818784,
            })
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
            PolygonMessage::Second(Aggregate {
                symbol: "SPCE".into(),
                volume: 200,
                accumulated_volume: 8642007,
                day_open: Some(dec!(25.66)),
                vwap: dec!(25.3981),
                open: dec!(25.39),
                close: dec!(25.39),
                high: dec!(25.39),
                low: dec!(25.39),
                average: dec!(25.3714),
                average_trade_size: Some(50),
                start_timestamp: 1610144868000,
                end_timestamp: 1610144869000,
            })
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
            PolygonMessage::Second(Aggregate {
                symbol: "SPCE".into(),
                volume: 200,
                accumulated_volume: 8642007,
                day_open: None,
                vwap: dec!(25.3981),
                open: dec!(25.39),
                close: dec!(25.39),
                high: dec!(25.39),
                low: dec!(25.39),
                average: dec!(25.3714),
                average_trade_size: None,
                start_timestamp: 1610144868000,
                end_timestamp: 1610144869000,
            })
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
            PolygonMessage::Minute(Aggregate {
                symbol: "GTE".into(),
                volume: 4110,
                accumulated_volume: 9470157,
                day_open: Some(dec!(0.4372)),
                vwap: dec!(0.4488),
                open: dec!(0.4488),
                close: dec!(0.4486),
                high: dec!(0.4489),
                low: dec!(0.4486),
                average: dec!(0.4352),
                average_trade_size: Some(685),
                start_timestamp: 1610144640000,
                end_timestamp: 1610144700000,
            })
        );
        let serialized = serde_json::to_string(&deserialized).unwrap();
        assert_eq!(serialized, json);
    }
}
