use super::calculate_formula;
use crate::types::{self, XlNum};
use std::{fmt::Debug, str::FromStr};

pub fn get_unary_function_arg<N>(
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    }
}

pub fn get_binary_function_args<N>(
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> (types::Value<N>, types::Value<N>)
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let rhs = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    };
    let lhs = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    };
    (lhs, rhs)
}

pub fn get_ternary_function_args<N>(
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> (types::Value<N>, types::Value<N>, types::Value<N>)
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let last = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Blank,
    };
    let middle = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Blank,
    };
    let first = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Blank,
    };
    (first, middle, last)
}

/// Returns (find_text, within_text, start_num) for FIND/SEARCH.
/// start_num defaults to 1 if only two arguments are provided.
pub fn get_find_args<N>(
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> (types::Value<N>, types::Value<N>, types::Value<N>)
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let start_num = if exp.values.len() >= 3 {
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f),
            None => types::Value::Number(N::one()),
        }
    } else {
        types::Value::Number(N::one())
    };
    let within_text = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    };
    let find_text = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    };
    (find_text, within_text, start_num)
}

pub fn get_number_and_string_values<N>(
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> (types::Value<N>, types::Value<N>)
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let (number, string) = if exp.values.len() == 1 {
        (
            types::Value::Number(N::one()),
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => types::Value::Error(types::Error::Argument),
            },
        )
    } else {
        (
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => types::Value::Error(types::Error::Argument),
            },
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f),
                None => types::Value::Error(types::Error::Argument),
            },
        )
    };
    (string, number)
}
