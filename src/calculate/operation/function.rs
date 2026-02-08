use super::{
    boolean::{calculate_boolean, calculate_isblank, calculate_negate, calculate_negation},
    date::{calculate_day, calculate_days, calculate_month, calculate_year},
    number::{
        calculate_abs, calculate_average, calculate_collective_operator,
        calculate_collective_product_operator,
    },
    string::{
        find_position_case_sensitive, search_position_with_wildcards, value_to_string_for_find,
    },
};
use crate::{
    calculate::args::{
        get_binary_function_args, get_find_args, get_number_and_string_values,
        get_ternary_function_args, get_unary_function_arg,
    },
    types::{self, Boolean, Error, XlNum},
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
        types::Function::Find => calculate_find(get_find_args(exp, f)),
        types::Function::Search => calculate_search(get_find_args(exp, f)),
        types::Function::IsError => calculate_iserror(get_unary_function_arg(exp, f)),
    }
}

fn calculate_find<N>(
    (find_text, within_text, start_num): (
        types::Value<N>,
        types::Value<N>,
        types::Value<N>,
    ),
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    if let types::Value::Error(e) = find_text {
        return types::Value::Error(e);
    }
    if let types::Value::Error(e) = within_text {
        return types::Value::Error(e);
    }
    if let types::Value::Error(e) = start_num {
        return types::Value::Error(e);
    }
    let find_s = match value_to_string_for_find(&find_text) {
        Ok(s) => s,
        Err(v) => return v,
    };
    let within_s = match value_to_string_for_find(&within_text) {
        Ok(s) => s,
        Err(v) => return v,
    };
    let start_i64: i64 = match &start_num {
        types::Value::Number(n) => n.as_(),
        _ => return types::Value::Error(Error::Value),
    };
    if start_i64 <= 0 {
        return types::Value::Error(Error::Value);
    }
    let within_len = within_s.chars().count() as i64;
    if start_i64 > within_len {
        return types::Value::Error(Error::Value);
    }
    match find_position_case_sensitive(&find_s, &within_s, start_i64) {
        Some(pos) => types::Value::Number(
            N::from_i64(pos).unwrap_or_else(|| N::from_f64(1.0).unwrap()),
        ),
        None => types::Value::Error(Error::Value),
    }
}

fn calculate_search<N>(
    (find_text, within_text, start_num): (
        types::Value<N>,
        types::Value<N>,
        types::Value<N>,
    ),
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    if let types::Value::Error(e) = find_text {
        return types::Value::Error(e);
    }
    if let types::Value::Error(e) = within_text {
        return types::Value::Error(e);
    }
    if let types::Value::Error(e) = start_num {
        return types::Value::Error(e);
    }
    let find_s = match value_to_string_for_find(&find_text) {
        Ok(s) => s,
        Err(v) => return v,
    };
    let within_s = match value_to_string_for_find(&within_text) {
        Ok(s) => s,
        Err(v) => return v,
    };
    let start_i64: i64 = match &start_num {
        types::Value::Number(n) => n.as_(),
        _ => return types::Value::Error(Error::Value),
    };
    if start_i64 <= 0 {
        return types::Value::Error(Error::Value);
    }
    let within_len = within_s.chars().count() as i64;
    if start_i64 > within_len {
        return types::Value::Error(Error::Value);
    }
    match search_position_with_wildcards(&find_s, &within_s, start_i64) {
        Some(pos) => types::Value::Number(
            N::from_i64(pos).unwrap_or_else(|| N::from_f64(1.0).unwrap()),
        ),
        None => types::Value::Error(Error::Value),
    }
}

fn calculate_iserror<N>(arg: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
{
    let is_err = matches!(arg, types::Value::Error(_));
    types::Value::Boolean(if is_err {
        Boolean::True
    } else {
        Boolean::False
    })
}
