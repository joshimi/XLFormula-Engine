#[doc = include_str!("../README.md")]
/// Evaluates a formula.
pub mod calculate;

/// The Structs and Enums for the calculation.
pub mod types;

/// Parses a string using `pest` and `pest::prec_climber`.
pub mod parse_formula;

pub type NoReference<'a, N> = &'a fn(String) -> types::Value<N>;
pub type NoCustomFunction<'a, N> = &'a fn(String, Vec<N>) -> types::Value<N>;
