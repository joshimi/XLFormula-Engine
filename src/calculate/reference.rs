use super::calculate_formula;
use crate::{
    parse_formula,
    types::{self, XlNum},
};
use std::{fmt::Debug, str::FromStr};

type NoCustomFunction<'a, N> = &'a fn(String, Vec<N>) -> types::Value<N>;

pub fn calculate_reference<N>(
    string: String,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match f {
        Some(f) => match f(string) {
            types::Value::Number(x) => types::Value::Number(x),
            types::Value::Text(s) => calculate_formula(
                parse_formula::parse_string_to_formula(&s, None::<NoCustomFunction<N>>),
                Some(f),
            ),
            types::Value::Boolean(x) => types::Value::Boolean(x),
            types::Value::Error(types::Error::Value) => types::Value::Error(types::Error::Value),
            types::Value::Iterator(v) => types::Value::Iterator(v),
            types::Value::Date(d) => types::Value::Date(d),
            types::Value::Blank => types::Value::Blank,
            _ => types::Value::Error(types::Error::Reference),
        },
        None => types::Value::Error(types::Error::Reference),
    }
}
