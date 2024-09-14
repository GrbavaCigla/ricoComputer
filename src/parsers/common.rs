use nom::{
    character::complete::{digit1, multispace0, space0},
    combinator::{map_res, recognize},
    error::{FromExternalError, ParseError},
    sequence::{delimited, terminated},
    IResult, Parser,
};
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

pub fn number<'a, T, E, E1>(input: &'a str) -> IResult<&'a str, T, E>
where
    E: FromExternalError<&'a str, E1> + ParseError<&'a str>,
    T: FromStr<Err = E1>,
{
    map_res(recognize(digit1), str::parse).parse(input)
}

pub fn empty_line<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    terminated(inner, multispace0)
}
