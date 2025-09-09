use chrono::{DateTime, Datelike, Duration, FixedOffset};
use std::{fmt::Debug, str::FromStr};
use xlformula_engine::{
    calculate, parse_formula,
    types::{self, XlNum},
    NoCustomFunction, NoReference,
};

fn evaluate_formula_number<N>(s: &str) -> N
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    let result = calculate::calculate_formula(formula, None::<NoReference<N>>);
    calculate::result_to_string(result).parse::<N>().unwrap()
}

fn evaluate_formula_string<N>(s: &str) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    let result = calculate::calculate_formula(formula, None::<NoReference<N>>);
    calculate::result_to_string(result)
}

fn evaluate_formula_string_with_reference<N>(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    let result = calculate::calculate_formula(formula, f);
    calculate::result_to_string(result)
}

fn evaluate_formula_number_with_reference<N>(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> N
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    let result = calculate::calculate_formula(formula, f);
    calculate::result_to_string(result).parse::<N>().unwrap()
}

fn evaluate_formula_number_with_reference_no_conversion<N>(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> types::Value<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    calculate::calculate_formula(formula, f)
    //calculate::result_to_string(result).parse::<XlNum>().unwrap()
}

fn evaluate_formula_boolean_with_reference<N>(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    let result = calculate::calculate_formula(formula, f);
    calculate::result_to_string(result) //.parse::<XlNum>().unwrap()
}

fn evaluate_formula_date_with_reference<N>(
    s: &str,
    f: Option<&impl Fn(String) -> types::Value<N>>,
) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, None::<NoCustomFunction<N>>);
    let result = calculate::calculate_formula(formula, f);
    calculate::result_to_string(result)
}

fn evaluate_formula_number_with_custom_function<N>(
    s: &str,
    custom_function: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
    //reference: Option<&impl Fn(String) -> types::Value>,
) -> N
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, custom_function);
    let result = calculate::calculate_formula(formula, None::<NoReference<N>>);
    calculate::result_to_string(result).parse::<N>().unwrap()
}

fn evaluate_formula_string_with_custom_function<N>(
    s: &str,
    custom_function: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
    //reference: Option<&impl Fn(String) -> types::Value>,
) -> String
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, custom_function);
    let result = calculate::calculate_formula(formula, None::<NoReference<N>>);
    calculate::result_to_string(result)
}

fn _evaluate_formula_number_with_custom_function_and_reference<N>(
    s: &str,
    custom_function: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
    reference: Option<&impl Fn(String) -> types::Value<N>>,
) -> N
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let formula = parse_formula::parse_string_to_formula(s, custom_function);
    let result = calculate::calculate_formula(formula, reference);
    calculate::result_to_string(result).parse::<N>().unwrap()
}

macro_rules! test_all_types {
    ($func:ident, $formula:expr, $result:expr) => {
        assert_eq!($func::<f32>($formula), $result);
        assert_eq!($func::<f64>($formula), $result);
    };
}

macro_rules! test_all_types_with_data {
    ($func:ident, $formula:expr, $data:expr, $result:expr) => {
        assert_eq!($func::<f32>($formula, $data), $result);
        assert_eq!($func::<f64>($formula, $data), $result);
    };
}

/////////////////// Simple math operators with floats and integer ///////////////////
#[test]
fn it_evaluate_add_operator_simple_addition() {
    test_all_types!(evaluate_formula_number, "=1+2", 3.0);
}

#[test]
fn it_evaluate_add_operator_spaces_between_operators() {
    test_all_types!(evaluate_formula_number, "=1 +  2", 3.0);
}

#[test]
fn it_evaluate_add_operator_spaces_before_number() {
    test_all_types!(evaluate_formula_number, "=  1+2", 3.0);
}

#[test]
fn it_evaluate_add_operator_with_large_numbers() {
    test_all_types!(
        evaluate_formula_number,
        "=1234567890 + 1234567890",
        2469135780.0
    );
}

#[test]
fn it_evaluate_add_operator_with_negative_numbers() {
    test_all_types!(evaluate_formula_number, "=-1 + -2", -3.0);
}

#[test]
fn it_evaluate_minus_operator1() {
    test_all_types!(evaluate_formula_number, "=123 - 23", 100.0);
}

#[test]
fn it_evaluate_minus_operator_with_negative_numbers() {
    test_all_types!(evaluate_formula_number, "=-12--6", -6.0);
}

#[test]
fn it_evaluate_multiply_operator() {
    test_all_types!(evaluate_formula_number, "=3 * 2", 6.0);
}

#[test]
fn it_evaluate_divide_operator() {
    test_all_types!(evaluate_formula_number, "=6 / 3", 2.0);
}

#[test]
fn it_evaluate_divide_operator_divsion_by_zero() {
    test_all_types!(evaluate_formula_string, "=6 / 0", "#DIV/0!");
}

#[test]
fn it_evaluate_negative() {
    test_all_types!(evaluate_formula_number, "=-1 * -5", 5.0);
}

#[test]
fn it_evaluate_power_int() {
    test_all_types!(evaluate_formula_number, "=2^3", 8.0);
}

#[test]
fn it_evaluate_float() {
    test_all_types!(evaluate_formula_number, "=1.2+0.5", 1.7);
}

