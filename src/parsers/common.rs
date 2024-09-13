use nom::{
    character::complete::{digit1, multispace0, space0},
    combinator::{map_res, recognize},
    error::ParseError,
    sequence::delimited,
    IResult, Parser,
};
use std::str::FromStr;
use num_traits::Unsigned;

pub fn mws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(
    inner: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
{
    delimited(space0, inner, space0)
}

pub fn number<T: Unsigned + FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(digit1), str::parse)(input)
}