use super::conditions::QuoteCondition;
use crate::domain::tape::Tape;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    #[serde(rename = "t")]
    pub timestamp: u64,
    #[serde(rename = "y")]
    pub participant_timestamp: u64,
    #[serde(rename = "trf_timestamp", skip_serializing_if = "Option::is_none")]
    pub f: Option<u64>,
    #[serde(rename = "q")]
    pub sequence_number: u32,
    #[serde(rename = "c", skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<QuoteCondition>,
    #[serde(rename = "i", skip_serializing_if = "Option::is_none")]
    pub indicators: Option<Vec<u32>>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub bid_quote: Option<BidQuote>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ask_quote: Option<AskQuote>,
    #[serde(rename = "z")]
    pub tape: Tape,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BidQuote {
    #[serde(rename = "x")]
    pub exchange_id: u8,
    #[serde(rename = "p")]
    pub price: Decimal,
    #[serde(rename = "s")]
    pub size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AskQuote {
    #[serde(rename = "X")]
    pub exchange_id: u8,
    #[serde(rename = "P")]
    pub price: Decimal,
    #[serde(rename = "S")]
    pub size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteWrapper {
    pub ticker: String,
    pub results_count: u32,
    pub db_latency: u32,
    pub success: bool,
    pub results: Vec<Quote>,
}