#[test]
fn it_evaluate_negative_float() {
    // left: `-0.70000005`, right: `-0.7`'
    assert_approx_eq::assert_approx_eq!(evaluate_formula_number::<f32>("=-1.2+0.5"), -0.7);
    assert_eq!(evaluate_formula_number::<f64>("=-1.2+0.5"), -0.7);
}

#[test]
fn it_evaluate_power_float() {
    test_all_types!(evaluate_formula_number, "=4^0.5", 2.0);
}

#[test]
fn it_evaluate_multiple_operations() {
    test_all_types!(evaluate_formula_number, "=1+2+3", 6.0);
}

#[test]
fn it_evaluate_multiple_operations2() {
    test_all_types!(evaluate_formula_number, "=1+2-3", 0.0);
}
#[test]
fn it_evaluate_multiple_operations_in_right_order() {
    test_all_types!(evaluate_formula_number, "=1+2*3", 7.0);
}

#[test]
fn it_evaluate_multiple_operations_in_right_order2() {
    test_all_types!(evaluate_formula_number, "=1+3/3", 2.0);
}

#[test]
fn it_evaluate_multiple_operations_with_errors() {
    test_all_types!(evaluate_formula_string, "=1+3/0", "#DIV/0!");
}

#[test]
fn it_evaluate_parens() {
    test_all_types!(evaluate_formula_number, "=(1+2)", 3.0);
}

#[test]
fn it_evaluate_multiple_parens() {
    test_all_types!(evaluate_formula_number, "=(1+2)+(3+4)", 10.0);
}

#[test]
fn it_evaluate_nested_parens() {
    test_all_types!(evaluate_formula_number, "=(1*(2+3))*2", 10.0);
}

/////////////////// Strings ///////////////////
#[test]
fn it_evaluate_strings() {
    test_all_types!(evaluate_formula_string, "=\"Hello!  \"", "Hello!  ");
}

#[test]
fn it_evaluate_strings_in_numeric_operator() {
    test_all_types!(evaluate_formula_string, "=\"Hello\"+1", "#CAST!");
}

#[test]
fn it_evaluate_strings_in_numeric_operator2() {
    test_all_types!(evaluate_formula_string, "=1 + \"Hello\"", "#CAST!");
}

#[test]
fn it_evaluate_concat_operator1() {
    test_all_types!(
        evaluate_formula_string,
        "=\"Hello\" & \"World!\"",
        "HelloWorld!"
    );
}

#[test]
fn it_evaluate_concat_operator3() {
    test_all_types!(
        evaluate_formula_string,
        "=\"Hello \" & \" World!\"",
        "Hello  World!"
    );
}

#[test]
fn it_evaluate_concat_operator_with_casting() {
    test_all_types!(evaluate_formula_string, "=\"Hello\"&1", "Hello1");
}

#[test]
fn it_evaluate_concat_operator_with_casting2() {
    test_all_types!(evaluate_formula_string, "=\"Hello \"&1.2", "Hello 1.2");
}

#[test]
fn it_evaluate_concat_operator_with_numberic() {
    test_all_types!(evaluate_formula_string, "=1   &  2", "12");
}

#[test]
fn it_evaluate_strings_with_quoted_quotes1() {
    test_all_types!(
        evaluate_formula_string,
        "=\"Hello 'World'\"",
        "Hello 'World'"
    );
}

#[test]
fn it_evaluate_strings_with_quoted_quotes() {
    test_all_types!(
        evaluate_formula_string,
        "=\"Hello \"\"World\"\"\"",
        "Hello \"World\""
    );
}

#[test]
fn it_evaluate_strings_with_single_quotes() {
    test_all_types!(
        evaluate_formula_string,
        "=\"Hello \"&'World'",
        "Hello World"
    );
}

#[test]
fn it_evaluate_strings_with_quotes() {
    test_all_types!(
        evaluate_formula_string,
        "='Hello \"World\"'",
        "Hello \"World\""
    );
}

#[test]
fn it_evaluate_strings_with_quotes2() {
    test_all_types!(
        evaluate_formula_string,
        "='Hello'", // '& 'World'
        "Hello"
    );
}

/////////////////// Constants  ///////////////////
#[test]
fn it_evaluate_constant_number() {
    test_all_types!(evaluate_formula_number, "1", 1.0);
}

#[test]
fn it_evaluate_constant_number_float() {
    test_all_types!(evaluate_formula_number, "1.2", 1.2);
}

#[test]
fn it_evaluate_constant_text() {
    test_all_types!(evaluate_formula_string, "Hello World", "Hello World");
}

#[test]
fn it_evaluate_constant_text_with_quotes() {
    test_all_types!(evaluate_formula_string, "Hello \"World'", "Hello \"World'");
}

#[test]
fn it_evaluate_constant_starting_with_equal() {
    test_all_types!(evaluate_formula_string, "'=", "=");
    test_all_types!(evaluate_formula_string, "'=hello", "=hello");
}

/////////////////// Formulas ///////////////////
#[test]
fn it_support_basic_math_function() {
    test_all_types!(evaluate_formula_number, "=ABS(-1)", 1.0);
}

#[test]
fn it_support_basic_math_function_with_nested_formulas() {
    test_all_types!(evaluate_formula_number, "=ABS(-1-4)", 5.0);
}

