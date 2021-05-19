use super::conditions::QuoteCondition;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Quote {
    #[serde(rename = "sym")]
    pub symbol: String,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub bid_quote: Option<BidQuote>,
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub ask_quote: Option<AskQuote>,
    #[serde(rename = "c", skip_serializing_if = "Option::is_none")]
    pub condition: Option<QuoteCondition>,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BidQuote {
    #[serde(rename = "bx")]
    pub exchange_id: u8,
    #[serde(rename = "bp")]
    pub price: Decimal,
    #[serde(rename = "bs")]
    pub size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AskQuote {
    #[serde(rename = "ax")]
    pub exchange_id: u8,
    #[serde(rename = "ap")]
    pub price: Decimal,
    #[serde(rename = "as")]
    pub size: u32,
}
