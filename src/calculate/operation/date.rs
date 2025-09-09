use crate::types::{self, XlNum};
use chrono::{DateTime, Datelike, Duration, FixedOffset};
use std::{fmt::Debug, str::FromStr};

pub fn add_days_to_date<N>(d: DateTime<FixedOffset>, rhs: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Number(x) => types::Value::Date(d + Duration::days(x.as_())),
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn subtract_days_from_date<N>(d: DateTime<FixedOffset>, rhs: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Number(x) => types::Value::Date(d - Duration::days(x.as_())),
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_days<N>(start: types::Value<N>, end: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let begin_of_date: DateTime<FixedOffset> =
        DateTime::parse_from_rfc3339("1900-01-01T02:00:00.000Z")
            .ok()
            .unwrap();
    match (start, end) {
        (types::Value::Date(start), types::Value::Date(end)) => {
            types::Value::Number(N::from_i64((end - start).num_days()).unwrap())
        }
        (types::Value::Blank, types::Value::Date(end)) => {
            types::Value::Number(N::from_i64((end - begin_of_date).num_days()).unwrap())
        }
        (types::Value::Date(start), types::Value::Blank) => {
            types::Value::Number(N::from_i64((begin_of_date - start).num_days()).unwrap())
        }
        (types::Value::Blank, types::Value::Blank) => types::Value::Number(N::zero()),
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_year<N>(date_value: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match date_value {
        types::Value::Date(d) => match N::from_i32(d.year()) {
            Some(year) => types::Value::Number(year),
            None => types::Value::Error(types::Error::Value),
        },
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_month<N>(date_value: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match date_value {
        types::Value::Date(d) => match N::from_u32(d.month()) {
            Some(year) => types::Value::Number(year),
            None => types::Value::Error(types::Error::Value),
        },
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_day<N>(date_value: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match date_value {
        types::Value::Date(d) => match N::from_u32(d.day()) {
            Some(year) => types::Value::Number(year),
            None => types::Value::Error(types::Error::Value),
        },
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn compare_dates<N>(
    date1: DateTime<FixedOffset>,
    date2: DateTime<FixedOffset>,
    f: fn(d1: DateTime<FixedOffset>, d2: DateTime<FixedOffset>) -> bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    types::Value::Boolean(f(date1, date2).into())
}