#[test]
fn it_support_basic_math_function_with_nested_functions() {
    test_all_types!(evaluate_formula_number, "=ABS(ABS(-1))", 1.0);
}

#[test]
fn it_evaluate_functions_sum() {
    test_all_types!(
        evaluate_formula_number,
        "=SUM(1*1, ABS(2), ABS(2+1), 4)",
        10.0
    );
    test_all_types!(evaluate_formula_number, "=SUM(1, 2, , 3)", 6.0);
    test_all_types!(evaluate_formula_number, "=SUM( 1 , 2,,3,)", 6.0);
    test_all_types!(evaluate_formula_number, "=SUM( 1 , 2,,,3,)", 6.0);
    test_all_types!(evaluate_formula_number, "=SUM(,)", 0.0);
}

#[test]
fn it_evaluate_functions_avg() {
    test_all_types!(evaluate_formula_number, "=AVERAGE(1,2)", 1.5);
    test_all_types!(evaluate_formula_number, "=AVERAGE({1,2,3})", 2.0);
    test_all_types!(evaluate_formula_number, "=AVERAGE({1,2,3},1,2,3)", 2.0);
    test_all_types!(evaluate_formula_number, "=AVERAGE(3,1,2,3)", 2.25);
    test_all_types!(evaluate_formula_number, "=AVERAGE(1,2,3,4,5,1,2,3)", 2.625);
    test_all_types!(
        evaluate_formula_number,
        "=AVERAGE({1,2,3,4,5},1,2,3)",
        2.625
    );
    test_all_types!(
        evaluate_formula_number,
        "=AVERAGE(AVERAGE({1,2,3,4,5}),1,2,3)",
        2.25
    );
    test_all_types!(evaluate_formula_number, "=AVERAGE({100,200})", 150.0);
    test_all_types!(evaluate_formula_number, "=AVERAGE( 1 , 2,,3)", 1.5);
    test_all_types!(evaluate_formula_number, "=AVERAGE( 1 , )", 0.5);
    test_all_types!(evaluate_formula_number, "=AVERAGE(,)", 0.0);
    test_all_types!(evaluate_formula_number, "=AVERAGE(1,,2,3,)", 1.2);
    // test_all_types!(evaluate_formula_number, "=AVERAGE({{100,200}})", 150.0);
}

#[test]
fn it_evaluate_functions_product() {
    test_all_types!(evaluate_formula_number, "=PRODUCT(ABS(1),2*1, 3,4*1)", 24.0);
    test_all_types!(evaluate_formula_number, "=PRODUCT( 1 , 2,,3)", 6.0);
    test_all_types!(evaluate_formula_number, "=PRODUCT(,)", 0.0);
}

#[test]
fn it_evaluate_operators_with_casting() {
    test_all_types!(evaluate_formula_number, "=\"1\"+2+\"3\"", 6.0);
}

#[test]
fn it_evaluate_functions_with_casting() {
    test_all_types!(evaluate_formula_number, "=SUM(1,2,\"3\")", 6.0);
}

/////////////////////// Parse error //////////////////////////////////
#[test]
fn it_evaluate_wrong_parens1() {
    test_all_types!(evaluate_formula_string, "=(2+3", "#PARSE!");
    test_all_types!(evaluate_formula_string, "=\"Hello World", "#PARSE!");
    test_all_types!(evaluate_formula_string, "=Hello World", "#PARSE!");
}

//////////////////////////// Boolean //////////////////////////////////
#[test]
fn it_evaluate_comparison_operators() {
    test_all_types!(evaluate_formula_string, "=1*1=1/1", "TRUE");
    test_all_types!(evaluate_formula_string, "=1^1<>1", "FALSE");
    test_all_types!(evaluate_formula_string, "=1*2>1", "TRUE");
    test_all_types!(evaluate_formula_string, "=1*1/1+2<1^1", "FALSE");
    test_all_types!(evaluate_formula_string, "=2>=1", "TRUE");
    test_all_types!(evaluate_formula_string, "=11<=3", "FALSE");
    test_all_types!(evaluate_formula_string, "=\"Joshu\"=\"Joshu\"", "TRUE");
    test_all_types!(evaluate_formula_string, "=\"Joshu\"<>\"NotJoshu\"", "TRUE");
}

