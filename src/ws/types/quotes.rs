use chrono::{serde::ts_milliseconds, DateTime, Utc};
use rust_decimal::Decimal;
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
    #[serde(rename = "t", with = "ts_milliseconds")]
    pub timestamp: DateTime<Utc>,
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
    DueToRelatedSecurityNewsDissemination = 24,
    DueToRelatedSecurityNewsPending = 25,
    AdditionalInformation = 26,
    NewsPending = 27,
    AdditionalInformationDueToRelatedSecurity = 28,
    DueToRelatedSecurity = 29,
    InViewOfCommon = 30,
    EquipmentChangeover = 31,
    NoOpenNoResponse = 32,
    SubPennyTrading = 33,
    AutomatedBidNoOfferNoBid = 34,
    LuldPriceBand = 35,
    MarketWideCircuitBreakerLevel1 = 36,
    MarketWideCircuitBreakerLevel2 = 37,
    MarketWideCircuitBreakerLevel3 = 38,
    RepublishedLuldPriceBand = 39,
    OnDemandAuction = 40,
    CashOnlySettlement = 41,
    NextDaySettlement = 42,
    LuldTradingPause = 43,
    SlowDuelRpBidAsk = 71,
    Cancel = 80,
    CorrectedPrice = 81,
    SipGenerated = 82,
    Unknown = 83,
    CrossedMarket = 84,
    LockedMarket = 85,
    DepthOnOfferSide = 86,
    DepthOnBidSide = 87,
    DepthOnBidAndOffer = 88,
    PreOpeningIndication = 89,
    SyndicateBid = 90,
    PreSyndicateBid = 91,
    PenaltyBid = 92,
}
