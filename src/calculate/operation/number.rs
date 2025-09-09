use crate::{
    calculate::calculate_formula,
    types::{self, XlNum},
};
use std::{fmt::Debug, str::FromStr};

pub fn calculate_divide_operator<N>(num1: N, num2: N) -> N
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    num1 / num2
}

fn is_float_int<N>(num: N) -> bool
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    num.fract().is_zero()
}

pub fn calculate_power_operator<N>(num1: N, num2: N) -> N
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    if is_float_int(num2) {
        num1.powi(num2.as_())
    } else {
        num1.powf(num2)
    }
}

fn calculate_numeric_operator_rhs_text<N>(
    t: String,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match t.parse::<N>() {
        Ok(nl) => match rhs {
            types::Value::Boolean(_) | types::Value::Error(_) => rhs,
            types::Value::Text(t) => match t.parse::<N>() {
                Ok(nr) => types::Value::Number(f(nl, nr)),
                Err(_) => types::Value::Error(types::Error::Cast),
            },
            types::Value::Number(r) => types::Value::Number(f(nl, r)),
            types::Value::Iterator(_) | types::Value::Date(_) | types::Value::Blank => {
                types::Value::Error(types::Error::Value)
            }
        },
        Err(_) => types::Value::Error(types::Error::Cast),
    }
}

fn calculate_numeric_operator_rhs_number<N>(
    l: N,
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Boolean(_) | types::Value::Error(_) => rhs,
        types::Value::Text(t) => match t.parse::<N>() {
            Ok(nr) => types::Value::Number(f(l, nr)),
            Err(_) => types::Value::Error(types::Error::Cast),
        },
        types::Value::Number(r) => types::Value::Number(f(l, r)),
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                }
                calculate_numeric_operator(lhs, temp, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => types::Value::Number(f(l, N::zero())),
    }
}

fn calculate_numeric_operator_product_rhs_number<N>(
    l: N,
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Boolean(_) | types::Value::Error(_) => rhs,
        types::Value::Text(t) => match t.parse::<N>() {
            Ok(nr) => types::Value::Number(f(l, nr)),
            Err(_) => types::Value::Error(types::Error::Cast),
        },
        types::Value::Number(r) => types::Value::Number(f(l, r)),
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_product_operator(temp, top, f);
                }
                calculate_numeric_product_operator(lhs, temp, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => match lhs {
            types::Value::Blank => types::Value::Blank,
            _ => types::Value::Number(l),
        },
    }
}

fn calculate_numeric_operator_rhs_iterator<N>(
    mut lhs_vec: Vec<types::Value<N>>,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Number(_) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                }
                calculate_numeric_operator(temp, rhs, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Iterator(mut rhs_vec) => {
            let mut result_vec = Vec::new();
            loop {
                match (lhs_vec.pop(), rhs_vec.pop()) {
                    (Some(x), Some(y)) => {
                        result_vec.push(calculate_numeric_operator(x, y, f));
                    }
                    (Some(_), None) | (None, Some(_)) => {
                        result_vec.push(types::Value::Error(types::Error::Argument))
                    }
                    (None, None) => break,
                };
            }
            types::Value::Iterator(result_vec)
        }
        _ => types::Value::Error(types::Error::Value),
    }
}

pub fn calculate_numeric_operator<N>(
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    //println!("{:?}::{:?}", lhs, rhs);
    match lhs {
        types::Value::Boolean(_) | types::Value::Error(_) => lhs,
        types::Value::Text(t) => calculate_numeric_operator_rhs_text(t, rhs, f),
        types::Value::Number(l) => calculate_numeric_operator_rhs_number(l, lhs, rhs, f),
        types::Value::Iterator(lhs_vec) => calculate_numeric_operator_rhs_iterator(lhs_vec, rhs, f),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => calculate_numeric_operator_rhs_number(N::zero(), lhs, rhs, f),
    }
}

fn calculate_numeric_product_operator<N>(
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    //println!("{:?}::{:?}", lhs, rhs);
    match lhs {
        types::Value::Boolean(_) | types::Value::Error(_) => lhs,
        types::Value::Text(t) => calculate_numeric_operator_rhs_text(t, rhs, f),
        types::Value::Number(l) => calculate_numeric_operator_product_rhs_number(l, lhs, rhs, f),
        types::Value::Iterator(lhs_vec) => calculate_numeric_operator_rhs_iterator(lhs_vec, rhs, f),
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => calculate_numeric_operator_product_rhs_number(N::one(), lhs, rhs, f),
    }
}