#[test]
fn it_evaluate_boolean_or() {
    test_all_types!(evaluate_formula_string, "=OR(1>1,1<>1)", "FALSE");
    test_all_types!(evaluate_formula_string, "=OR(1=1,2<=4)", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR(\"True\")", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR(True)", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR(1)", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR(\"test\")", "#CAST!");
    test_all_types!(
        evaluate_formula_string,
        "=OR(\"false\",\"FALSE\", 1, FALSE)",
        "TRUE"
    );
    test_all_types!(
        evaluate_formula_string,
        "=OR(\"True\",1,\"test\",  true) ",
        "TRUE"
    );
    test_all_types!(evaluate_formula_string, "=OR(1, )", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR(1,,, )", "TRUE");
}

#[test]
fn it_evaluate_boolean_and() {
    test_all_types!(evaluate_formula_string, "=AND(1>1,1=1)", "FALSE");
    test_all_types!(evaluate_formula_string, "=AND(1=1,2<=4)", "TRUE");
    test_all_types!(evaluate_formula_string, "=AND(\"true\", 0)", "FALSE");
    test_all_types!(evaluate_formula_string, "=AND(\"True\")", "TRUE");
    test_all_types!(evaluate_formula_string, "=AND(false)", "FALSE");
    test_all_types!(evaluate_formula_string, "=AND(1)", "TRUE");
    test_all_types!(
        evaluate_formula_string,
        "=AND(\"test\", \"test\")",
        "#VALUE!"
    );
    test_all_types!(
        evaluate_formula_string,
        "=AND(\"test\",\"True\", 1, true) ",
        "#VALUE!"
    );
    test_all_types!(
        evaluate_formula_string,
        "=AND(\"True\",\"test\", 1, true) ",
        "#VALUE!"
    );
    test_all_types!(
        evaluate_formula_string,
        "=AND(\"True\", 1, true, \"test\")",
        "#VALUE!"
    );
    test_all_types!(evaluate_formula_string, "=AND(1, )", "FALSE");
    test_all_types!(evaluate_formula_string, "=AND(1,,,1)", "FALSE");
}

#[test]
fn it_evaluate_boolean_xor() {
    test_all_types!(evaluate_formula_string, "=XOR(2=2,1=1)", "FALSE");
    test_all_types!(evaluate_formula_string, "=XOR(1=1,2>4)", "TRUE");
    test_all_types!(evaluate_formula_string, "=XOR(\"True\")", "TRUE");
    test_all_types!(evaluate_formula_string, "=XOR(False)", "FALSE");
    test_all_types!(evaluate_formula_string, "=XOR(1)", "TRUE");
    test_all_types!(evaluate_formula_string, "=XOR(1=1,\"test\")", "TRUE");
    test_all_types!(
        evaluate_formula_string,
        "=XOR(TRUE, TRUE, TRUE, TRUE)",
        "FALSE"
    );
    test_all_types!(evaluate_formula_string, "=XOR(TRUE, TRUE)", "FALSE");
    test_all_types!(
        evaluate_formula_string,
        "=XOR(false, FALSE, \"false\", false)",
        "FALSE"
    );
    test_all_types!(evaluate_formula_string, "=XOR(true)", "TRUE");
    test_all_types!(evaluate_formula_string, "=XOR(false)", "FALSE");
    test_all_types!(evaluate_formula_string, "=XOR(1, )", "TRUE");
    test_all_types!(evaluate_formula_string, "=XOR(1,, )", "TRUE");
    test_all_types!(evaluate_formula_string, "=XOR(1,,1 )", "FALSE");
    test_all_types!(evaluate_formula_string, "=XOR(1,,1, )", "FALSE");
}

#[test]
fn it_evaluate_boolean_not() {
    test_all_types!(evaluate_formula_string, "=NOT(11<=3)", "TRUE");
    test_all_types!(evaluate_formula_string, "=NOT(1=1)", "FALSE");
    test_all_types!(evaluate_formula_string, "=NOT(True)", "FALSE");
    test_all_types!(evaluate_formula_string, "=NOT(\"false\")", "TRUE");
    test_all_types!(evaluate_formula_string, "=NOT(\"test\")", "#CAST!");
    test_all_types!(evaluate_formula_string, "=NOT(0)", "TRUE");
    test_all_types!(evaluate_formula_string, "=not(11<=3)", "TRUE");
    test_all_types!(evaluate_formula_string, "=Not(11<=3)", "TRUE");
    test_all_types!(evaluate_formula_string, "=nOT(11<=3)", "TRUE");
}

//////////////////////////// References //////////////////////////////////
#[test]
fn it_evaluate_references() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Number(N::from_f32(1.0).unwrap()),
            "B" => types::Value::Number(N::from_f32(2.0).unwrap()),
            "C" => types::Value::Number(N::from_f32(3.0).unwrap()),
            "fix_rate" => types::Value::Number(N::from_f32(10.0).unwrap()),
            "input." => types::Value::Number(N::from_f32(2.0).unwrap()),
            "D" => types::Value::Number(N::from_f32(1.0).unwrap()),
            "F" => types::Value::Text("=D+1".to_string()),
            "G" => types::Value::Text("=F+1+D+1".to_string()),
            "Test" => types::Value::Iterator(vec![
                types::Value::Number(N::from_f32(100.0).unwrap()),
                types::Value::Number(N::from_f32(200.0).unwrap()),
                types::Value::Number(N::from_f32(300.0).unwrap()),
            ]),

            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=A+B",
        Some(&data_function),
        3.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=(A*(B+C))*B",
        Some(&data_function),
        10.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=fix_rate*input.",
        Some(&data_function),
        20.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=D+F",
        Some(&data_function),
        3.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=G+F",
        Some(&data_function),
        7.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM(A,B,C)",
        Some(&data_function),
        6.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE(Test)",
        Some(&data_function),
        200.0
    );
}

#[test]
fn it_evaluate_references_other_formulas() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Text("=1+B".to_string()),
            "B" => types::Value::Number(N::from_f32(3.0).unwrap()),
            _ => types::Value::Error(types::Error::Value),
        }
    }

    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=A+B",
        Some(&data_function),
        7.0
    );
}

#[test]
fn it_evaluate_references_boolean_formulas() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Boolean(types::Boolean::True),
            "B" => types::Value::Boolean(types::Boolean::False),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND(A,B)",
        Some(&data_function),
        "FALSE"
    );
}

