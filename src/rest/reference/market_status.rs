use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use vila::Request;

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
    type Data = ();
    type Response = MarketStatus;

    fn endpoint(&self) -> Cow<str> {
        "/v1/marketstatus/now".into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rest::client_with_url;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_market_status() {
        let _m = mock("GET", "/v1/marketstatus/now")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            .with_body(r#"{"market":"extended-hours","serverTime":"2020-11-10T22:37:37.000Z","exchanges":{"nyse":"extended-hours","nasdaq":"extended-hours","otc":"closed"},"currencies":{"fx":"open","crypto":"open"}}"#).create();
        let url = mockito::server_url();

        let client = client_with_url(&url, "TOKEN");
        let req = GetMarketStatus;
        client.send(&req).await.unwrap();
    }
}
