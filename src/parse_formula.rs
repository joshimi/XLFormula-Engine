use crate::types::{self, XlNum};
use pest::{
    pratt_parser::{Assoc, Op, PrattParser},
    Parser,
};
use pest_derive::Parser;
use std::{fmt::Debug, str::FromStr};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct GrammarParser;

/// Use this function to catch a parse error.
fn parse_string(s: &'_ str) -> Option<pest::iterators::Pair<'_, Rule>> {
    let parse_result = GrammarParser::parse(Rule::formula, s);
    //println!("{:#?}", parse_result);
    match parse_result {
        Ok(mut result) => {
            let parse_result = result.next().unwrap();
            Some(parse_result)
        }
        Err(_) => None,
    }
    // GrammarParser::parse(Rule::formula, s)
    //     .expect("unsuccessful parse")
    //     .next()
}

fn parse_string_constant<N>(parse_result: pest::iterators::Pair<Rule>) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let string = parse_result
        .into_inner()
        .as_str()
        .parse::<String>()
        .unwrap();
    types::Formula::Value(types::Value::Text(
        string.trim_start_matches('\'').to_string(),
    ))
}

/// Parses a string and stores it in Formula Enum.
pub fn parse_string_to_formula<N>(
    s: &str,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    match parse_string(s) {
        Some(parse_result) => match parse_result.as_rule() {
            Rule::expr => build_formula_with_parser(parse_result.into_inner(), f),
            Rule::string_constant => parse_string_constant(parse_result),
            _ => types::Formula::Value(types::Value::Error(types::Error::Parse)),
        },
        None => types::Formula::Value(types::Value::Error(types::Error::Parse)),
    }
}

fn build_formula_number<N>(pair: pest::iterators::Pair<Rule>) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let s = pair.as_str();
    let x = s.parse::<N>().unwrap();
    let value = types::Value::Number(x);
    types::Formula::Value(value)
}

fn build_formula_string_double_quote<N>(pair: pest::iterators::Pair<Rule>) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let string = pair.into_inner().as_str().parse::<String>().unwrap();
    let value = types::Value::Text(string.replace("\"\"", "\""));
    types::Formula::Value(value)
}

fn build_formula_string_single_quote<N>(pair: pest::iterators::Pair<Rule>) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let string = pair.into_inner().as_str().parse::<String>().unwrap();
    let value = types::Value::Text(string);
    types::Formula::Value(value)
}

fn build_formula_boolean<N>(boolean_value: bool) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    if boolean_value {
        types::Formula::Value(types::Value::Boolean(types::Boolean::True))
    } else {
        types::Formula::Value(types::Value::Boolean(types::Boolean::False))
    }
}

fn build_formula_unary_operator<N>(
    unary_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let op_type = match unary_operation {
        Rule::abs => types::Operator::Function(types::Function::Abs),
        Rule::not => types::Operator::Function(types::Function::Not),
        Rule::negate => types::Operator::Function(types::Function::Negate),
        _ => unreachable!(),
    };
    let operation = types::Expression {
        op: op_type,
        values: vec![build_formula_with_parser(pair.into_inner(), f)],
    };
    types::Formula::Operation(operation)
}

fn build_formula_reference<N>(pair: pest::iterators::Pair<Rule>) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let string = pair.as_str().parse::<String>().unwrap();
    types::Formula::Reference(string)
}

fn build_formula_iterator<N>(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        vec.push(build_formula_with_parser(term.into_inner(), f));
    }
    types::Formula::Iterator(vec)
}

fn build_formula_collective_operator<N>(
    collective_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        if (term.as_str().parse::<String>().unwrap() == "")
            | (term.as_str().parse::<String>().unwrap() == ",")
            | (term.as_str().parse::<String>().unwrap() == ", ")
            | (term.as_str().parse::<String>().unwrap() == " ,")
        {
            vec.push(types::Formula::Value(types::Value::Blank))
        } else {
            vec.push(build_formula_with_parser(term.into_inner(), f))
        }
    }
    let op_type = rule_to_function_operator(collective_operation);
    let operation = types::Expression {
        op: op_type,
        values: vec,
    };
    types::Formula::Operation(operation)
}

fn rule_to_function_operator(collective_operation: Rule) -> types::Operator {
    match collective_operation {
        Rule::sum => types::Operator::Function(types::Function::Sum),
        Rule::product => types::Operator::Function(types::Function::Product),
        Rule::average => types::Operator::Function(types::Function::Average),
        Rule::or => types::Operator::Function(types::Function::Or),
        Rule::and => types::Operator::Function(types::Function::And),
        Rule::xor => types::Operator::Function(types::Function::Xor),
        Rule::days => types::Operator::Function(types::Function::Days),
        Rule::right => types::Operator::Function(types::Function::Right),
        Rule::left => types::Operator::Function(types::Function::Left),
        Rule::iff => types::Operator::Function(types::Function::Iff),
        Rule::isblank => types::Operator::Function(types::Function::IsBlank),
        _ => unreachable!(),
    }
}

