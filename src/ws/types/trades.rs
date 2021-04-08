use serde::{Deserialize, Serialize};
use serde_repr::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Trade {
    #[serde(rename = "sym")]
    pub symbol: String,
    #[serde(rename = "x")]
    pub exchange_id: u8,
    #[serde(rename = "i")]
    pub trade_id: String,
    #[serde(rename = "z")]
    pub tape: Tape,
    #[serde(rename = "p")]
    pub price: f64,
    #[serde(rename = "s")]
    pub size: u32,
    #[serde(
        rename = "c",
        default = "default_conditions",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub conditions: Vec<TradeCondition>,
    #[serde(rename = "t")]
    pub timestamp: u64,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum Tape {
    A = 1,
    B = 2,
    C = 3,
}

#[derive(Serialize_repr, Deserialize_repr, Debug, Clone, PartialEq, Hash, Eq)]
#[repr(u8)]
pub enum TradeCondition {
    /// A trade made without stated conditions is deemed regular way for settlement on the third
    /// business day following the transaction date.
    RegularSale = 0,
    /// A transaction made on the Exchange as a result of an Exchange acquisition.
    Acquisition = 1,
    /// A trade where the price reported is based upon an average of the prices for transactions in
    /// a security during all or any portion of the trading day.
    AveragePriceTrade = 2,
    /// A sale condition code that identifies a NYSE trade that has been automatically executed
    /// without the potential benefit of price improvement.
    AutomaticExecution = 3,
    /// The combining of multiple odd-lot or round-lot orders for the same security so that they
    /// can all be executed at the same time. All affected clients must agree to the bunching
    /// before the order is submitted. Bunched trades may also be referred to as block trades.
    BunchedTrade = 4,
    /// A bunched trade that is reported late.
    BunchedSoldTrade = 5,
    CapElection = 6,
    /// A transaction which requires delivery of securities and payment on the same day the trade
    /// takes place.
    CashSale = 7,
    /// The Participant Closing Price represents the last qualifying trade paid for a security by a
    /// Participant during the trading day.
    ClosingPrints = 8,
    /// Indicates that the trade resulted from a Market Center’s crossing session.
    CrossTrade = 9,
    /// The transaction that constituted the trade-through was the execution of an order at a price
    /// that was not based, directly or indirectly, on the quoted price of the security at the time
    /// of execution, and for which the material terms were not reasonably determinable at the time
    /// the commitment to execute the order was made (REG NMS 611b7).
    DerivativelyPriced = 10,
    /// Distribution stock refers to a large blocks of a security that are carefully sold into the
    /// market gradually in smaller blocks so as to inundate the market with sell orders for the
    /// security and driving down its price.
    Distribution = 11,
    /// Identifies a trade that was executed outside of regular primary market hours and is
    /// reported as an extended hours trade.
    FormT = 12,
    /// Identifies a trade that takes place outside of regular market hours and is reported as an
    /// extended hours trade out of sequence and at a time different from the actual transaction
    /// time.
    ExtendedTradingHoursSoldOutOfSequence = 13,
    /// The transaction that constituted the trade-through was the execution of an order identified
    /// as an Intermarket Sweep Order.
    IntermarketSweep = 14,
    /// Indicates the ‘Official’ closing value as determined by a Market Center. This transaction
    /// report will contain the market center generated closing price.
    MarketCenterOfficialClose = 15,
    /// Indicates the ‘Official’ opening value as determined by a Market Center. This transaction
    /// report will contain the market center generated opening price.
    MarketCenterOfficialOpen = 16,
    /// The trade that constituted the trade-through was a single priced opening transaction by the
    /// Market Center (REG NMS Rule 611b3).
    MarketCenterOpeningTrade = 17,
    /// The trade that constituted the trade-through was a single priced reopening transaction by
    /// the Market Center (REG NMS Rule 611b3).
    MarketCenterReopeningTrade = 18,
    /// The transaction that constituted the trade-through was a single priced closing transaction
    /// by the Market Center (REG NMS Rule 611b3).
    MarketCenterClosingTrade = 19,
    /// A transaction that requires the delivery of securities on the first business day following
    /// the trade date.
    NextDay = 20,
    /// Indicates a regular market session trade transaction that carries a price that is
    /// significantly away from the prevailing consolidated or primary market value at the time of
    /// the transaction.
    PriceVariationTrade = 21,
    /// A sale condition that identifies a trade based on a price at a prior point in time, i.e.,
    /// more than 90 seconds prior to the time of the trade report. The execution time of the trade
    /// will be the time of the prior reference price.
    PriorReferencePrice = 22,
    /// A Seller’s Option transaction gives the seller the right to deliver the security at any
    /// time within a specific period, ranging from not less than two calendar days, to not more
    /// than sixty calendar days. A security offered “Seller’s Option” may command a lesser price
    /// than if offered “Regular Way”.
    Rule155Trade = 23,
    /// "To qualify as a NYSE Rule 127 the trade is executed outside the present quote and meets
    /// one or both of the following conditions: 1. has a volume of 10,000 shares or more and/or 2.
    /// has a dollar value of $200,000 or more."
    Rule127Trade = 24,
    /// The trading day's first drawings of a symbol's candlestick charts.
    OpeningPrints = 25,
    Opened = 26,
    /// A Seller’s Option transaction gives the seller the right to deliver the security at any
    /// time within a specific period, ranging from not less than two calendar days, to not more
    /// than sixty calendar days. A security offered “Seller’s Option” may command a lesser price
    /// than if offered “Regular Way”.
    StoppedStockRegularTrade = 27,
    /// The transaction or group of transactions reported as a result of a single- priced
    /// re-opening event by the Market Center.
    ReopeningPrints = 28,
    /// A Seller’s Option transaction gives the seller the right to deliver the security at any
    /// time within a specific period, ranging from not less than two calendar days, to not more
    /// than sixty calendar days. A security offered “Seller’s Option” may command a lesser price
    /// than if offered “Regular Way”.
    Seller = 29,
    /// Sold Last sale condition modifier is used when a trade prints in sequence but is reported
    /// late OR the trade is printed by Amex in conformance to the One or Two Point Rule. A Sold
    /// Last transaction should only impact the consolidated last sale price for an issue if the
    /// market center reporting the sold last transaction also reported the transaction setting the
    /// current last sale price.
    SoldLast = 30,
    SoldOut = 32,
    /// Sold Out of Sequence is used when a trade is printed (reported) out of sequence and at a
    /// time different from the actual transaction time.
    SoldOutOfSequence = 33,
    /// An execution in two markets when the specialist or Market Maker in the market first
    /// receiving the order agrees to execute a portion of it at whatever price is realized in
    /// another market to which the balance of the order is forwarded for execution.
    SplitTrade = 34,
    /// This is typically the stock portion of a delta neutral option trade executed by an option
    /// market maker.
    StockOption = 35,
    /// Market Centers will have the ability to identify regular trades being reported during
    /// specific events as out of the ordinary by appending a new sale condition code Yellow Flag
    /// (“Y”) on each transaction reported to the UTP SIP. The new sale condition “.Y” will be
    /// eligible to update all market center and consolidated statistics. In certain instances, the
    /// UTP SIP will be required to append the .Y for the market center for trades reported as
    /// regular-way (Sale Condition @)
    YellowFlagRegularTrade = 36,
    /// The Odd Lot Trade modifier will distinguish a trade resulting from a market center's
    /// execution in increments less than the defined round lot size.
    OddLotTrade = 37,
    /// A transaction executed by the Listing Market to establish the official Consolidated Last
    /// Price as indicated by the Listing Exchange.
    CorrectedConsolidatedClose = 38,
    Unknown = 39,
    /// Trades received from a non-primary Participant during a primary market regulatory halt.
    /// These trades are held by the CTS Processor and are disseminated after the close of the
    /// primary market with an appropriate Held Trade Indicator code applicable to the trade.
    Held = 40,
    /// The Trade Through rule is a 20 year-old rule applied to NYSE-listed stocks that states that
    /// when a market receives an order, it cannot execute it at a price inferior to any found on
    /// another market. In modern electronic markets where trades are executed in milliseconds,
    /// this rule can prevent a broker’s ability to meet their “best execution” obligation--because
    /// speed provides certainty that the price that is advertised can be accessed.:w
    ///
    TradeThruExempt = 41,
    NonEligible = 42,
    NonEligibleExtended = 43,
    Cancelled = 44,
    Recovery = 45,
    /// Denotes a correction to the last indication or new indication. It will contain the
    /// corrected approximation of what that security's opening or reopening price range (Bid and
    /// Offer prices, no sizes) will be when trading resumes after a delayed opening or after a
    /// trading halt.
    Correction = 46,
    AsOf = 47,
    AsOfCorrection = 48,
    AsOfCancel = 49,
    Oob = 50,
    Summary = 51,
    /// A Sale Condition code used to identify a transaction where the execution of the transaction
    /// is contingent upon some event.
    ContingentTrade = 52,
    /// A transaction consisting of two or more component orders executed as agent or principal
    /// where the execution of one component is contingent upon the execution of all other
    /// components at or near the same time and the price is determined by the relationship between
    /// the component orders and not the current market price for the security.
    QualifiedContingentTrade = 53,
    Errored = 54,
    OpeningReopeningTradeDetail = 55,
    IntradayTradeDetail = 56,
    ShortSaleRestrictionsActivated = 57,
    ShortSaleRestrictionsContinued = 58,
    ShortSaleRestrictionsDeactivated = 59,
    /// Any stock that has dropped more than 10% intraday has SSR in effect for that day and the following.
    ShortSaleRestrictionsInEffect = 60,
    FinancialStatusNormal = 61,
    FinancialStatusBankrupt = 62,
    FinancialStatusDeficient = 63,
    FinancialStatusDelinquent = 64,
    FinancialStatusBankruptAndDeficient = 65,
    FinancialStatusBankruptAndDelinquent = 66,
    FinancialStatusDeficientAndDelinquent = 67,
    FinancialStatusDeficientDelinquentAndBankrupt = 68,
    FinancialStatusLiquidation = 69,
    FinancialStatusCreationsSuspended = 70,
    FinancialStatusRedemptionsSuspended = 71,
    FinancialStatusCreationsAndOrRedemptionsSuspended = 72,
}

fn default_conditions() -> Vec<TradeCondition> {
    Vec::new()
}
