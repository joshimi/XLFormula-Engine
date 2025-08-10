use chrono::{DateTime, FixedOffset};
use num_traits::{AsPrimitive, Float, FromPrimitive};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};
/// Defines Excel Functions.
#[derive(Debug, Copy, Clone)]
pub enum Function {
    Abs,
    Sum,
    Product,
    Average,
    Or,
    And,
    Xor,
    Not,
    Negate,
    Days,
    Right,
    Left,
    Iff,
    IsBlank,
}

/// Defines Excel Operators.
#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Concat,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    Function(Function),
}

/// Defines error types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Error {
    Div0,
    Cast,
    Parse,
    Value,
    Argument,
    Reference,
}

/// Defines boolean types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Boolean {
    True,
    False,
}

pub trait XlNum:
    Float
    + AsPrimitive<i64>
    + AsPrimitive<i32>
    + AsPrimitive<usize>
    + FromPrimitive
    + FromStr
    + Debug
    + Display
{
}

impl XlNum for f32 {}
impl XlNum for f64 {}

/// The result of an evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Value<N>
where
    N: XlNum,
{
    Number(N),
    Text(String),
    Boolean(Boolean),
    Iterator(Vec<Self>),
    Error(Error),
    Date(DateTime<FixedOffset>),
    Blank,
}

/// Defines each term in Expression Struct.
#[derive(Debug, Clone)]
pub enum Formula<N>
where
    N: XlNum,
{
    Operation(Expression<N>),
    Value(Value<N>),
    Reference(String),
    Iterator(Vec<Self>),
}

/// Struct that holds a parsed string. Formula enum and Expression Struct are defined recursively.
#[derive(Debug, Clone)]
pub struct Expression<N>
where
    N: XlNum,
{
    pub op: Operator,
    pub values: Vec<Formula<N>>,
}