fn build_formula_collective_operator_average<N>(
    collective_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        if (term.as_str().parse::<String>().unwrap() == "")
            | (term.as_str().parse::<String>().unwrap() == ",")
            | (term.as_str().parse::<String>().unwrap() == ", ")
            | (term.as_str().parse::<String>().unwrap() == " ,")
        {
            vec.push(types::Formula::Value(types::Value::Number(N::zero())))
        } else {
            vec.push(build_formula_with_parser(term.into_inner(), f))
        }
    }
    let op_type = rule_to_function_operator(collective_operation);
    let operation = types::Expression {
        op: op_type,
        values: vec,
    };
    types::Formula::Operation(operation)
}

fn build_formula_collective_operator_and<N>(
    collective_operation: Rule,
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        if (term.as_str().parse::<String>().unwrap() == "")
            | (term.as_str().parse::<String>().unwrap() == ",")
            | (term.as_str().parse::<String>().unwrap() == ", ")
            | (term.as_str().parse::<String>().unwrap() == " ,")
        {
            vec.push(types::Formula::Value(types::Value::Boolean(
                types::Boolean::False,
            )))
        } else {
            vec.push(build_formula_with_parser(term.into_inner(), f))
        }
    }
    let op_type = rule_to_function_operator(collective_operation);
    let operation = types::Expression {
        op: op_type,
        values: vec,
    };
    types::Formula::Operation(operation)
}

fn build_formula_iff<N>(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut vec = Vec::new();
    for term in pair.into_inner() {
        if (term.as_str().parse::<String>().unwrap() == "")
            | (term.as_str().parse::<String>().unwrap() == ",")
            | (term.as_str().parse::<String>().unwrap() == ", ")
            | (term.as_str().parse::<String>().unwrap() == " ,")
        {
            vec.push(types::Formula::Value(types::Value::Blank))
        } else {
            vec.push(build_formula_with_parser(term.into_inner(), f))
        }
    }
    let operation = types::Expression {
        op: types::Operator::Function(types::Function::Iff),
        values: vec,
    };
    types::Formula::Operation(operation)
}

fn build_formula_custom_function<N>(
    pair: pest::iterators::Pair<Rule>,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let mut vec = Vec::new();
    for field in pair.clone().into_inner() {
        if field.as_rule() == Rule::expr {
            let x = field.into_inner().as_str();
            let y = x.parse::<N>();
            if let Ok(y) = y {
                vec.push(y);
            }
        }
    }
    let mut ref_string = String::new();
    for field in pair.clone().into_inner() {
        ref_string = match field.as_rule() {
            Rule::reference => field.as_str().parse::<String>().unwrap(),
            _ => ref_string,
        }
    }
    match f {
        Some(f) => match f(ref_string, vec) {
            types::Value::Number(x) => types::Formula::Value(types::Value::Number(x)),
            types::Value::Text(s) => types::Formula::Value(types::Value::Text(s)),
            types::Value::Boolean(x) => types::Formula::Value(types::Value::Boolean(x)),
            types::Value::Error(types::Error::Value) => {
                types::Formula::Value(types::Value::Error(types::Error::Value))
            }
            types::Value::Iterator(v) => types::Formula::Value(types::Value::Iterator(v)),
            types::Value::Date(d) => types::Formula::Value(types::Value::Date(d)),
            types::Value::Blank => types::Formula::Value(types::Value::Blank),
            _ => types::Formula::Value(types::Value::Error(types::Error::Reference)),
        },
        None => types::Formula::Value(types::Value::Error(types::Error::Reference)),
    }
}

fn build_formula_binary_operator<N>(
    binary_operator: Rule,
    lhs: types::Formula<N>,
    rhs: types::Formula<N>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let op_type = match binary_operator {
        Rule::add => types::Operator::Plus,
        Rule::subtract => types::Operator::Minus,
        Rule::multiply => types::Operator::Multiply,
        Rule::divide => types::Operator::Divide,
        Rule::power => types::Operator::Power,
        Rule::concat => types::Operator::Concat,
        Rule::equal => types::Operator::Equal,
        Rule::not_equal => types::Operator::NotEqual,
        Rule::greater => types::Operator::Greater,
        Rule::less => types::Operator::Less,
        Rule::greater_or_equal => types::Operator::GreaterOrEqual,
        Rule::less_or_equal => types::Operator::LessOrEqual,
        _ => unreachable!(),
    };
    let operation = types::Expression {
        op: op_type,
        values: vec![lhs, rhs],
    };
    types::Formula::Operation(operation)
}

