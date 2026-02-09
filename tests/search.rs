mod common;

use std::{fmt::Debug, str::FromStr};
use xlformula_engine::types::XlNum;

fn eval<N>(s: &str) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    common::evaluate_formula_string::<N>(s)
}

#[test]
fn search_case_insensitive() {
    assert_eq!(eval::<f64>(r#"=SEARCH("m","MiriamMcGovern")"#), "1");
    assert_eq!(eval::<f64>(r#"=SEARCH("M","MiriamMcGovern")"#), "1");
    assert_eq!(eval::<f64>(r#"=SEARCH("gloves","Gloves (Youth)")"#), "1");
    assert_eq!(eval::<f64>(r#"=SEARCH("M","abc")"#), "#VALUE!");
}

#[test]
fn search_wildcard_question() {
    assert_eq!(eval::<f64>(r#"=SEARCH("sm?th","smith")"#), "1");
    assert_eq!(eval::<f64>(r#"=SEARCH("sm?th","smyth")"#), "1");
}

#[test]
fn search_wildcard_star() {
    // "*east" matches "east" in "Northeast"; "east" starts at 1-based position 6
    assert_eq!(eval::<f64>(r#"=SEARCH("*east","Northeast")"#), "6");
}

#[test]
fn search_literal_tilde() {
    assert_eq!(eval::<f64>(r#"=SEARCH("~?","a?b")"#), "2");
}

#[test]
fn search_empty_find_text() {
    assert_eq!(eval::<f64>(r#"=SEARCH("","abc")"#), "1");
    assert_eq!(eval::<f64>(r#"=SEARCH("","abc",2)"#), "2");
    // empty find_text with start past length returns #VALUE!
    assert_eq!(eval::<f64>(r#"=SEARCH("","abc",4)"#), "#VALUE!");
}

#[test]
fn search_not_found() {
    assert_eq!(eval::<f64>(r#"=SEARCH("x","abc")"#), "#VALUE!");
}

#[test]
fn search_start_num_bounds() {
    assert_eq!(eval::<f64>(r#"=SEARCH("a","abc",0)"#), "#VALUE!");
    assert_eq!(eval::<f64>(r#"=SEARCH("a","abc",-1)"#), "#VALUE!");
    assert_eq!(eval::<f64>(r#"=SEARCH("a","abc",4)"#), "#VALUE!");
}

#[test]
fn search_f32() {
    assert_eq!(eval::<f32>(r#"=SEARCH("m","Miriam")"#), "1");
}
