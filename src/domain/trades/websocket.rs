use super::conditions::*;
use crate::domain::tape::Tape;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

fn is_zero(x: &u32) -> bool {
    *x == 0
}

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
    pub price: Decimal,
    #[serde(
        rename = "s",
        default = "Default::default",
        skip_serializing_if = "is_zero"
    )]
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