#[test]
fn it_evaluate_references_error_value_formulas() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Boolean(types::Boolean::True),
            "B" => types::Value::Error(types::Error::Value), //types::Value::Boolean(types::Boolean::False),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND(A,B)",
        Some(&data_function),
        "#VALUE!"
    );
}

#[test]
fn it_evaluate_references_with_dot() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A.B" => types::Value::Number(N::from_f32(1.0).unwrap()),
            "B.C" => types::Value::Number(N::from_f32(2.0).unwrap()),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=A.B+B.C",
        Some(&data_function),
        3.0
    );
}

#[test]
fn it_evaluate_iterators() {
    test_all_types!(evaluate_formula_number, "=SUM({1,2,3})", 6.0);
    test_all_types!(evaluate_formula_number, "=PRODUCT({1,2,3})", 6.0);
    test_all_types!(evaluate_formula_number, "=SUM({1,2,3}, {5,6})", 17.0);
}

#[test]
fn it_evaluate_iterators_and_scalars() {
    test_all_types!(evaluate_formula_number, "=SUM({1,2,3}, 4)", 10.0);
    test_all_types!(evaluate_formula_number, "=PRODUCT({1,2,3}, 4)", 24.0);
}

#[test]
fn it_evaluate_multiple_iterators_and_scalars() {
    test_all_types!(evaluate_formula_number, "=SUM({1,2,3})", 6.0);
    test_all_types!(evaluate_formula_number, "=SUM({  1,2,3}, 4, {5, 6})", 21.0);
    test_all_types!(evaluate_formula_number, "=SUM({1,2,3},4,{5,6})", 21.0);
    test_all_types!(evaluate_formula_number, "=SUM({1+1,2,3-3},4,{5,6*1})", 19.0);
    test_all_types!(
        evaluate_formula_number,
        "=PRODUCT({1+1,2,3-2},4, {5,6*1})",
        480.0
    );
}

#[test]
fn it_evaluate_references_iterator() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A.B" => types::Value::Number(N::from_f32(1.0).unwrap()),
            "B.C" => types::Value::Number(N::from_f32(2.0).unwrap()),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM({A.B,B.C})",
        Some(&data_function),
        3.0
    );
}

#[test]
fn it_evaluate_iterator_operators() {
    test_all_types!(evaluate_formula_string, "={1,2,3}+{1,2,3}", "{2,4,6}");
    test_all_types!(evaluate_formula_string, "={3,2,1}-{1,2,3}", "{2,0,-2}");
    test_all_types!(evaluate_formula_string, "={1,2,3}*{1,2,3}", "{1,4,9}");
    test_all_types!(evaluate_formula_string, "={1,2,3}/{1,2,3}", "{1,1,1}");
    test_all_types!(
        evaluate_formula_string,
        "={1,2,3}/{0,0,0}",
        "{#DIV/0!,#DIV/0!,#DIV/0!}"
    );
    test_all_types!(evaluate_formula_string, "=-{1,2,3}", "{-1,-2,-3}");
    test_all_types!(evaluate_formula_string, "=-({1,2,3})", "{-1,-2,-3}");
}

#[test]
fn it_evaluate_iterator_in_logic_functions() {
    test_all_types!(evaluate_formula_string, "=AND(0,{0,0,0},0)", "FALSE");
    test_all_types!(evaluate_formula_string, "=AND(0,{0,0,0})", "FALSE");
    test_all_types!(evaluate_formula_string, "=AND({0,0,0},0)", "FALSE");
    test_all_types!(evaluate_formula_string, "=OR(1,{1,0,0},0,1)", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR(0,{0,1,0})", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR({0,0,0},1)", "TRUE");
    test_all_types!(evaluate_formula_string, "=AND({0,0,0})", "FALSE");
    test_all_types!(evaluate_formula_string, "=AND({1,0,0})", "FALSE");
    test_all_types!(evaluate_formula_string, "=AND({1,1,1})", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR({0,0,0})", "FALSE");
    test_all_types!(evaluate_formula_string, "=OR({1,0,0})", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR({0,1,1})", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR({1,0,1})", "TRUE");
    test_all_types!(evaluate_formula_string, "=OR({1,1,1})", "TRUE");
    test_all_types!(evaluate_formula_string, "=XOR({1,0,1})", "FALSE");
    test_all_types!(evaluate_formula_string, "=XOR({0,1,0})", "TRUE");
    test_all_types!(evaluate_formula_string, "=XOR({0,0,0})", "FALSE");
}

#[test]
fn it_evaluate_iterator_with_diffrent_number_of_entries() {
    test_all_types!(evaluate_formula_string, "={0,0}+{1,2,3}", "{1,2,#ARG!}");
    test_all_types!(evaluate_formula_string, "={0,0}*{1,2,3}", "{0,0,#ARG!}");
    test_all_types!(
        evaluate_formula_string,
        "={1,2,3}/{0,0}",
        "{#DIV/0!,#DIV/0!,#ARG!}"
    );
    test_all_types!(evaluate_formula_string, "={0,0}+{1,\"Hi\"}", "{1,#CAST!}");
}

#[test]
fn it_evaluate_date() {
    fn start_date() -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z").unwrap()
    }
    fn end_date() -> DateTime<FixedOffset> {
        DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z").unwrap()
    }
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "start" => types::Value::Date(start_date()),
            "end" => types::Value::Date(end_date()),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=DAYS(end, start)",
        Some(&data_function),
        182.00
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=days(end, start)",
        Some(&data_function),
        182.00
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=start + 1",
        Some(&data_function),
        (start_date() + Duration::days(1)).to_string()
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=end-3",
        Some(&data_function),
        (end_date() - Duration::days(3)).to_string()
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=year(end)",
        Some(&data_function),
        end_date().year().to_string()
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=month(end)",
        Some(&data_function),
        end_date().month().to_string()
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=day(end)",
        Some(&data_function),
        end_date().day().to_string()
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=year(end, 2)",
        Some(&data_function),
        "#REF!"
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=month(end, 2)",
        Some(&data_function),
        "#REF!"
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=day(end, 2)",
        Some(&data_function),
        "#REF!"
    );
}

