use crate::{
    calculate::{calculate_formula, iterator::convert_iterator_to_result},
    types::{self, XlNum},
};
use std::{fmt::Debug, str::FromStr};

fn calculate_boolean_operator_rhs_boolean<N>(
    l: types::Boolean,
    rh: types::Value<N>,
    f: fn(bool, bool) -> bool,
    allow_error: bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rh {
        types::Value::Boolean(r) => types::Value::Boolean(f(l.into(), r.into()).into()),
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f, allow_error);
                }
                let rhs = cast_value_to_boolean(temp);
                match rhs {
                    types::Value::Boolean(r) => types::Value::Boolean(f(l.into(), r.into()).into()),
                    _ => types::Value::Error(types::Error::Value),
                }
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Blank => types::Value::Boolean(l),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_boolean_operator_rhs_error<N>(rh: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rh {
        types::Value::Boolean(_) => rh,
        types::Value::Error(_) => types::Value::Error(types::Error::Cast),
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_boolean_operator_rhs_iterator<N>(
    rh: types::Value<N>,
    mut lhs_vec: Vec<types::Value<N>>,
    f: fn(bool, bool) -> bool,
    allow_error: bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rh {
        types::Value::Boolean(r) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f, allow_error);
                }
                let lhs = cast_value_to_boolean(temp);
                match lhs {
                    types::Value::Boolean(l) => types::Value::Boolean(f(l.into(), r.into()).into()),
                    _ => types::Value::Error(types::Error::Value),
                }
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }

        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_boolean_operator<N>(
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(bool, bool) -> bool,
    allow_error: bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let lh = cast_value_to_boolean(lhs);
    match lh {
        types::Value::Boolean(l) => {
            calculate_boolean_operator_rhs_boolean(l, cast_value_to_boolean(rhs), f, allow_error)
        }
        types::Value::Error(_) if allow_error => {
            calculate_boolean_operator_rhs_error(cast_value_to_boolean(rhs))
        }
        types::Value::Iterator(lhs_vec) => calculate_boolean_operator_rhs_iterator(
            cast_value_to_boolean(rhs),
            lhs_vec,
            f,
            allow_error,
        ),
        types::Value::Blank => calculate_boolean_operator_rhs_boolean(
            (!allow_error).into(),
            cast_value_to_boolean(rhs),
            f,
            allow_error,
        ),
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_comparison_operator<N>(
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: impl Fn(N, N) -> bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match lhs {
        types::Value::Text(l) => match rhs {
            types::Value::Text(r) => types::Value::Boolean(types::Boolean::from(l.eq(&r))),
            types::Value::Blank => types::Value::Boolean(l.is_empty().into()),
            _ => types::Value::Error(types::Error::Value),
        },
        types::Value::Number(l) => match rhs {
            types::Value::Number(r) => types::Value::Boolean(types::Boolean::from(f(l, r))),
            types::Value::Blank => types::Value::Boolean(types::Boolean::False),
            _ => types::Value::Error(types::Error::Value),
        },
        types::Value::Blank => match rhs {
            types::Value::Number(_) => types::Value::Boolean(types::Boolean::False),
            types::Value::Text(r) => types::Value::Boolean(r.is_empty().into()),
            types::Value::Blank => types::Value::Boolean(types::Boolean::True),
            _ => types::Value::Error(types::Error::Value),
        },
        types::Value::Boolean(_)
        | types::Value::Error(_)
        | types::Value::Iterator(_)
        | types::Value::Date(_) => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_negation<N>(value: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match value {
        types::Value::Boolean(l) => types::Value::Boolean(match l {
            types::Boolean::True => types::Boolean::False,
            types::Boolean::False => types::Boolean::True,
        }),
        types::Value::Error(_) => value,
        types::Value::Text(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => types::Value::Boolean(match l {
                    types::Boolean::True => types::Boolean::False,
                    types::Boolean::False => types::Boolean::True,
                }),
                None => types::Value::Error(types::Error::Cast),
            }
        }
        types::Value::Number(l) => types::Value::Boolean(l.is_zero().into()),
        types::Value::Blank => types::Value::Boolean(types::Boolean::True),
        types::Value::Iterator(_) | types::Value::Date(_) => {
            types::Value::Error(types::Error::Value)
        }
    }
}

pub fn calculate_negate<N>(value: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match value {
        types::Value::Number(n) => types::Value::Number(-n),
        types::Value::Iterator(mut value_vec) => {
            let mut result_vec = Vec::new();
            while let Some(top) = value_vec.pop() {
                result_vec.push(calculate_negate(top));
            }
            types::Value::Iterator(result_vec)
        }
        types::Value::Blank => types::Value::Blank,
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_boolean<N>(
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
    f_bool: fn(bool, bool) -> bool,
    allow_error: bool,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f),
        None => types::Value::Error(types::Error::Argument),
    };
    result = cast_value_to_boolean(result);
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator(result, calculate_formula(top, f), f_bool, allow_error);
    }
    convert_iterator_to_result(result, f_bool, allow_error)
}

pub fn calculate_isblank<N>(value: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match value {
        types::Value::Text(s) => {
            if s.is_empty() {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        types::Value::Blank
        | types::Value::Error(types::Error::Value)
        | types::Value::Error(types::Error::Reference) => {
            types::Value::Boolean(types::Boolean::True)
        }
        _ => types::Value::Boolean(types::Boolean::False),
    }
}

fn cast_text_to_boolean(s: &str) -> Option<types::Boolean> {
    if s.eq_ignore_ascii_case("TRUE") {
        Some(types::Boolean::True)
    } else if s.eq_ignore_ascii_case("FALSE") {
        Some(types::Boolean::False)
    } else {
        None
    }
}

pub fn cast_value_to_boolean<N>(value: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match value {
        types::Value::Boolean(_) | types::Value::Blank | types::Value::Error(_) => value,
        types::Value::Text(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => types::Value::Boolean(l),
                None => types::Value::Error(types::Error::Cast),
            }
        }
        types::Value::Number(l) => {
            if !l.is_zero() {
                types::Value::Boolean(types::Boolean::True)
            } else {
                types::Value::Boolean(types::Boolean::False)
            }
        }
        types::Value::Iterator(mut value_vec) => {
            let mut boolean_vec = Vec::new();
            while let Some(top) = value_vec.pop() {
                let value = cast_value_to_boolean(top);
                boolean_vec.push(value);
            }
            types::Value::Iterator(boolean_vec)
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Cast),
    }
}
