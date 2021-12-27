use super::Locale;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::{Request, RequestData};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Market {
    Stocks,
    Crypto,
    Fx,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TickerDetails {
    pub cik: String,
    pub composite_figi: String,
    pub currency_name: String,
    pub last_updated_utc: DateTime<Utc>,
    pub locale: Locale,
    pub market: Market,
    pub market_cap: usize,
    pub name: String,
    pub outstanding_shares: usize,
    pub phone_number: String,
    pub primary_exchange: String,
    pub share_class_figi: String,
    pub sic_code: String,
    pub sic_description: String,
    pub ticker: String,
    pub r#type: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TickerDetailsWrapper {
    pub count: usize,
    pub request_id: Uuid,
    pub results: TickerDetails,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetTickerDetails {
    ticker: String,
    date: Option<NaiveDate>,
}

impl Request for GetTickerDetails {
    type Data = Self;
    type Response = TickerDetailsWrapper;

    fn endpoint(&self) -> Cow<str> {
        format!("/vX/reference/tickers/{}", self.ticker).into()
    }

    fn data(&self) -> RequestData<&Self> {
        RequestData::Query(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rest::client_with_url;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_ticker_details() {
        let _m = mock("GET", "/vX/reference/tickers/AAPL")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            .with_body(r#"{"count":1,"request_id":"31d59dda-80e5-4721-8496-d0d32a654afe","results":{"active":true,"address":{"address1":"One Apple Park Way","city":"Cupertino","state":"CA"},"cik":"0000320193","composite_figi":"BBG000B9XRY4","currency_name":"usd","last_updated_utc":"2020-12-27T00:00:00Z","locale":"us","market":"stocks","market_cap":2082042128180,"name":"Apple Inc.","outstanding_shares":17001800000,"phone_number":"(408) 996-1010","primary_exchange":"XNAS","share_class_figi":"BBG001S5N8V8","sic_code":"3571","sic_description":"ELECTRONIC COMPUTERS","ticker":"AAPL","type":"CS"},"status":"OK"}"#).create();
        let url = mockito::server_url();

        let client = client_with_url(&url, "TOKEN");
        let req = GetTickerDetails {
            ticker: "AAPL".to_string(),
            date: None,
        };
        client.send(&req).await.unwrap();
    }
}