#[test]
fn it_evaluate_custom_functions_() {
    fn custom_functions<N: XlNum>(s: String, params: Vec<N>) -> types::Value<N> {
        match s.as_str() {
            "Increase" => types::Value::Number(params[0] + N::one()),
            "SimpleSum" => types::Value::Number(params[0] + params[1]),
            "CustomSum" => types::Value::Number(params[0] + params[1] + params[2]),
            "EqualFive" => types::Value::Number(N::from_f32(5.0).unwrap()),
            "CountText" => types::Value::Text(10.0.to_string()),
            "CountNumber" => types::Value::Number(N::from_f32(20.0).unwrap()),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=Increase(1)",
        Some(&custom_functions),
        2.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=SimpleSum(1,2)",
        Some(&custom_functions),
        3.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=CustomSum(1,2,3)",
        Some(&custom_functions),
        6.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=EqualFive()+1",
        Some(&custom_functions),
        6.0
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_custom_function,
        "=\"P\"&CountText()",
        Some(&custom_functions),
        "P10"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_custom_function,
        "=\"P\"&CountNumber()",
        Some(&custom_functions),
        "P20"
    );
}

#[test]
fn it_evaluate_left_and_right_functions() {
    test_all_types!(evaluate_formula_string, "=RIGHT(\"apple\", 3)", "ple");
    test_all_types!(evaluate_formula_string, "=RIGHT(\"apple\")", "e");

    test_all_types!(evaluate_formula_string, "=\"P\"&RIGHT(\"000\"&1,3)", "P001");
    test_all_types!(evaluate_formula_string, "=LEFT(\"apple\", 3)", "app");
    test_all_types!(evaluate_formula_string, "=LEFT(\"apple\")", "a");

    test_all_types!(evaluate_formula_string, "=\"P\"&LEFT(\"000\"&1,3)", "P000");

    test_all_types!(evaluate_formula_string, "=LEFT(\"apple\", 10)", "apple");
    test_all_types!(evaluate_formula_string, "=RIGHT(\"apple\", 10)", "apple");
}

#[test]
fn it_evaluates_blanks() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Number(N::one()),
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM(A,B)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=A+B",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference_no_conversion,
        "=SUM(A,C)",
        Some(&data_function),
        types::Value::Error(types::Error::Value)
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE(A,B)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM(A,B,2,3,B)",
        Some(&data_function),
        6.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE(A,B,2,3,B)",
        Some(&data_function),
        2.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT(A,B)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT(1,B,B,B,A,2)",
        Some(&data_function),
        2.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=1+B+B+B",
        Some(&data_function),
        1.0
    );
}

#[test]
fn it_evaluates_blanks_only() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM(B)",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=AVERAGE(B)",
        Some(&data_function),
        "#DIV/0!"
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT(B)",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=-B",
        Some(&data_function),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_when_blank_in_first_position() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Number(N::one()),
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM(B,A)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B+A",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE(B,A)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM(B,2,3,B)",
        Some(&data_function),
        5.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE(B,2,3,B)",
        Some(&data_function),
        2.5
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT(B,A)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT(B,A,B)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT(B,B,B)",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT(B,B,B,A,2)",
        Some(&data_function),
        2.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B+B+B",
        Some(&data_function),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_in_abs_function() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=ABS(B)",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=ABS(-B)",
        Some(&data_function),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_in_days_function() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        let start: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339("2019-02-01T02:00:00.000Z").unwrap();
        match s.as_str() {
            "start" => types::Value::Date(start),
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=DAYS(B, B)",
        Some(&data_function),
        0.00
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=DAYS(start, B)",
        Some(&data_function),
        43495.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=DAYS(B, start)",
        Some(&data_function),
        -43495.0
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=year(B)",
        Some(&data_function),
        "#VALUE!"
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=month(B)",
        Some(&data_function),
        "#VALUE!"
    );
    test_all_types_with_data!(
        evaluate_formula_date_with_reference,
        "=day(B)",
        Some(&data_function),
        "#VALUE!"
    );
}

