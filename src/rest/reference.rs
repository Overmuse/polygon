use crate::rest::Request;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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
    pub market: String,
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
    async fn get_exchange_status() {
        let client = Client::from_env().unwrap();
        println!("{:?}", client.send(GetMarketStatus).await.unwrap())
    }
}
