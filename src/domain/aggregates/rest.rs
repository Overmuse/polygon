use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aggregate {
    #[serde(rename = "o")]
    pub open: Decimal,
    #[serde(rename = "h")]
    pub high: Decimal,
    #[serde(rename = "l")]
    pub low: Decimal,
    #[serde(rename = "c")]
    pub close: Decimal,
    #[serde(rename = "v")]
    pub volume: Decimal,
    #[serde(rename = "vw")]
    pub vwap: Option<Decimal>,
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "n")]
    pub number_transactions: Option<u32>,
}