#[test]
fn it_evaluates_blanks_with_operators() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=1-B",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B-1",
        Some(&data_function),
        -1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=1+B",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B+1",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=1/B",
        Some(&data_function),
        "#DIV/0!"
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B/1",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=1*B",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B*1",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=1^B",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B^1",
        Some(&data_function),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_with_operators_and_reference() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Number(N::one()),
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=A-B",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B-A",
        Some(&data_function),
        -1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=A+B",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B+A",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=A/B",
        Some(&data_function),
        "#DIV/0!"
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B/A",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=A*B",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B*A",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=A^B",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=B^A",
        Some(&data_function),
        0.0
    );
}

#[test]
fn it_evaluates_blanks_in_boolean_operations() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "T" => types::Value::Boolean(types::Boolean::True),
            "B" => types::Value::Blank,
            "F" => types::Value::Boolean(types::Boolean::False),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND(T,B)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=OR(T,B)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=NOT(B)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND(F,B)",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=OR(F,B)",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=OR(T,B,F,B)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=OR(F,B,T,B)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND(T,B,F,B)",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND(F,B,T,B)",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR(T,B)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR(F,B)",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR(T,B,F,B)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR(F,B,T,B)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR(T,B,F,B,T)",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR(F,B,T,B,F)",
        Some(&data_function),
        "TRUE"
    );
}

#[test]
fn evaluate_invalid_reference_in_functions() {
    fn data_function<N: XlNum>(_s: String) -> types::Value<N> {
        types::Value::Error(types::Error::Value)
    }

    // test with a non existing reference
    assert_eq!(
        calculate::calculate_formula::<f32>(
            parse_formula::parse_string_to_formula(
                "=AND(T, NO_REFERENCE)",
                None::<NoCustomFunction<_>>,
            ),
            Some(&data_function)
        ),
        types::Value::Error(types::Error::Value)
    );
    assert_eq!(
        calculate::calculate_formula::<f64>(
            parse_formula::parse_string_to_formula(
                "=AND(T, NO_REFERENCE)",
                None::<NoCustomFunction<_>>,
            ),
            Some(&data_function)
        ),
        types::Value::Error(types::Error::Value)
    );

    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND(T, NO_REFERENCE)",
        Some(&data_function),
        "#VALUE!"
    );
}

#[test]
fn it_evaluates_blanks_in_comparison_operators() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B=B",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=1=B",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B=1",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=1>B",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B>1",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=0=B",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B=0",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=B=\"\"",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=\"\"=B",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=\"something\"=B",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B='test'",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "='test'=B",
        Some(&data_function),
        "FALSE"
    );
}

#[test]
fn it_evaluates_blanks_in_comparison_operators_with_references() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Number(N::from_f32(-2.0).unwrap()),
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=A>=B",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B>=A",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=A<B",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B<A",
        Some(&data_function),
        "FALSE"
    );
}

#[test]
fn it_evaluates_blanks_string_operations() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Number(N::from_f32(-2.0).unwrap()),
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=\"Hello\"&B",
        Some(&data_function),
        "Hello"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B&\"Hello\"",
        Some(&data_function),
        "Hello"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=B&B",
        Some(&data_function),
        ""
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=RIGHT(B)",
        Some(&data_function),
        ""
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=LEFT(B)",
        Some(&data_function),
        ""
    );
}

