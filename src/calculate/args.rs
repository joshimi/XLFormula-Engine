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
