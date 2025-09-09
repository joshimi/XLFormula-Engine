use crate::types::{self, XlNum};
use std::{fmt::Debug, str::FromStr};

pub fn calculate_concat_operator(str1: &str, str2: &str) -> String {
    str1.to_owned() + str2
}

fn calculate_string_operation_rhs<N>(
    l: &str,
    rhs: types::Value<N>,
    f: fn(str1: &str, str2: &str) -> String,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Boolean(_) | types::Value::Error(_) => rhs,
        types::Value::Number(r) => types::Value::Text(f(l, &r.to_string())),
        types::Value::Text(r) => types::Value::Text(f(l, &r)),
        types::Value::Iterator(_) | types::Value::Date(_) => {
            types::Value::Error(types::Error::Value)
        }
        types::Value::Blank => types::Value::Text(f(l, "")),
    }
}

pub fn calculate_string_operator<N>(
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(str1: &str, str2: &str) -> String,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match lhs {
        types::Value::Boolean(_) | types::Value::Error(_) => lhs,
        types::Value::Number(l) => calculate_string_operation_rhs(&l.to_string(), rhs, f),
        types::Value::Text(l) => calculate_string_operation_rhs(&l, rhs, f),
        types::Value::Iterator(_) | types::Value::Date(_) => {
            types::Value::Error(types::Error::Value)
        }
        types::Value::Blank => calculate_string_operation_rhs("", rhs, f),
    }
}

pub fn compare_strings<N>(
    string1: String,
    string2: String,
    f: impl Fn(String, String) -> bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    types::Value::Boolean(f(string1, string2).into())
}