#[test]
fn it_evaluates_blank_constructors() {
    fn custom_functions<N: XlNum>(s: String, _params: Vec<N>) -> types::Value<N> {
        match s.as_str() {
            "BLANK" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=SUM(BLANK())",
        Some(&custom_functions),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=SUM(BLANK(), 1)",
        Some(&custom_functions),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=PRODUCT(BLANK(), 1)",
        Some(&custom_functions),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=AVERAGE(BLANK(), 1)",
        Some(&custom_functions),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=BLANK()+1",
        Some(&custom_functions),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_custom_function,
        "=BLANK()*1",
        Some(&custom_functions),
        0.0
    );
}

#[test]
fn it_evaluates_blank_in_iterators() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "A" => types::Value::Number(N::from_f32(100.0).unwrap()),
            "Array" => types::Value::Iterator(vec![
                types::Value::Number(N::from_f32(100.0).unwrap()),
                types::Value::Blank,
                types::Value::Blank,
            ]),
            "B" => types::Value::Blank,
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE({A, B, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE({A, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE({B, A, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=AVERAGE(Array)",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM({A, B, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM({A, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM({B, A, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=SUM(Array)",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT({A, B, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT({A, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT({B, A, B})",
        Some(&data_function),
        100.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=PRODUCT(Array)",
        Some(&data_function),
        100.0
    );
}

#[test]
fn it_evaluates_blank_with_iterators_in_boolean_operations() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "T" => types::Value::Boolean(types::Boolean::True),
            "B" => types::Value::Blank,
            "F" => types::Value::Boolean(types::Boolean::False),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND({T,B})",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=OR({T,B})",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND({F,B})",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=OR({F,B})",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=OR({T,B,F,B})",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=OR({F,B,T,B})",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND({T,B,F,B})",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND({F,B,T,B})",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR({T,B})",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR({F,B})",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR({T,B,F,B})",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR({F,B,T,B})",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR({T,B,F,B,T})",
        Some(&data_function),
        "FALSE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR({F,B,T,B,F})",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=XOR({F,B,T,B,F,{F,B,T,B,F}})",
        Some(&data_function),
        "FALSE"
    );
}

#[test]
fn it_evaluates_formulas_with_3_params() {
    test_all_types!(evaluate_formula_number, "=IF()", 0.0);
    test_all_types!(evaluate_formula_number, "=IF( )", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF( ,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(, )", 0.0);
    test_all_types!(evaluate_formula_number, "=IF( , )", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(,,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(, ,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(,,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(,, )", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(1,,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(1,2,)", 2.0);
    test_all_types!(evaluate_formula_number, "=IF(1,2)", 2.0);
    test_all_types!(evaluate_formula_number, "=IF(0,2,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(0,2)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(0,,3)", 3.0);
    test_all_types!(evaluate_formula_number, "=IF(1,,3)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(0,2,3)", 3.0);
    test_all_types!(evaluate_formula_number, "=IF(,2,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(,2)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(,2,3)", 3.0);
    test_all_types!(evaluate_formula_number, "=IF(,,3)", 3.0);
}

#[test]
fn it_evaluates_if_formulas() {
    test_all_types!(evaluate_formula_number, "=IF(TRUE,1,0)", 1.0);
    test_all_types!(evaluate_formula_number, "=IF(FALSE,1,0)", 0.0);
    test_all_types!(evaluate_formula_string, "=IF(TRUE,\"a\",0)", "a");
    test_all_types!(evaluate_formula_number, "=IF(FALSE,\"a\",0)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(1=1,1,0)", 1.0);
    test_all_types!(evaluate_formula_number, "=IF(1=2,1,0)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(AND(TRUE,FALSE),1,0)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(TRUE,IF(FALSE,1,2),0)", 2.0);
    test_all_types!(evaluate_formula_number, "=IF(2,1,0)", 1.0);
    test_all_types!(evaluate_formula_number, "=IF(-1,1,0)", 1.0);
    test_all_types!(evaluate_formula_number, "=IF(0,1,0)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(TRUE,1+2+3,0)", 6.0);
    test_all_types!(evaluate_formula_number, "=IF(FALSE,1)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(FALSE,1,)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(TRUE,,1)", 0.0);
    test_all_types!(evaluate_formula_number, "=IF(TRUE, ,1)", 0.0);
    test_all_types!(evaluate_formula_string, "=IF(TRUE,TRUE,FALSE)", "TRUE");
    test_all_types!(
        evaluate_formula_string,
        "=IF( TRUE , TRUE , FALSE )",
        "TRUE"
    );
    test_all_types!(evaluate_formula_string, "=IF(TRUE , TRUE, FALSE )", "TRUE");
    test_all_types!(evaluate_formula_number, "=IF(1,IF(FALSE,1,2),0)", 2.0);
    test_all_types!(
        evaluate_formula_number,
        "=IF(1=0,IF(FALSE,1,2),IF(TRUE,1,2))",
        1.0
    );
    test_all_types!(
        evaluate_formula_string,
        "=IF(1/0,IF(FALSE,1,2),0)",
        "#DIV/0!"
    );
    test_all_types!(evaluate_formula_string, "=IF(\"text\",1,0)", "#VALUE!");

    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        let date1: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z").unwrap();
        let date2: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z").unwrap();
        match s.as_str() {
            "date1" => types::Value::Date(date1),
            "date2" => types::Value::Date(date2),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=IF(date1=date2,1,0)",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=IF(date1<>date2,1,0)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=IF(date1<date2,1,0)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=IF(date1>date2,1,0)",
        Some(&data_function),
        0.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=IF(date1<=date2,1,0)",
        Some(&data_function),
        1.0
    );
    test_all_types_with_data!(
        evaluate_formula_number_with_reference,
        "=IF(date1>=date2,1,0)",
        Some(&data_function),
        0.0
    );
}

#[test]
fn it_evaluates_if_formulas_with_text() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "ReferenceKey" => types::Value::Text("100".to_string()),
            "ReferenceName" => types::Value::Text("Test".to_string()),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=IF(ReferenceKey=\"10\",\"\",ReferenceKey&\" - \")&ReferenceName",
        Some(&data_function),
        "100 - Test"
    );
}

#[test]
fn it_evaluates_isblank_function() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            // "ReferenceKey" => types::Value::Text("100".to_string()),
            "ReferenceName" => types::Value::Text("Test".to_string()),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=ISBLANK(ReferenceKey)",
        Some(&data_function),
        "TRUE"
    );
    test_all_types_with_data!(
        evaluate_formula_string_with_reference,
        "=ISBLANK(ReferenceName)",
        Some(&data_function),
        "FALSE"
    );
}

#[test]
fn test_inner_function_with_whitespace() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "any_dropdowns" => types::Value::Number(N::one()),
            "primary_carrier" => types::Value::Text("Allianz*".to_owned()),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=AND(RIGHT(primary_carrier, 1)=\"*\", 1 > 0)",
        Some(&data_function),
        "TRUE"
    );
}

#[test]
fn test_zero_padding() {
    fn data_function<N: XlNum>(s: String) -> types::Value<N> {
        match s.as_str() {
            "num" => types::Value::Number(N::from_f32(91.0).unwrap()),
            _ => types::Value::Error(types::Error::Value),
        }
    }
    test_all_types_with_data!(
        evaluate_formula_boolean_with_reference,
        "=RIGHT(\"00000000\" & num, 8)",
        Some(&data_function),
        "00000091"
    );
}
