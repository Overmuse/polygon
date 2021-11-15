use super::stocks::Timespan;
use chrono::{DateTime, Datelike, Duration, DurationRound, TimeZone, Utc};

fn snap_backward(start: DateTime<Utc>, timespan: Timespan) -> DateTime<Utc> {
    match timespan {
        Timespan::Minute => start.duration_trunc(Duration::minutes(1)).unwrap(),
        Timespan::Hour => start.duration_trunc(Duration::hours(1)).unwrap(),
        Timespan::Day => start.duration_trunc(Duration::days(1)).unwrap(),
        Timespan::Week => {
            let start = start.duration_trunc(Duration::days(1)).unwrap();
            start - Duration::days(start.weekday().num_days_from_sunday().into())
        }
        Timespan::Month => Utc.ymd(start.year(), start.month(), 1).and_hms(0, 0, 0),
        Timespan::Quarter => Utc
            .ymd(start.year(), 3 * ((start.month() - 1) / 3) + 1, 1)
            .and_hms(0, 0, 0),
        Timespan::Year => Utc.ymd(start.year(), 1, 1).and_hms(0, 0, 0),
    }
}

pub(crate) fn snap_forward(start: DateTime<Utc>, timespan: Timespan) -> DateTime<Utc> {
    match timespan {
        Timespan::Minute => {
            snap_backward(start, timespan) + Duration::minutes(1) - Duration::milliseconds(1)
        }
        Timespan::Hour => {
            snap_backward(start, timespan) + Duration::hours(1) - Duration::milliseconds(1)
        }
        Timespan::Day => {
            snap_backward(start, timespan) + Duration::days(1) - Duration::milliseconds(1)
        }
        Timespan::Week => {
            snap_backward(start, timespan) + Duration::weeks(1) - Duration::milliseconds(1)
        }
        Timespan::Month => {
            if start.month() == 12 {
                Utc.ymd(start.year() + 1, 1, 1).and_hms(0, 0, 0) - Duration::milliseconds(1)
            } else {
                Utc.ymd(start.year(), start.month() + 1, 1).and_hms(0, 0, 0)
                    - Duration::milliseconds(1)
            }
        }
        Timespan::Quarter => {
            if [10, 11, 12].contains(&start.month()) {
                Utc.ymd(start.year() + 1, 1, 1).and_hms(0, 0, 0) - Duration::milliseconds(1)
            } else {
                Utc.ymd(start.year(), 3 * ((start.month() - 1) / 3) + 4, 1)
                    .and_hms(0, 0, 0)
                    - Duration::milliseconds(1)
            }
        }
        Timespan::Year => {
            Utc.ymd(start.year() + 1, 1, 1).and_hms(0, 0, 0) - Duration::milliseconds(1)
        }
    }
}

fn is_multiple(
    date: DateTime<Utc>,
    base: DateTime<Utc>,
    multiplier: u32,
    timespan: Timespan,
) -> bool {
    let adjusted_date = date + Duration::milliseconds(1);
    let diff = adjusted_date - base;
    match timespan {
        Timespan::Minute => (diff.num_minutes() % i64::from(multiplier)) == 0,
        Timespan::Hour => (diff.num_minutes() % i64::from(multiplier * 60)) == 0,
        Timespan::Day => (diff.num_minutes() % i64::from(multiplier * 60 * 24)) == 0,
        Timespan::Week => (diff.num_minutes() % i64::from(multiplier * 60 * 24 * 7)) == 0,
        Timespan::Month => {
            let diff_months = (adjusted_date.year() - base.year()) * 12
                + (adjusted_date.month() - base.month()) as i32;
            diff_months % multiplier as i32 == 0
        }
        Timespan::Quarter => {
            let diff_months = (adjusted_date.year() - base.year()) * 12
                + (adjusted_date.month() - base.month()) as i32;
            diff_months % (multiplier * 3) as i32 == 0
        }
        Timespan::Year => {
            let diff_years = (adjusted_date.year() - base.year()) * 12;
            diff_years % multiplier as i32 == 0
        }
    }
}