fn build_formula_with_parser<N>(
    expression: pest::iterators::Pairs<Rule>,
    f: Option<&impl Fn(String, Vec<N>) -> types::Value<N>>,
) -> types::Formula<N>
where
    N: XlNum,
    <N as FromStr>::Err: Debug,
{
    let pratt = PrattParser::new()
        .op(Op::infix(Rule::concat, Assoc::Left))
        .op(Op::infix(Rule::equal, Assoc::Left) | Op::infix(Rule::not_equal, Assoc::Left))
        .op(Op::infix(Rule::greater, Assoc::Left)
            | Op::infix(Rule::less, Assoc::Left)
            | Op::infix(Rule::greater_or_equal, Assoc::Left)
            | Op::infix(Rule::less_or_equal, Assoc::Left))
        .op(Op::infix(Rule::add, Assoc::Left) | Op::infix(Rule::subtract, Assoc::Left))
        .op(Op::infix(Rule::multiply, Assoc::Left) | Op::infix(Rule::divide, Assoc::Left))
        .op(Op::infix(Rule::power, Assoc::Right));
    pratt
        .map_primary(|pair: pest::iterators::Pair<Rule>| match pair.as_rule() {
            Rule::number => build_formula_number(pair),
            Rule::string_double_quote => build_formula_string_double_quote(pair),
            Rule::string_single_quote => build_formula_string_single_quote(pair),
            Rule::t => build_formula_boolean(true),
            Rule::f => build_formula_boolean(false),
            Rule::abs => build_formula_unary_operator(Rule::abs, pair, f),
            Rule::sum => build_formula_collective_operator(Rule::sum, pair, f),
            Rule::product => build_formula_collective_operator(Rule::product, pair, f),
            Rule::average => build_formula_collective_operator_average(Rule::average, pair, f),
            Rule::or => build_formula_collective_operator(Rule::or, pair, f),
            Rule::and => build_formula_collective_operator_and(Rule::and, pair, f),
            Rule::xor => build_formula_collective_operator(Rule::xor, pair, f),
            Rule::not => build_formula_unary_operator(Rule::not, pair, f),
            Rule::reference => build_formula_reference(pair),
            Rule::iterator => build_formula_iterator(pair, f),
            Rule::negate => build_formula_unary_operator(Rule::negate, pair, f),
            Rule::expr => build_formula_with_parser(pair.into_inner(), f),
            Rule::days => build_formula_collective_operator(Rule::days, pair, f),
            Rule::right => build_formula_collective_operator(Rule::right, pair, f),
            Rule::left => build_formula_collective_operator(Rule::left, pair, f),
            Rule::custom_function => build_formula_custom_function(pair, f),
            Rule::iff => build_formula_iff(pair, f),
            Rule::isblank => build_formula_collective_operator(Rule::isblank, pair, f),
            _ => unreachable!(),
        })
        .map_infix(
            |lhs: types::Formula<N>, op: pest::iterators::Pair<Rule>, rhs: types::Formula<N>| {
                match op.as_rule() {
                    Rule::add => build_formula_binary_operator(Rule::add, lhs, rhs),
                    Rule::subtract => build_formula_binary_operator(Rule::subtract, lhs, rhs),
                    Rule::multiply => build_formula_binary_operator(Rule::multiply, lhs, rhs),
                    Rule::divide => build_formula_binary_operator(Rule::divide, lhs, rhs),
                    Rule::power => build_formula_binary_operator(Rule::power, lhs, rhs),
                    Rule::concat => build_formula_binary_operator(Rule::concat, lhs, rhs),
                    Rule::equal => build_formula_binary_operator(Rule::equal, lhs, rhs),
                    Rule::not_equal => build_formula_binary_operator(Rule::not_equal, lhs, rhs),
                    Rule::greater => build_formula_binary_operator(Rule::greater, lhs, rhs),
                    Rule::less => build_formula_binary_operator(Rule::less, lhs, rhs),
                    Rule::greater_or_equal => {
                        build_formula_binary_operator(Rule::greater_or_equal, lhs, rhs)
                    }
                    Rule::less_or_equal => {
                        build_formula_binary_operator(Rule::less_or_equal, lhs, rhs)
                    }
                    _ => unreachable!(),
                }
            },
        )
        .parse(expression)
}
