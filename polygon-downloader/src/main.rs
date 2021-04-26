use anyhow::{Context, Result};
use chrono::NaiveDate;
use csv::WriterBuilder;
use polygon::rest::{Client, GetAggregate, Timespan};
use rust_decimal::Decimal;
use serde::Serialize;

mod app;
use app::get_matches;

#[derive(Serialize)]
struct TickerAggregate<'a> {
    ticker: &'a str,
    // TODO: monitor https://github.com/BurntSushi/rust-csv/pull/223 and switch to
    // #[serde(flatten)] when available
    o: Decimal,
    h: Decimal,
    l: Decimal,
    c: Decimal,
    v: Decimal,
    vw: Option<Decimal>,
    t: u64,
    n: Option<u32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let matches = get_matches();
    let tickers = matches
        .values_of("tickers")
        .expect("Required value")
        .collect::<Vec<_>>();
    let token = std::env::var("POLYGON_TOKEN").context("Missing POLYGON_TOKEN env variable")?;
    let client = Client::new("https://api.polygon.io", &token);
    let start_date = NaiveDate::parse_from_str(
        matches.value_of("start-date").expect("Required value"),
        "%Y-%m-%d",
    )
    .context("Failed to parse start date")?;
    let end_date = NaiveDate::parse_from_str(
        matches.value_of("end-date").expect("Required value"),
        "%Y-%m-%d",
    )
    .context("Failed to parse end date")?;
    let multiplier = matches
        .value_of("multiplier")
        .expect("Required value")
        .parse::<u32>()
        .context("Failed to parse multiplier as an integer")?;
    let timespan = match matches.value_of("timespan").expect("Required value") {
        "minute" => Timespan::Minute,
        "hour" => Timespan::Hour,
        "day" => Timespan::Day,
        "week" => Timespan::Week,
        "quarter" => Timespan::Quarter,
        "year" => Timespan::Year,
        _ => unreachable!(),
    };
    let requests = tickers.iter().map(|&ticker| {
        GetAggregate::new(ticker, start_date, end_date)
            .multiplier(multiplier)
            .timespan(timespan)
            .unadjusted(!matches.is_present("adjust"))
            .limit(50000)
    });
    let output_file = format!(
        "{}.{}",
        matches.value_of("file").expect("Missing file name"),
        matches
            .value_of("output-format")
            .expect("Missing file format")
    );
    let mut writer = WriterBuilder::new()
        .from_path(output_file)
        .context("Failed to create csv writer")?;
    let v: Vec<_> = client.send_all(requests).await;
    for wrapper in v.into_iter() {
        let wrapper = wrapper.context("Error in data from polygon")?;
        for row in wrapper.results.expect("Missing data") {
            writer
                .serialize(TickerAggregate {
                    ticker: &wrapper.ticker,
                    o: row.o,
                    h: row.h,
                    l: row.l,
                    c: row.c,
                    v: row.v,
                    vw: row.vw,
                    t: row.t,
                    n: row.n,
                })
                .context("Failed to serialize data")?
        }
    }
    writer.flush().context("Failed to flush writer")?;
    Ok(())
}