pub(crate) fn adjust_timeperiods(
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    multiplier: u32,
    timespan: Timespan,
) -> (DateTime<Utc>, DateTime<Utc>) {
    let from = snap_backward(from, timespan);
    let mut to = snap_forward(to, timespan);
    while !is_multiple(to, from, multiplier, timespan) {
        to = snap_forward(to + Duration::milliseconds(1), timespan);
    }
    (from, to)
}

pub(crate) fn next_pagination_date(
    from: DateTime<Utc>,
    to: DateTime<Utc>,
    limit: u32,
    multiplier: u32,
    timespan: Timespan,
) -> DateTime<Utc> {
    let (max_periods, periods) = match timespan {
        Timespan::Minute => (limit, (to - from + Duration::microseconds(1)).num_minutes()),
        Timespan::Hour => (
            limit / 60,
            (to - from + Duration::microseconds(1)).num_hours(),
        ),
        Timespan::Day => (limit, (to - from + Duration::microseconds(1)).num_days()),
        Timespan::Week => (
            limit / 7,
            (to - from + Duration::microseconds(1)).num_weeks(),
        ),
        Timespan::Month => (
            limit / 31,
            (to - from + Duration::microseconds(1)).num_days() / 31,
        ),
        Timespan::Quarter => (
            limit / 92,
            (to - from + Duration::microseconds(1)).num_days() / 92,
        ),
        Timespan::Year => (
            limit / 365,
            (to - from + Duration::microseconds(1)).num_days() / 366,
        ),
    };
    if periods <= i64::from(max_periods) {
        to
    } else if max_periods == 0 {
        panic!("Limit is too small to create a request")
    } else {
        let diff = i64::from(max_periods) - 1;
        let snap_to = match timespan {
            Timespan::Minute => {
                from + Duration::minutes(diff - i64::from(max_periods % multiplier))
            }
            Timespan::Hour => from + Duration::hours(diff - i64::from(max_periods % multiplier)),
            Timespan::Day => from + Duration::days(diff - i64::from(max_periods % multiplier)),
            Timespan::Week => from + Duration::weeks(diff - i64::from(max_periods % multiplier)),
            Timespan::Month => {
                from + Duration::days(31 * (diff - i64::from(max_periods % multiplier)))
            }
            Timespan::Quarter => {
                from + Duration::days(92 * (diff - i64::from(max_periods % multiplier)))
            }
            Timespan::Year => {
                from + Duration::days(366 * (diff - i64::from(max_periods % multiplier)))
            }
        };
        snap_forward(snap_to, timespan)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_snap_period() {
        let start = Utc.ymd(2021, 5, 14).and_hms(1, 2, 3);
        assert_eq!(
            snap_backward(start, Timespan::Minute),
            Utc.ymd(2021, 5, 14).and_hms(1, 2, 0)
        );
        assert_eq!(
            snap_forward(start, Timespan::Minute),
            Utc.ymd(2021, 5, 14).and_hms_milli(1, 2, 59, 999)
        );
        assert_eq!(
            snap_backward(start, Timespan::Hour),
            Utc.ymd(2021, 5, 14).and_hms(1, 0, 0)
        );
        assert_eq!(
            snap_forward(start, Timespan::Hour),
            Utc.ymd(2021, 5, 14).and_hms_milli(1, 59, 59, 999)
        );
        assert_eq!(
            snap_backward(start, Timespan::Day),
            Utc.ymd(2021, 5, 14).and_hms(0, 0, 0)
        );
        assert_eq!(
            snap_forward(start, Timespan::Day),
            Utc.ymd(2021, 5, 14).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            snap_backward(start, Timespan::Week),
            Utc.ymd(2021, 5, 9).and_hms(0, 0, 0)
        );
        assert_eq!(
            snap_forward(start, Timespan::Week),
            Utc.ymd(2021, 5, 15).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            snap_backward(start, Timespan::Month),
            Utc.ymd(2021, 5, 1).and_hms(0, 0, 0)
        );
        assert_eq!(
            snap_forward(start, Timespan::Month),
            Utc.ymd(2021, 5, 31).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            snap_backward(start, Timespan::Quarter),
            Utc.ymd(2021, 4, 1).and_hms(0, 0, 0)
        );
        assert_eq!(
            snap_forward(start, Timespan::Quarter),
            Utc.ymd(2021, 6, 30).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            snap_backward(start, Timespan::Year),
            Utc.ymd(2021, 1, 1).and_hms(0, 0, 0)
        );
        assert_eq!(
            snap_forward(start, Timespan::Year),
            Utc.ymd(2021, 12, 31).and_hms_milli(23, 59, 59, 999)
        );
    }

    #[test]
    fn test_is_multiple() {
        let base = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        assert!(is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(0, 0, 59, 999),
            base,
            1,
            Timespan::Minute
        ));
        assert!(!is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(0, 0, 59, 999),
            base,
            2,
            Timespan::Minute
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(0, 1, 59, 999),
            base,
            2,
            Timespan::Minute
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(0, 59, 59, 999),
            base,
            1,
            Timespan::Hour
        ));
        assert!(!is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(0, 59, 59, 999),
            base,
            3,
            Timespan::Hour
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(2, 59, 59, 999),
            base,
            3,
            Timespan::Hour
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(23, 59, 59, 999),
            base,
            1,
            Timespan::Day
        ));
        assert!(!is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(23, 59, 59, 999),
            base,
            4,
            Timespan::Day
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 1, 4).and_hms_milli(23, 59, 59, 999),
            base,
            4,
            Timespan::Day
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 1, 7).and_hms_milli(23, 59, 59, 999),
            base,
            1,
            Timespan::Week
        ));
        assert!(!is_multiple(
            Utc.ymd(2021, 1, 1).and_hms_milli(23, 59, 59, 999),
            base,
            5,
            Timespan::Week
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 2, 4).and_hms_milli(23, 59, 59, 999),
            base,
            5,
            Timespan::Week
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 1, 31).and_hms_milli(23, 59, 59, 999),
            base,
            1,
            Timespan::Month
        ));
        assert!(!is_multiple(
            Utc.ymd(2021, 1, 31).and_hms_milli(23, 59, 59, 999),
            base,
            6,
            Timespan::Month
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 6, 30).and_hms_milli(23, 59, 59, 999),
            base,
            6,
            Timespan::Month
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 3, 31).and_hms_milli(23, 59, 59, 999),
            base,
            1,
            Timespan::Quarter
        ));
        assert!(!is_multiple(
            Utc.ymd(2021, 3, 31).and_hms_milli(23, 59, 59, 999),
            base,
            7,
            Timespan::Quarter
        ));
        assert!(is_multiple(
            Utc.ymd(2022, 9, 30).and_hms_milli(23, 59, 59, 999),
            base,
            7,
            Timespan::Quarter
        ));
        assert!(is_multiple(
            Utc.ymd(2021, 12, 31).and_hms_milli(23, 59, 59, 999),
            base,
            1,
            Timespan::Year
        ));
        assert!(!is_multiple(
            Utc.ymd(2021, 12, 31).and_hms_milli(23, 59, 59, 999),
            base,
            8,
            Timespan::Year
        ));
        assert!(is_multiple(
            Utc.ymd(2028, 12, 31).and_hms_milli(23, 59, 59, 999),
            base,
            8,
            Timespan::Year
        ));
    }

    #[test]
    fn adjust_time_periods() {
        let start = Utc.ymd(2021, 1, 1).and_hms(0, 0, 0);
        let end = Utc.ymd(2022, 1, 1).and_hms(0, 0, 0);
        assert_eq!(
            adjust_timeperiods(start, end, 1, Timespan::Minute),
            (
                Utc.ymd(2021, 1, 1).and_hms(0, 0, 0),
                Utc.ymd(2022, 1, 1).and_hms_milli(0, 0, 59, 999)
            )
        );
        assert_eq!(
            adjust_timeperiods(start, end, 2, Timespan::Hour),
            (
                Utc.ymd(2021, 1, 1).and_hms(0, 0, 0),
                Utc.ymd(2022, 1, 1).and_hms_milli(1, 59, 59, 999)
            )
        );
        assert_eq!(
            adjust_timeperiods(start, end, 3, Timespan::Day),
            (
                Utc.ymd(2021, 1, 1).and_hms(0, 0, 0),
                Utc.ymd(2022, 1, 1).and_hms_milli(23, 59, 59, 999)
            )
        );
        assert_eq!(
            adjust_timeperiods(start, end, 4, Timespan::Week),
            (
                Utc.ymd(2020, 12, 27).and_hms(0, 0, 0),
                Utc.ymd(2022, 1, 22).and_hms_milli(23, 59, 59, 999)
            )
        );
        assert_eq!(
            adjust_timeperiods(start, end, 5, Timespan::Month),
            (
                Utc.ymd(2021, 1, 1).and_hms(0, 0, 0),
                Utc.ymd(2022, 3, 31).and_hms_milli(23, 59, 59, 999)
            )
        );
        assert_eq!(
            adjust_timeperiods(start, end, 6, Timespan::Quarter),
            (
                Utc.ymd(2021, 1, 1).and_hms(0, 0, 0),
                Utc.ymd(2022, 6, 30).and_hms_milli(23, 59, 59, 999)
            )
        );
        assert_eq!(
            adjust_timeperiods(start, end, 7, Timespan::Year),
            (
                Utc.ymd(2021, 1, 1).and_hms(0, 0, 0),
                Utc.ymd(2027, 12, 31).and_hms_milli(23, 59, 59, 999)
            )
        );
    }

    #[test]
    fn test_next_pagination_date() {
        let from = Utc.ymd(2023, 1, 1).and_hms(0, 0, 0);
        let to = Utc.ymd(2032, 12, 31).and_hms_milli(23, 59, 59, 999);
        assert_eq!(
            next_pagination_date(from, to, 2, 1, Timespan::Minute),
            Utc.ymd(2023, 1, 1).and_hms_milli(0, 1, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 2, 2, Timespan::Minute),
            Utc.ymd(2023, 1, 1).and_hms_milli(0, 1, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 7, 5, Timespan::Minute),
            Utc.ymd(2023, 1, 1).and_hms_milli(0, 4, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 5270400, 1, Timespan::Minute),
            to
        );
        assert_eq!(
            next_pagination_date(from, to, 120, 1, Timespan::Hour),
            Utc.ymd(2023, 1, 1).and_hms_milli(1, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 120, 2, Timespan::Hour),
            Utc.ymd(2023, 1, 1).and_hms_milli(1, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 420, 5, Timespan::Hour),
            Utc.ymd(2023, 1, 1).and_hms_milli(4, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 5270400, 1, Timespan::Hour),
            to
        );
        assert_eq!(
            next_pagination_date(from, to, 2, 1, Timespan::Day),
            Utc.ymd(2023, 1, 2).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 2, 2, Timespan::Day),
            Utc.ymd(2023, 1, 2).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 7, 5, Timespan::Day),
            Utc.ymd(2023, 1, 5).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(next_pagination_date(from, to, 3660, 1, Timespan::Day), to);
        assert_eq!(
            next_pagination_date(from, to, 14, 1, Timespan::Week),
            Utc.ymd(2023, 1, 14).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 14, 2, Timespan::Week),
            Utc.ymd(2023, 1, 14).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 49, 5, Timespan::Week),
            Utc.ymd(2023, 2, 4).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(next_pagination_date(from, to, 3660, 1, Timespan::Week), to);
        assert_eq!(
            next_pagination_date(from, to, 62, 1, Timespan::Month),
            Utc.ymd(2023, 2, 28).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 62, 2, Timespan::Month),
            Utc.ymd(2023, 2, 28).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 217, 5, Timespan::Month),
            Utc.ymd(2023, 5, 31).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(next_pagination_date(from, to, 3660, 1, Timespan::Month), to);
        assert_eq!(
            next_pagination_date(from, to, 186, 1, Timespan::Quarter),
            Utc.ymd(2023, 6, 30).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 186, 2, Timespan::Quarter),
            Utc.ymd(2023, 6, 30).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 366, 3, Timespan::Quarter),
            Utc.ymd(2023, 9, 30).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 3660, 1, Timespan::Quarter),
            to
        );
        assert_eq!(
            next_pagination_date(from, to, 732, 1, Timespan::Year),
            Utc.ymd(2024, 12, 31).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 732, 2, Timespan::Year),
            Utc.ymd(2024, 12, 31).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(
            next_pagination_date(from, to, 2562, 5, Timespan::Year),
            Utc.ymd(2027, 12, 31).and_hms_milli(23, 59, 59, 999)
        );
        assert_eq!(next_pagination_date(from, to, 3660, 1, Timespan::Year), to);
    }
}
