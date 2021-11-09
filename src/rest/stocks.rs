use chrono::{
    serde::{ts_milliseconds, ts_nanoseconds, ts_nanoseconds_option},
    DateTime, NaiveDate, TimeZone, Utc,
};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt;
use vila::pagination::{PaginatedRequest, PaginationState, PaginationType, QueryPaginator};
use vila::{Request, RequestData};

// Quotes

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetQuotes<'a> {
    ticker: &'a str,
    date: NaiveDate,
    #[serde(with = "ts_nanoseconds")]
    timestamp: DateTime<Utc>,
    #[serde(rename = "timestampLimit", default, with = "ts_nanoseconds_option")]
    timestamp_limit: Option<DateTime<Utc>>,
    reverse: bool,
    limit: u32,
}

impl<'a> GetQuotes<'a> {
    pub fn new(ticker: &'a str, date: NaiveDate) -> Self {
        Self {
            ticker,
            date,
            timestamp: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
            timestamp_limit: None,
            reverse: false,
            limit: 5000,
        }
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn timestamp_limit(mut self, timestamp_limit: DateTime<Utc>) -> Self {
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
    #[serde(with = "ts_nanoseconds")]
    pub t: DateTime<Utc>,
    #[serde(with = "ts_nanoseconds")]
    pub y: DateTime<Utc>,
    #[serde(default, with = "ts_nanoseconds_option")]
    pub f: Option<DateTime<Utc>>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuoteWrapper {
    pub ticker: String,
    pub results_count: u32,
    pub db_latency: u32,
    pub success: bool,
    pub results: Vec<Quote>,
}

impl<'a> Request for GetQuotes<'a> {
    type Data = Self;
    type Response = QuoteWrapper;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/ticks/stocks/nbbo/{}/{}", self.ticker, self.date).into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

impl<'a> PaginatedRequest for GetQuotes<'a> {
    type Paginator = QueryPaginator<QuoteWrapper>;
    fn paginator(&self) -> Self::Paginator {
        QueryPaginator::new(|_: &PaginationState<PaginationType>, res: &QuoteWrapper| {
            res.results.iter().last().map(|q| {
                vec![(
                    "timestamp".to_string(),
                    format!("{}", q.t.timestamp_nanos()),
                )]
            })
        })
    }
}

// Aggregates

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(with = "ts_milliseconds")]
    pub t: DateTime<Utc>,
    pub n: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetAggregateQuery {
    unadjusted: bool,
    sort: SortOrder,
    limit: u32,
}

impl<'a> Request for GetAggregate<'a> {
    type Response = AggregateWrapper;
    type Data = GetAggregateQuery;

    fn endpoint(&self) -> Cow<str> {
        format!(
            "v2/aggs/ticker/{}/range/{}/{}/{}/{}",
            self.ticker, self.multiplier, self.timespan, self.from, self.to
        )
        .into()
    }

    fn data(&self) -> RequestData<&GetAggregateQuery> {
        RequestData::Query(&self.query)
    }
}

// Snapshot

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTickerSnapshot<'a>(pub &'a str);

impl Request for GetTickerSnapshot<'_> {
    type Data = ();
    type Response = TickerSnapshotWrapper;

    fn endpoint(&self) -> Cow<str> {
        format!("v2/snapshot/locale/us/markets/stocks/tickers/{}", self.0).into()
    }
}

#[non_exhaustive]
#[derive(Deserialize, Debug, Clone)]
pub enum TickerSnapshotStatus {
    OK,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TickerSnapshotWrapper {
    pub status: TickerSnapshotStatus,
    pub ticker: TickerSnapshot,
}

#[derive(Deserialize, Debug, Clone)]
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
    #[serde(with = "ts_nanoseconds")]
    pub updated: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AggregateSnapshot {
    pub av: Option<Decimal>,
    pub o: Decimal,
    pub h: Decimal,
    pub l: Decimal,
    pub c: Decimal,
    pub v: u64,
    pub vw: Decimal,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QuoteSnapshot {
    #[serde(rename = "p")]
    pub bid_price: Decimal,
    #[serde(rename = "s")]
    pub bid_size: u32,
    #[serde(rename = "P")]
    pub ask_price: Decimal,
    #[serde(rename = "S")]
    pub ask_size: u32,
    #[serde(with = "ts_nanoseconds")]
    pub t: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TradeSnapshot {
    // TODO: Implement with TradeCondition from ws.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub c: Option<Vec<u8>>,
    pub i: String,
    pub p: Decimal,
    pub s: u32,
    #[serde(with = "ts_nanoseconds")]
    pub t: DateTime<Utc>,
    pub x: u8,
}

// Previous close

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetPreviousClose<'a> {
    pub ticker: &'a str,
    pub unadjusted: bool,
}

impl Request for GetPreviousClose<'_> {
    type Data = Self;
    type Response = PreviousCloseWrapper;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/aggs/ticker/{}/prev", self.ticker).into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PreviousCloseWrapper {
    pub ticker: String,
    pub status: String,
    pub adjusted: bool,
    #[serde(rename = "queryCount")]
    pub query_count: usize,
    #[serde(rename = "resultsCount")]
    pub results_count: usize,
    pub request_id: String,
    pub results: Vec<PreviousClose>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PreviousClose {
    pub o: Decimal,
    pub h: Decimal,
    pub l: Decimal,
    pub c: Decimal,
    // TODO: This should be an integer, but Polygon sometimes returns the value in scientific
    // notation, which messes with deserialization
    pub v: Decimal,
    pub vw: Decimal,
    #[serde(with = "ts_milliseconds")]
    pub t: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rest::client_with_url;
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

        let client = client_with_url(&url, "TOKEN");
        let req = GetAggregate::new(
            "AAPL",
            NaiveDate::from_ymd(2021, 3, 1),
            NaiveDate::from_ymd(2021, 3, 1),
        );
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn get_quotes() {
        let _m = mock("GET", "/v2/ticks/stocks/nbbo/AAPL/2021-03-01")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            .with_body(r#"{"ticker":"AAPL","success":true,"results_count":2,"db_latency":43,"results":[{"t":1517562000065700400,"y":1517562000065321200,"q":2060,"c":[1],"z":3,"p":102.7,"s":60,"x":11,"P":0,"S":0,"X":0}]}"#).create();

        let url = mockito::server_url();

        let client = client_with_url(&url, "TOKEN");
        let req = GetQuotes::new("AAPL", NaiveDate::from_ymd(2021, 3, 1)).reverse(false);
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn get_quotes_paginated() {
        use futures::StreamExt;
        let _m = mock("GET", "/v2/ticks/stocks/nbbo/AAPL/2021-03-01")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            .with_body(r#"{"ticker":"AAPL","success":true,"results_count":2,"db_latency":43,"results":[{"t":1517562000065700400,"y":1517562000065321200,"q":2060,"c":[1],"z":3,"p":102.7,"s":60,"x":11,"P":0,"S":0,"X":0}]}"#).create();

        let url = mockito::server_url();

        let client = client_with_url(&url, "TOKEN");
        let req = GetQuotes::new("AAPL", NaiveDate::from_ymd(2021, 3, 1)).reverse(false);
        let mut stream = client.send_paginated(&req);
        stream.next().await.unwrap().unwrap();
        stream.next().await.unwrap().unwrap();
    }

    #[tokio::test]
    async fn get_ticker_snapshot() {
        let _m = mock("GET", "/v2/snapshot/locale/us/markets/stocks/tickers/AAPL")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            .with_body(r#"{"status":"OK","ticker":{"day":{"c":120.4229,"h":120.53,"l":118.81,"o":119.62,"v":28727868,"vw":119.725},"lastQuote":{"P":120.47,"S":4,"p":120.46,"s":8,"t":1605195918507251700},"lastTrade":{"c":null,"i":"4046","p":120.47,"s":236,"t":1605195918306274000,"x":10},"min":{"av":28724441,"c":120.4201,"h":120.468,"l":120.37,"o":120.435,"v":270796,"vw":120.4129},"prevDay":{"c":119.49,"h":119.63,"l":116.44,"o":117.19,"v":110597265,"vw":118.4998},"ticker":"AAPL","todaysChange":0.98,"todaysChangePerc":0.82,"updated":1605195918306274000}}"#).create();

        let url = mockito::server_url();

        let client = client_with_url(&url, "TOKEN");
        let req = GetTickerSnapshot("AAPL");
        client.send(&req).await.unwrap();
    }

    #[tokio::test]
    async fn get_previous_close() {
        let _m = mock("GET", "/v2/aggs/ticker/AAPL/prev")
            .match_query(Matcher::AllOf(vec![
                    Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()),
                    Matcher::UrlEncoded("unadjusted".into(), "false".into())
            ]))
            .with_body(r#"{"ticker":"AAPL","status":"OK","queryCount":1,"resultsCount":1,"adjusted":true,"results":[{"T":"AAPL","v":1.31704427e+079,"vw":116.3058,"o":115.55,"c":115.97,"h":117.59,"l":114.13,"t":1605042000000}],"request_id":"6a7e466379af0a71039d60cc78e72282"}"#).create();

        let url = mockito::server_url();

        let client = client_with_url(&url, "TOKEN");
        let req = GetPreviousClose {
            ticker: "AAPL",
            unadjusted: false,
        };
        client.send(&req).await.unwrap();
    }
}
