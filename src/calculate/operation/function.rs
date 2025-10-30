use super::{
    boolean::{calculate_boolean, calculate_isblank, calculate_negate, calculate_negation},
    date::{calculate_day, calculate_days, calculate_month, calculate_year},
    number::{
        calculate_abs, calculate_average, calculate_collective_operator,
        calculate_collective_product_operator,
    },
};
use crate::{
    calculate::args::{
        get_binary_function_args, get_number_and_string_values, get_ternary_function_args,
        get_unary_function_arg,
    },
    types::{self, XlNum},
};
use std::{fmt::Debug, str::FromStr};

fn calculate_right<N>(string: types::Value<N>, number: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let trim_length = match number {
        types::Value::Number(x) => x.as_(),
        _ => 0,
    };

    let trimmed_string = match string {
        types::Value::Text(s) => {
            let temp: &'static str = Box::leak(s.into_boxed_str());
            let len = temp.len();
            let start = if trim_length > len { len } else { trim_length };
            &temp[(len - start)..]
        }
        types::Value::Number(s) => {
            let temp: &'static str = Box::leak(s.to_string().into_boxed_str());
            let len = temp.len();
            let start = if trim_length > len { len } else { trim_length };
            &temp[(len - start)..]
        }
        _ => "",
    };
    types::Value::Text(trimmed_string.to_string())
}

fn calculate_left<N>(string: types::Value<N>, number: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let trim_length = match number {
        types::Value::Number(x) => x.as_(),
        _ => 0,
    };

    let trimmed_string = match string {
        types::Value::Text(s) => {
            let temp: &'static str = Box::leak(s.into_boxed_str());
            let len = temp.len();
            let end = if trim_length > len { len } else { trim_length };
            &temp[..end]
        }
        types::Value::Number(s) => {
            let temp: &'static str = Box::leak(s.to_string().into_boxed_str());
            let len = temp.len();
            let end = if trim_length > len { len } else { trim_length };
            &temp[..end]
        }
        _ => "",
    };
    types::Value::Text(trimmed_string.to_string())
}

fn calculate_iff<N>(
    bool_expression: types::Value<N>,
    true_value: types::Value<N>,
    false_value: types::Value<N>,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match bool_expression {
        types::Value::Boolean(bool_value) => {
            if bool_value.into() {
                true_value
            } else {
                false_value
            }
        }
        types::Value::Number(number_value) => {
            if number_value.is_zero() {
                false_value
            } else {
                true_value
            }
        }
        types::Value::Blank => false_value,
        types::Value::Error(_) => bool_expression,
        types::Value::Text(_) => types::Value::Error(types::Error::Value),
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_function<N>(
    func: types::Function,
    exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match func {
        types::Function::Abs => calculate_abs(get_unary_function_arg(exp, f)),
        types::Function::Sum => {
            calculate_collective_operator(types::Value::Number(N::zero()), exp, f, |n1, n2| n1 + n2)
        }
        types::Function::Product => {
            calculate_collective_product_operator(types::Value::Blank, exp, f, |n1, n2| n1 * n2)
        }
        types::Function::Average => {
            calculate_average(types::Value::Number(N::zero()), exp, f, |n1, n2| n1 + n2)
        }
        types::Function::Or => calculate_boolean(exp, f, |n1, n2| n1 || n2, true),
        types::Function::And => calculate_boolean(exp, f, |n1, n2| n1 && n2, false),
        types::Function::Xor => calculate_boolean(exp, f, |n1, n2| n1 ^ n2, true),
        types::Function::Not => calculate_negation(get_unary_function_arg(exp, f)),
        types::Function::Negate => calculate_negate(get_unary_function_arg(exp, f)),
        types::Function::Days => {
            let (end, start) = get_binary_function_args(exp, f);
            calculate_days(start, end)
        }
        types::Function::Right => {
            let (string, number) = get_number_and_string_values(exp, f);
            calculate_right(string, number)
        }
        types::Function::Left => {
            let (string, number) = get_number_and_string_values(exp, f);
            calculate_left(string, number)
        }
        types::Function::Iff => {
            let (bool_expression, true_value, false_value) = get_ternary_function_args(exp, f);
            calculate_iff(bool_expression, true_value, false_value)
        }
        types::Function::IsBlank => calculate_isblank(get_unary_function_arg(exp, f)),
        types::Function::Year => calculate_year(get_unary_function_arg(exp, f)),
        types::Function::Month => calculate_month(get_unary_function_arg(exp, f)),
        types::Function::Day => calculate_day(get_unary_function_arg(exp, f)),
    }
}
