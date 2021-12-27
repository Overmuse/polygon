use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use vila::Request;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StockSplit {
    pub forfactor: Option<usize>,
    pub tofactor: Option<usize>,
    pub ratio: f64,
    pub declared_date: Option<NaiveDate>,
    pub ex_date: NaiveDate,
    pub payment_date: NaiveDate,
    pub ticker: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct StockSplitsWrapper {
    pub count: usize,
    pub status: String,
    pub results: Vec<StockSplit>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GetStockSplits {
    #[serde(rename = "stocksTicker")]
    pub stocks_ticker: String,
}

impl Request for GetStockSplits {
    type Data = ();
    type Response = StockSplitsWrapper;

    fn endpoint(&self) -> Cow<str> {
        format!("/v2/reference/splits/{}", self.stocks_ticker).into()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::rest::client_with_url;
    use mockito::{mock, Matcher};

    #[tokio::test]
    async fn get_stock_splits() {
        let _m = mock("GET", "/v2/reference/splits/AAPL")
            .match_query(Matcher::UrlEncoded("apiKey".into(), "TOKEN".into()))
            // The format of dates here is different than that which appears in the Polygon docs.
            // But the endpoint definitely returns dates, not datetimes.
            .with_body(r#"{"count":4,"results":[{"declaredDate":"2020-07-30","exDate":"2020-08-31","forfactor":4,"paymentDate":"2020-08-28","ratio":0.25,"ticker":"AAPL","tofactor":1},{"exDate":"2014-06-09","forfactor":7,"paymentDate":"2014-06-10","ratio":0.14285714285714285,"ticker":"AAPL","tofactor":1},{"exDate":"2005-02-28","paymentDate":"2005-02-28","ratio":0.5,"ticker":"AAPL"},{"exDate":"2000-06-21","paymentDate":"2000-06-21","ratio":0.5,"ticker":"AAPL"}],"status":"OK"}"#).create();
        let url = mockito::server_url();

        let client = client_with_url(&url, "TOKEN");
        let req = GetStockSplits {
            stocks_ticker: "AAPL".to_string(),
        };
        client.send(&req).await.unwrap();
    }
}
