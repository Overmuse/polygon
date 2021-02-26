use crate::rest::Request;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

// Market holidays

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "status")]
pub enum MarketHolidayStatus {
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "early-close")]
    EarlyClose {
        open: DateTime<Utc>,
        close: DateTime<Utc>,
    },
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MarketHoliday {
    exchange: String,
    name: String,
    #[serde(flatten)]
    status: MarketHolidayStatus,
    date: NaiveDate,
}

pub struct GetMarketHolidays;

impl Request for GetMarketHolidays {
    type Body = ();
    type Response = Vec<MarketHoliday>;

    fn endpoint(&self) -> Cow<str> {
        "/v1/marketstatus/upcoming".into()
    }
}

// Market Status

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Status {
    #[serde(rename = "open")]
    Open,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "extended-hours")]
    ExtendedHours,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct ExchangesStatus {
    pub nyse: Status,
    pub nasdaq: Status,
    pub otc: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct CurrenciesStatus {
    pub fx: Status,
    pub crypto: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MarketStatus {
    pub market: Status,
    #[serde(rename = "serverTime")]
    pub server_time: DateTime<Utc>,
    pub exchanges: ExchangesStatus,
    pub currencies: CurrenciesStatus,
}

pub struct GetMarketStatus;

impl Request for GetMarketStatus {
    type Body = ();
    type Response = MarketStatus;

    fn endpoint(&self) -> Cow<str> {
        "/v1/marketstatus/now".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rest::Client;

    #[tokio::test]
    async fn get_market_holidays() {
        let client = Client::from_env().unwrap();
        println!("{:?}", client.send(GetMarketHolidays).await.unwrap())
    }
}
