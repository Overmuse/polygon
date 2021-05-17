use crate::rest::{Request, RequestBody};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;

// Quotes

#[derive(Serialize, Deserialize, Debug)]
pub struct GetQuotes<'a> {
    ticker: &'a str,
    date: NaiveDate,
    timestamp: u64,
    #[serde(rename = "timestampLimit")]
    timestamp_limit: Option<u64>,
    reverse: bool,
    limit: u32,
}

impl<'a> GetQuotes<'a> {
    pub fn new(ticker: &'a str, date: NaiveDate) -> Self {
        Self {
            ticker,
            date,
            timestamp: 0,
            timestamp_limit: None,
            reverse: false,
            limit: 5000,
        }
    }

    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn timestamp_limit(mut self, timestamp_limit: u64) -> Self {
        self.timestamp_limit = Some(timestamp_limit);
        self
    }

    pub fn reverse(mut self, reverse: bool) -> Self {
        self.reverse = reverse;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = limit;
        self
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quote {
    pub t: u64,
    pub y: u64,
    pub f: Option<u64>,
    pub q: u32,
    pub c: Vec<u32>,
    pub i: Option<Vec<u32>>,
    #[serde(rename = "p")]
    pub bid_price: Decimal,
    #[serde(rename = "x")]
    pub bid_exchange: u32,
    #[serde(rename = "s")]
    pub bid_size: u32,
    #[serde(rename = "P")]
    pub ask_price: Decimal,
    #[serde(rename = "X")]
    pub ask_exchange: u32,
    #[serde(rename = "S")]
    pub ask_size: u32,
    pub z: u8,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteWrapper {
    pub ticker: String,
    pub results_count: u32,
    pub db_latency: u32,
    pub success: bool,
    pub results: Vec<Quote>,
}

impl<'a> Request for GetQuotes<'a> {
    type Body = Self;
    type Response = QuoteWrapper;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/ticks/stocks/nbbo/{}/{}", self.ticker, self.date).into()
    }

    fn body(&self) -> RequestBody<&Self> {
        RequestBody::Query(&self)
    }
}

// Aggregates

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Timespan {
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl fmt::Display for Timespan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = match &self {
            Timespan::Minute => "minute",
            Timespan::Hour => "hour",
            Timespan::Day => "day",
            Timespan::Week => "week",
            Timespan::Month => "month",
            Timespan::Quarter => "quarter",
            Timespan::Year => "year",
        };
        write!(f, "{}", x)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Aggregate {
    pub o: Decimal,
    pub h: Decimal,
    pub l: Decimal,
    pub c: Decimal,
    pub v: Decimal,
    pub vw: Option<Decimal>,
    pub t: u64,
    pub n: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AggregateWrapper {
    pub ticker: String,
    pub status: String,
    pub adjusted: bool,
    #[serde(rename = "queryCount")]
    pub query_count: u32,
    #[serde(rename = "resultsCount")]
    pub results_count: u32,
    pub request_id: String,
    pub results: Option<Vec<Aggregate>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAggregate<'a> {
    #[serde(rename = "stocksTicker")]
    ticker: &'a str,
    multiplier: u32,
    timespan: Timespan,
    from: NaiveDate,
    to: NaiveDate,
    query: GetAggregateQuery,
}

impl<'a> GetAggregate<'a> {
    pub fn new(ticker: &'a str, from: NaiveDate, to: NaiveDate) -> Self {
        Self {
            ticker,
            multiplier: 1,
            timespan: Timespan::Day,
            from,
            to,
            query: GetAggregateQuery {
                unadjusted: false,
                sort: SortOrder::Asc,
                limit: 5000,
            },
        }
    }

    pub fn multiplier(mut self, multiplier: u32) -> Self {
        self.multiplier = multiplier;
        self
    }

    pub fn timespan(mut self, timespan: Timespan) -> Self {
        self.timespan = timespan;
        self
    }

    pub fn unadjusted(mut self, unadjusted: bool) -> Self {
        self.query.unadjusted = unadjusted;
        self
    }

    pub fn sort(mut self, sort: SortOrder) -> Self {
        self.query.sort = sort;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.query.limit = limit;
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAggregateQuery {
    unadjusted: bool,
    sort: SortOrder,
    limit: u32,
}

impl<'a> Request for GetAggregate<'a> {
    type Response = AggregateWrapper;
    type Body = GetAggregateQuery;

    fn endpoint(&self) -> Cow<str> {
        format!(
            "v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            self.ticker, self.multiplier, self.timespan, self.from, self.to
        )
        .into()
    }

    fn body(&self) -> RequestBody<&GetAggregateQuery> {
        RequestBody::Query(&self.query)
    }
}

// Snapshot

#[derive(Serialize, Deserialize, Debug)]
pub struct GetTickerSnapshot<'a>(pub &'a str);

impl Request for GetTickerSnapshot<'_> {
    type Body = ();
    type Response = TickerSnapshotWrapper;

    fn endpoint(&self) -> Cow<str> {
        format!("v2/snapshot/locale/us/markets/stocks/tickers/{}", self.0).into()
    }
}

#[non_exhaustive]
#[derive(Deserialize, Debug)]
pub enum TickerSnapshotStatus {
    OK,
}

#[derive(Deserialize, Debug)]
pub struct TickerSnapshotWrapper {
    pub status: TickerSnapshotStatus,
    pub ticker: TickerSnapshot,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TickerSnapshot {
    pub day: AggregateSnapshot,
    pub last_quote: QuoteSnapshot,
    pub last_trade: TradeSnapshot,
    #[serde(rename = "min")]
    pub minute: AggregateSnapshot,
    #[serde(rename = "prevDay")]
    pub previous_day: AggregateSnapshot,
    pub ticker: String,
    pub todays_change: Decimal,
    #[serde(rename = "todaysChangePerc")]
    pub todays_change_percent: Decimal,
    pub updated: u64,
}

#[derive(Deserialize, Debug)]
pub struct AggregateSnapshot {
    pub av: Option<Decimal>,
    pub o: Decimal,
    pub h: Decimal,
    pub l: Decimal,
    pub c: Decimal,
    pub v: u64,
    pub vw: Decimal,
}

#[derive(Deserialize, Debug)]
pub struct QuoteSnapshot {
    #[serde(rename = "p")]
    pub bid_price: Decimal,
    #[serde(rename = "s")]
    pub bid_size: u32,
    #[serde(rename = "P")]
    pub ask_price: Decimal,
    #[serde(rename = "S")]
    pub ask_size: u32,
    pub t: u64,
}

#[derive(Deserialize, Debug)]
pub struct TradeSnapshot {
    // TODO: Implement with TradeCondition from ws.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub c: Option<Vec<u8>>,
    pub i: String,
    pub p: Decimal,
    pub s: u32,
    pub t: u64,
    pub x: u8,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rest::Client;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_aggregate() {
        let _aggs_mock = mock("GET", "/v2/aggs/ticker/AAPL/range/1/day/2021-03-01/2021-03-01")
            .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()),
                    Matcher::UrlEncoded("unadjusted".into(), "false".into()),
                    Matcher::UrlEncoded("sort".into(), "asc".into()),
                    Matcher::UrlEncoded("limit".into(), "5000".into()),
            ]))
            .with_body(r#"{"ticker":"AAPL","status":"OK","queryCount":2,"resultsCount":2,"adjusted":true,"results":[{"v":1.35647456e+08,"vw":74.6099,"o":74.06,"c":75.0875,"h":75.15,"l":73.7975,"t":1577941200000,"n":1}],"request_id":"6a7e466379af0a71039d60cc78e72282"}"#)
            .create();
        let url = mockito::server_url();

        let client = Client::new(&url, "TOKEN");
        let req = GetAggregate::new(
            "AAPL",
            NaiveDate::from_ymd(2021, 3, 1),
            NaiveDate::from_ymd(2021, 3, 1),
        );
        client.send(req).await.unwrap();
    }

    #[tokio::test]
    async fn get_quotes() {
        let _m = mock("GET", "/v2/ticks/stocks/nbbo/AAPL/2021-03-01")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            .with_body(r#"{"ticker":"AAPL","success":true,"results_count":2,"db_latency":43,"results":[{"t":1517562000065700400,"y":1517562000065321200,"q":2060,"c":[1],"z":3,"p":102.7,"s":60,"x":11,"P":0,"S":0,"X":0}]}"#).create();

        let url = mockito::server_url();

        let client = Client::new(&url, "TOKEN");
        let req = GetQuotes::new("AAPL", NaiveDate::from_ymd(2021, 3, 1)).reverse(false);
        client.send(req).await.unwrap();
    }

    #[tokio::test]
    async fn get_ticker_snapshot() {
        let _m = mock("GET", "/v2/snapshot/locale/us/markets/stocks/tickers/AAPL")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            .with_body(r#"{"status":"OK","ticker":{"day":{"c":120.4229,"h":120.53,"l":118.81,"o":119.62,"v":28727868,"vw":119.725},"lastQuote":{"P":120.47,"S":4,"p":120.46,"s":8,"t":1605195918507251700},"lastTrade":{"c":null,"i":"4046","p":120.47,"s":236,"t":1605195918306274000,"x":10},"min":{"av":28724441,"c":120.4201,"h":120.468,"l":120.37,"o":120.435,"v":270796,"vw":120.4129},"prevDay":{"c":119.49,"h":119.63,"l":116.44,"o":117.19,"v":110597265,"vw":118.4998},"ticker":"AAPL","todaysChange":0.98,"todaysChangePerc":0.82,"updated":1605195918306274000}}"#).create();

        let url = mockito::server_url();

        let client = Client::new(&url, "TOKEN");
        let req = GetTickerSnapshot("AAPL");
        client.send(req).await.unwrap();
    }
}
