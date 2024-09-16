use nom::{
    branch::alt,
    character::complete::{char, multispace0, one_of, space0},
    combinator::{map_res, opt, recognize},
    error::{FromExternalError, ParseError},
    multi::{many0, many1},
    sequence::{delimited, pair, terminated},
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

pub fn number<'a, T, E, E1>(input: &'a str) -> IResult<&'a str, (T, bool), E>
where
    E: FromExternalError<&'a str, E1> + ParseError<&'a str>,
    T: FromStr<Err = E1>,
{
    map_res(
        recognize(pair(
            opt(alt((char::<&str, E>('+'), char('-')))),
            many1(terminated(one_of("0123456789"), many0(char('_')))),
        )),
        |s| {
            match s.starts_with('-') {
                true => str::parse::<T>(&str::replace(s, "_", "")[1..]),
                false => str::parse::<T>(&str::replace(s, "_", "")[..]),
            }
            .map(|x| (x, s.starts_with('-')))
        },
    )
    .parse(input)
}

pub fn empty_line<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    terminated(inner, multispace0)
}
