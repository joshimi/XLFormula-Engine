pub mod boolean;
pub mod date;
pub mod function;
pub mod number;
pub mod string;

use self::{
    boolean::calculate_comparison_operator,
    date::{add_days_to_date, compare_dates, subtract_days_from_date},
    function::calculate_function,
    number::{calculate_divide_operator, calculate_numeric_operator, calculate_power_operator},
    string::{calculate_concat_operator, calculate_string_operator, compare_strings},
};
use super::args::get_binary_function_args;
use crate::types::{self, XlNum};
use std::{fmt::Debug, str::FromStr};

pub fn calculate_operation<N>(
    exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match exp.op {
        types::Operator::Plus => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match lhs {
                types::Value::Date(d) => add_days_to_date(d, rhs),
                _ => calculate_numeric_operator(lhs, rhs, |n1, n2| n1 + n2),
            }
        }

        types::Operator::Minus => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match lhs {
                types::Value::Date(d) => subtract_days_from_date(d, rhs),
                _ => calculate_numeric_operator(lhs, rhs, |n1, n2| n1 - n2),
            }
        }

        types::Operator::Multiply => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            calculate_numeric_operator(lhs, rhs, |n1, n2| n1 * n2)
        }
        types::Operator::Divide => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match rhs {
                types::Value::Number(n) if n.is_zero() => types::Value::Error(types::Error::Div0),
                _ => calculate_numeric_operator(lhs, rhs, calculate_divide_operator),
            }
        }
        types::Operator::Power => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            calculate_numeric_operator(lhs, rhs, calculate_power_operator)
        }
        types::Operator::Concat => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            calculate_string_operator(lhs, rhs, calculate_concat_operator)
        }
        types::Operator::Equal => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match (lhs.clone(), rhs.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 == d2)
                }
                (types::Value::Text(l), types::Value::Text(r)) => {
                    compare_strings(l, r, |s1, s2| s1 == s2)
                }
                _ => calculate_comparison_operator(lhs, rhs, |n1, n2| (n1 - n2).abs().is_zero()),
            }
        }
        types::Operator::NotEqual => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match (lhs.clone(), rhs.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 != d2)
                }
                (types::Value::Text(l), types::Value::Text(r)) => {
                    compare_strings(l, r, |s1, s2| s1 != s2)
                }
                _ => calculate_comparison_operator(lhs, rhs, |n1, n2| (n1 - n2).abs() > N::zero()),
            }
        }
        types::Operator::Greater => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match (lhs.clone(), rhs.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 > d2)
                }
                _ => calculate_comparison_operator(lhs, rhs, |n1, n2| n1 > n2),
            }
        }
        types::Operator::Less => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match (lhs.clone(), rhs.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 < d2)
                }
                _ => calculate_comparison_operator(lhs, rhs, |n1, n2| n1 < n2),
            }
        }
        types::Operator::GreaterOrEqual => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match (lhs.clone(), rhs.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 >= d2)
                }
                _ => calculate_comparison_operator(lhs, rhs, |n1, n2| n1 >= n2),
            }
        }
        types::Operator::LessOrEqual => {
            let (lhs, rhs) = get_binary_function_args(exp, f);
            match (lhs.clone(), rhs.clone()) {
                (types::Value::Date(l), types::Value::Date(r)) => {
                    compare_dates(l, r, |d1, d2| d1 <= d2)
                }
                _ => calculate_comparison_operator(lhs, rhs, |n1, n2| n1 <= n2),
            }
        }
        types::Operator::Function(func) => calculate_function(func, exp, f),
    }
}
