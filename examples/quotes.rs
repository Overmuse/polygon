use chrono::NaiveDate;
use futures::{StreamExt, TryStreamExt};
use polygon::rest::{client, GetQuotes};
use std::env;
use stream_flatten_iters::TryStreamExt as _;

#[tokio::main]
async fn main() {
    let key = env::var("POLYGON_TOKEN").unwrap();
    let client = client(&key);
    let req = GetQuotes::new("GE", NaiveDate::from_ymd(2021, 11, 5)).limit(50000);

    client
        .send_paginated(&req)
        .map_ok(|x| x.results)
        .try_flatten_iters()
        .take(10)
        .for_each(|x| async move { println!("{:?}", x) })
        .await;
}
