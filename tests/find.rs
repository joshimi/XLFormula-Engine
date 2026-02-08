mod common;

use std::fmt::Debug;
use std::str::FromStr;
use xlformula_engine::types::XlNum;

fn eval<N>(s: &str) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    common::evaluate_formula_string::<N>(s)
}

#[test]
fn find_basic() {
    assert_eq!(eval::<f64>(r#"=FIND("a","abc")"#), "1");
    assert_eq!(eval::<f64>(r#"=FIND("b","abc")"#), "2");
    assert_eq!(eval::<f64>(r#"=FIND("c","abc")"#), "3");
}

#[test]
fn find_not_found() {
    assert_eq!(eval::<f64>(r#"=FIND("x","abc")"#), "#VALUE!");
}

#[test]
fn find_from_position() {
    assert_eq!(eval::<f64>(r#"=FIND("a","abc",2)"#), "#VALUE!");
    assert_eq!(eval::<f64>(r#"=FIND("b","abc",2)"#), "2");
    assert_eq!(eval::<f64>(r#"=FIND("Y","AYF0093.YoungMensApparel",8)"#), "9");
}

#[test]
fn find_empty_find_text() {
    assert_eq!(eval::<f64>(r#"=FIND("","abc")"#), "1");
    assert_eq!(eval::<f64>(r#"=FIND("","abc",2)"#), "2");
    // empty find_text with start past length returns #VALUE!
    assert_eq!(eval::<f64>(r#"=FIND("","abc",4)"#), "#VALUE!");
}

#[test]
fn find_case_sensitive() {
    assert_eq!(eval::<f64>(r#"=FIND("A","abc")"#), "#VALUE!");
    // Second "M" in "MiriamMcGovern" starting from position 3 is at 1-based position 7
    assert_eq!(eval::<f64>(r#"=FIND("M","MiriamMcGovern",3)"#), "7");
}

#[test]
fn find_start_num_bounds() {
    assert_eq!(eval::<f64>(r#"=FIND("a","abc",0)"#), "#VALUE!");
    assert_eq!(eval::<f64>(r#"=FIND("a","abc",-1)"#), "#VALUE!");
    assert_eq!(eval::<f64>(r#"=FIND("a","abc",4)"#), "#VALUE!");
}

#[test]
fn find_start_num_truncate() {
    assert_eq!(eval::<f64>(r#"=FIND("b","abc",2.9)"#), "2");
}

#[test]
fn find_coercion() {
    assert_eq!(eval::<f64>(r#"=FIND(1,"121")"#), "1");
    assert_eq!(eval::<f64>(r#"=FIND(TRUE,"TRUEx")"#), "1");
    assert_eq!(eval::<f64>(r#"=FIND("1","121",1)"#), "1");
}

#[test]
fn find_character_based() {
    assert_eq!(eval::<f64>(r#"=FIND("ö","föö")"#), "2");
}

#[test]
fn find_error_propagation() {
    assert_eq!(eval::<f64>(r#"=FIND(1/0,"abc")"#), "#DIV/0!");
    assert_eq!(eval::<f64>(r#"=FIND("a",1/0)"#), "#DIV/0!");
}

#[test]
fn find_f32() {
    assert_eq!(eval::<f32>(r#"=FIND("a","abc")"#), "1");
    assert_eq!(eval::<f32>(r#"=FIND("x","abc")"#), "#VALUE!");
}
