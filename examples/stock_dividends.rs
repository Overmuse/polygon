use polygon::rest::{client, GetStockDividends};
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();
    let key = env::var("POLYGON_TOKEN").unwrap();
    let client = client(&key);
    let req = GetStockDividends {
        stocks_ticker: "AAPL".to_string(),
    };

    println!("{:#?}", client.send(&req).await);
}
