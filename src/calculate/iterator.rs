use super::{
    calculate_formula,
    operation::boolean::{calculate_boolean_operator, cast_value_to_boolean},
};
use crate::types::{self, XlNum};
use std::{fmt::Debug, str::FromStr};

pub fn convert_iterator_to_result<N>(
    result: types::Value<N>,
    f: fn(bool, bool) -> bool,
    allow_error: bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match result {
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f, allow_error);
                }
                match cast_value_to_boolean(temp) {
                    value @ types::Value::Boolean(_) => value,
                    _ => types::Value::Error(types::Error::Value),
                }
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        _ => result,
    }
}

pub fn calculate_iterator<N>(
    mut vec: Vec<types::Formula<N>>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut value_vec = Vec::new();
    while let Some(top) = vec.pop() {
        value_vec.push(calculate_formula(top, f));
    }
    types::Value::Iterator(value_vec)
}
