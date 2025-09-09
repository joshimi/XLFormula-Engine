use super::result_to_string;
use crate::types::{self, XlNum};

pub fn show_number<N>(number: N) -> String
where
    N: XlNum,
{
    if number.is_infinite() {
        types::Error::Div0.to_string()
    } else {
        number.to_string()
    }
}

pub fn show_iterator<N>(value_vec: Vec<types::Value<N>>) -> String
where
    N: XlNum,
{
    let mut result = "{".to_string();
    for top in value_vec {
        result = format!("{result}{},", result_to_string(top));
    }
    result = result.trim_end_matches(',').to_string();
    result + "}"
}

pub fn show_blank<N>() -> String
where
    N: XlNum,
{
    show_number(N::zero())
}
