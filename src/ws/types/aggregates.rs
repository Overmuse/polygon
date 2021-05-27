use chrono::{serde::ts_milliseconds, DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Aggregate {
    #[serde(rename = "sym")]
    pub symbol: String,
    #[serde(rename = "v")]
    pub volume: u32,
    #[serde(rename = "av")]
    pub accumulated_volume: u32,
    #[serde(rename = "op", skip_serializing_if = "Option::is_none")]
    pub day_open: Option<Decimal>,
    #[serde(rename = "vw")]
    pub vwap: Decimal,
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "a")]
    pub average: Decimal,
    #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
    pub average_trade_size: Option<u32>,
    #[serde(rename = "s", with = "ts_milliseconds")]
    pub start_timestamp: DateTime<Utc>,
    #[serde(rename = "e", with = "ts_milliseconds")]
    pub end_timestamp: DateTime<Utc>,
}
