use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use vila::Request;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StockDividend {
    pub amount: f64,
    pub ex_date: NaiveDate,
    pub payment_date: NaiveDate,
    pub record_date: NaiveDate,
    pub ticker: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StockDividendsWrapper {
    pub count: usize,
    pub status: String,
    pub results: Vec<StockDividend>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GetStockDividends {
    #[serde(rename = "stocksTicker")]
    pub stocks_ticker: String,
}

impl Request for GetStockDividends {
    type Data = ();
    type Response = StockDividendsWrapper;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/reference/dividends/{}", self.stocks_ticker).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rest::client_with_url;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_stock_dividends() {
        let _m = mock("GET", "/v2/reference/dividends/AAPL")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            // The format of dates here is different than that which appears in the Polygon docs.
            // But the endpoint definitely returns dates, not datetimes.
            .with_body(r#"{"count":2,"results":[{"amount":0.82,"exDate":"2020-05-08","paymentDate":"2020-05-14","recordDate":"2020-05-11","ticker":"AAPL"},{"amount":0.77,"exDate":"2020-02-07","paymentDate":"2020-02-13","recordDate":"2020-02-10","ticker":"AAPL"}],"status":"OK"}"#).create();
        let url = mockito::server_url();

        let client = client_with_url(&url, "TOKEN");
        let req = GetStockDividends {
            stocks_ticker: "AAPL".to_string(),
        };
        client.send(&req).await.unwrap();
    }
}
