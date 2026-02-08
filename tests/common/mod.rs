use std::fmt::Debug;
use std::str::FromStr;
use xlformula_engine::{
    calculate,
    parse_formula,
    types::{self, XlNum},
    NoCustomFunction, NoReference,
};

/// Evaluate formula string and return the result as string (e.g. "1", "#VALUE!", "TRUE").
pub fn evaluate_formula_string<N>(s: &str) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    let result = calculate::calculate_formula(formula, None::<NoReference<N>>);
    calculate::result_to_string(result)
}

/// Evaluate formula and return the raw Value (for checking Error variant etc.).
#[allow(dead_code)]
pub fn evaluate_formula_value<N>(s: &str) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    calculate::calculate_formula(formula, None::<NoReference<N>>)
}
