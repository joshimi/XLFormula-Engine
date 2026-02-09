mod common;

use std::{fmt::Debug, str::FromStr};
use xlformula_engine::types::{self, XlNum};

fn eval<N>(s: &str) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    common::evaluate_formula_string::<N>(s)
}

fn value<N>(s: &str) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    common::evaluate_formula_value::<N>(s)
}

#[test]
fn iserror_true_for_errors() {
    assert_eq!(eval::<f64>(r#"=ISERROR(1/0)"#), "TRUE");
    assert_eq!(eval::<f64>(r#"=ISERROR(FIND("x","abc"))"#), "TRUE");
}

#[test]
fn iserror_false_for_non_errors() {
    assert_eq!(eval::<f64>(r#"=ISERROR(1)"#), "FALSE");
    assert_eq!(eval::<f64>(r#"=ISERROR("")"#), "FALSE");
    assert_eq!(eval::<f64>(r#"=ISERROR(TRUE)"#), "FALSE");
    assert_eq!(eval::<f64>(r#"=ISERROR(FIND("a","abc"))"#), "FALSE");
}

#[test]
fn iserror_detects_error_value() {
    let v = value::<f64>(r#"=ISERROR(1/0)"#);
    assert!(matches!(v, types::Value::Boolean(types::Boolean::True)));
    let v = value::<f64>(r#"=ISERROR(42)"#);
    assert!(matches!(v, types::Value::Boolean(types::Boolean::False)));
}

#[test]
fn iserror_f32() {
    assert_eq!(eval::<f32>(r#"=ISERROR(1/0)"#), "TRUE");
    assert_eq!(eval::<f32>(r#"=ISERROR(1)"#), "FALSE");
}
