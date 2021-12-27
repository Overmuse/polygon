use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use uuid::Uuid;
use vila::Request;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssetClass {
    Stocks,
    Options,
    Crypto,
    Fx,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Locale {
    Us,
    Global,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerType {
    pub asset_class: AssetClass,
    pub code: String,
    pub description: String,
    pub locale: Locale,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TickerTypesWrapper {
    pub count: usize,
    pub request_id: Uuid,
    pub results: Vec<TickerType>,
    pub status: String,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct GetTickerTypes {
    asset_class: AssetClass,
    locale: Locale,
}

impl Request for GetTickerTypes {
    type Data = ();
    type Response = TickerTypesWrapper;

    fn endpoint(&self) -> Cow<str> {
        "/v3/reference/tickers/types".into()
    }
}
