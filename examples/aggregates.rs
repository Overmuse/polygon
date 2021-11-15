use chrono::{NaiveDate, TimeZone, Utc};
use futures::{StreamExt, TryStreamExt};
use polygon::rest::{client, GetAggregate, Timespan};
use std::env;
use stream_flatten_iters::TryStreamExt as _;

#[tokio::main]
async fn main() {
    env_logger::init();
    let key = env::var("POLYGON_TOKEN").unwrap();
    let client = client(&key);
    let req = GetAggregate::new(
        "GE",
        Utc.from_utc_datetime(&NaiveDate::from_ymd(2011, 11, 5).and_hms(0, 0, 0)),
        Utc.from_utc_datetime(&NaiveDate::from_ymd(2021, 11, 5).and_hms(0, 0, 0)),
    )
    .multiplier(1)
    .timespan(Timespan::Minute)
    .limit(50000);
    log::debug!("{:?}", req);

    client
        .send_paginated(&req)
        .map_ok(|x| x.results)
        .try_flatten_iters()
        .for_each(|x| async move { println!("{:?}", x.unwrap()) })
        .await;
}
