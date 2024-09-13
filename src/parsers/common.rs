use nom::{
    character::complete::{digit1, line_ending, multispace0, space0},
    combinator::{map_res, recognize},
    error::ParseError,
    sequence::{delimited, terminated},
    IResult, Parser,
};
use num_traits::Unsigned;
use std::str::FromStr;

pub fn mws<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    delimited(multispace0, inner, multispace0)
}

pub fn ws<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    delimited(space0, inner, space0)
}

pub fn number<T: Unsigned + FromStr>(input: &str) -> IResult<&str, T> {
    map_res(recognize(digit1), str::parse)(input)
}

pub fn empty_line<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    terminated(inner, multispace0)
}
