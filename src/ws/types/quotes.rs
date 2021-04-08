use serde::{Deserialize, Serialize};
use serde_repr::*;

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
    pub price: f64,
    #[serde(rename = "bs")]
    pub size: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AskQuote {
    #[serde(rename = "ax")]
    pub exchange_id: u8,
    #[serde(rename = "ap")]
    pub price: f64,
    #[serde(rename = "as")]
    pub size: u32,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Hash, Eq)]
#[repr(u8)]
pub enum QuoteCondition {
    Regular = 0,
    RegularTwoSidedOpen = 1,
    RegularOneSidedOpen = 2,
    SlowAsk = 3,
    SlowBid = 4,
    SlowBidASk = 5,
    SlowDueLrpBid = 6,
    SlowDueLrpAsk = 7,
    SlowDueNyseLrp = 8,
    SlowDueSetSlowListBidAsk = 9,
    ManualAskAutomatedBid = 10,
    ManualBidAutomatedAsk = 11,
    ManualBidAndAsk = 12,
    Opening = 13,
    Closing = 14,
    Closed = 15,
    Resume = 16,
    FastTrading = 17,
    TradingRangeIndication = 18,
    MarketMakerQuotesClosed = 19,
    NonFirm = 20,
    NewsDissemination = 21,
    OrderInflux = 22,
    OrderImbalance = 23,
}
