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
    pub day_open: Option<f64>,
    #[serde(rename = "vw")]
    pub vwap: f64,
    #[serde(rename = "o")]
    pub open: f64,
    #[serde(rename = "c")]
    pub close: f64,
    #[serde(rename = "h")]
    pub high: f64,
    #[serde(rename = "l")]
    pub low: f64,
    #[serde(rename = "a")]
    pub average: f64,
    #[serde(rename = "z", skip_serializing_if = "Option::is_none")]
    pub average_trade_size: Option<u32>,
    #[serde(rename = "s")]
    pub start_timestamp: u64,
    #[serde(rename = "e")]
    pub end_timestamp: u64,
}
