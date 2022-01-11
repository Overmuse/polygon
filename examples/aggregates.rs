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
    );
    let req2 = GetAggregate::new(
        "AAPL",
        NaiveDate::from_ymd(2011, 11, 5).and_hms(0, 0, 0),
        NaiveDate::from_ymd(2021, 11, 5).and_hms(0, 0, 0),
    );
    let reqs = [req1, req2];

    futures::stream::select_all(reqs.iter().map(|req| {
        client
            .send_paginated(req)
            .map_ok(|x| {
                // Polygon aggregates don't have tickers associated with them, so we clone the
                // ticker as well and return it with each aggregate. That way, we can send the
                // requests in parallel but still know which ticker each aggregate is associated
                // with.
                let ticker = x.ticker.clone();
                x.results.into_iter().map(move |r| (ticker.clone(), r))
            })
            .try_flatten_iters()
    }))
    .for_each_concurrent(None, |x| async move { println!("{:?}", x) })
    .await;
}
