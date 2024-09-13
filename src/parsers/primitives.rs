use std::str::FromStr;

use nom::{character::complete::digit1, combinator::{map_res, recognize}, IResult};
use num_traits::Unsigned;

pub fn number<T: Unsigned + FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(digit1), str::parse)(input)
}