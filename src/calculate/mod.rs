mod args;
mod display;
mod iterator;
mod operation;
mod reference;

use self::{
    display::{show_blank, show_iterator, show_number},
    iterator::calculate_iterator,
    operation::calculate_operation,
    reference::calculate_reference,
};
use crate::types::{self, XlNum};
use std::{fmt::Debug, str::FromStr};

/// Evaluates a string that was parsed and stored in Expression Struct.
/// Takes an optional closure with the trait bound Fn(String) -> types::Value.
pub fn calculate_formula<N>(
    formula: types::Formula<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match formula {
        types::Formula::Operation(exp) => calculate_operation(exp, f),
        types::Formula::Value(val) => val,
        types::Formula::Reference(string) => calculate_reference(string, f),
        types::Formula::Iterator(vec) => calculate_iterator(vec, f),
    }
}

/// Converts a result from Value Enum to a printable string.
pub fn result_to_string<N>(value: types::Value<N>) -> String
where
    N: XlNum,
{
    match value {
        types::Value::Number(number) => show_number(number),
        types::Value::Text(text) => text,
        types::Value::Error(error) => error.to_string(),
        types::Value::Boolean(boolean) => boolean.to_string(),
        types::Value::Iterator(value_vec) => show_iterator(value_vec),
        types::Value::Date(date) => date.to_string(),
        types::Value::Blank => show_blank::<N>(),
    }
}
