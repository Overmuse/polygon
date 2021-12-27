use chrono::NaiveDate;
use futures::{StreamExt, TryStreamExt};
use polygon::rest::{client, GetAggregate};
use std::env;
use stream_flatten_iters::TryStreamExt as _;

#[tokio::main]
async fn main() {
    env_logger::init();
    let key = env::var("POLYGON_TOKEN").unwrap();
    let client = client(&key);
    let req1 = GetAggregate::new(
        "GE",
        NaiveDate::from_ymd(2011, 11, 5).and_hms(0, 0, 0),
        NaiveDate::from_ymd(2021, 11, 5).and_hms(0, 0, 0),
    )
    .limit(1);
    let req2 = GetAggregate::new(
        "AAPL",
        NaiveDate::from_ymd(2011, 11, 5).and_hms(0, 0, 0),
        NaiveDate::from_ymd(2021, 11, 5).and_hms(0, 0, 0),
    )
    .limit(1);
    let reqs = [req1, req2];

    futures::stream::select_all(client.send_all_paginated(&reqs).map(|stream| {
        stream
            .map_ok(|x| {
                let ticker = x.ticker.clone();
                x.results.into_iter().map(move |r| (ticker.clone(), r))
            })
            .try_flatten_iters()
            .take(10)
    }))
    .for_each_concurrent(None, |x| async move { println!("{:?}", x) })
    .await;
}