fn calculate_average_operator_rhs_number<N>(
    element_count: &mut i64,
    l: N,
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Boolean(_) | types::Value::Error(_) => rhs,
        types::Value::Text(t) => match t.parse::<N>() {
            Ok(nr) => types::Value::Number(f(l, nr)),
            Err(_) => types::Value::Error(types::Error::Cast),
        },
        types::Value::Number(r) => types::Value::Number(f(l, r)),
        types::Value::Iterator(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                if temp == types::Value::Blank {
                    *element_count -= 1;
                }
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_operator(temp, top.clone(), f);
                    match top {
                        types::Value::Blank => (),
                        _ => *element_count += 1,
                    };
                }
                calculate_numeric_operator(lhs, temp, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => {
            *element_count -= 1;
            types::Value::Number(f(l, N::zero()))
        }
    }
}

fn calculate_average_operator_rhs_iterator<N>(
    element_count: &mut i64,
    mut lhs_vec: Vec<types::Value<N>>,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match rhs {
        types::Value::Number(_) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f);
                    *element_count += 1;
                }
                calculate_numeric_operator(temp, rhs, f)
            } else {
                types::Value::Error(types::Error::Argument)
            }
        }
        _ => types::Value::Error(types::Error::Value),
    }
}

fn calculate_average_operator<N>(
    element_count: &mut i64,
    lhs: types::Value<N>,
    rhs: types::Value<N>,
    f: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match lhs {
        types::Value::Boolean(_) | types::Value::Error(_) => lhs,
        types::Value::Text(t) => calculate_numeric_operator_rhs_text(t, rhs, f),
        types::Value::Number(l) => {
            calculate_average_operator_rhs_number(element_count, l, lhs, rhs, f)
        }
        types::Value::Iterator(lhs_vec) => {
            calculate_average_operator_rhs_iterator(element_count, lhs_vec, rhs, f)
        }
        types::Value::Date(_) => types::Value::Error(types::Error::Value),
        types::Value::Blank => {
            *element_count -= 1;
            calculate_average_operator_rhs_number(element_count, N::zero(), lhs, rhs, f)
        }
    }
}

pub fn calculate_abs<N>(value: types::Value<N>) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match value {
        types::Value::Boolean(_) | types::Value::Error(_) | types::Value::Text(_) => value,
        types::Value::Number(l) => types::Value::Number(l.abs()),
        types::Value::Iterator(_) | types::Value::Date(_) => {
            types::Value::Error(types::Error::Value)
        }
        types::Value::Blank => types::Value::Number(N::zero()),
    }
}

pub fn calculate_average<N>(
    mut collective_value: types::Value<N>,
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
    f_collective: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut element_count = 0;
    while let Some(top) = exp.values.pop() {
        element_count += 1;
        collective_value = calculate_average_operator(
            &mut element_count,
            collective_value,
            calculate_formula(top, f),
            f_collective,
        );
    }
    if element_count == 0 {
        types::Value::Error(types::Error::Div0)
    } else {
        calculate_numeric_operator(
            collective_value,
            types::Value::Number(N::from_i64(element_count).unwrap()),
            calculate_divide_operator,
        )
    }
}

pub fn calculate_collective_operator<N>(
    mut collective_value: types::Value<N>,
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
    f_collective: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    while let Some(top) = exp.values.pop() {
        collective_value =
            calculate_numeric_operator(collective_value, calculate_formula(top, f), f_collective);
    }
    collective_value
}

pub fn calculate_collective_product_operator<N>(
    mut collective_value: types::Value<N>,
    mut exp: types::Expression<N>,
    f: Option<&impl Fn(String) -> types::Value<N>>,
    f_collective: fn(N, N) -> N,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    while let Some(top) = exp.values.pop() {
        collective_value = calculate_numeric_product_operator(
            collective_value,
            calculate_formula(top, f),
            f_collective,
        );
    }
    match collective_value {
        types::Value::Blank => types::Value::Number(N::zero()),
        _ => collective_value,
    }
}
