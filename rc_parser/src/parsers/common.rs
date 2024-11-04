use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, char, line_ending, one_of, space0, space1},
    combinator::{eof, map_res, opt, recognize, value},
    error::{FromExternalError, ParseError},
    multi::{many0, many0_count, many1},
    sequence::{delimited, pair, preceded, terminated},
    IResult, Parser,
};
use std::str::FromStr;
use nom_supreme::tag::complete::tag;

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

pub fn comment<'a, E, O, F>(mut terminator: F) -> impl FnMut(&'a str) -> IResult<&'a str, (), E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    move |input: &'a str| {
        let (mut input, _) = char(';').parse(input)?;

        loop {
            if let Ok((inp, _)) = terminator.parse(input) {
                return Ok((inp, ()));
            }

            // TODO: Check unicode?
            let mut chars = input.chars();
            chars.next();
            input = chars.as_str();
        }
    }
}

pub fn opt_comment_end<'a, E, O, F>(terminator: F) -> impl Fn(&'a str) -> IResult<&'a str, (), E>
where
    F: Parser<&'a str, O, E> + Clone,
    E: ParseError<&'a str>,
{
    move |input: &'a str| {
        let term1 = terminator.clone();
        let term2 = terminator.clone();
        alt((value((), term1), comment(term2))).parse(input)
    }
}

pub fn cmultispace0<'a, E, F, O>(terminator: F) -> impl Fn(&'a str) -> IResult<&'a str, (), E>
where
    F: Parser<&'a str, O, E> + Clone + 'a,
    E: ParseError<&'a str> + 'a,
    O: 'a,
{
    move |input: &'a str| {
        let (input, _) = many0(alt((
            value((), ws(comment(terminator.clone()))),
            value((), line_ending),
            value((), space1),
        )))
        .parse(input)?;

        return Ok((input, ()));
    }
}

pub fn cmws<'a, E, F1, F2, O, O1>(
    mut inner: F1,
    terminator: F2,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F1: Parser<&'a str, O, E>,
    F2: Parser<&'a str, O1, E> + Clone + 'a,
    E: ParseError<&'a str> + 'a,
    O1: 'a,
{
    move |input: &'a str| {
        let term1 = terminator.clone();
        let term2 = terminator.clone();

        let (input, _) = cmultispace0(term1).parse(input)?;
        let (input, val) = inner.parse(input)?;
        let (input, _) = cmultispace0(term2).parse(input)?;

        Ok((input, val))
    }
}

pub fn eofl<'a, E>(input: &'a str) -> IResult<&'a str, (), E>
where
    E: ParseError<&'a str>,
{
    value((), alt((line_ending, eof))).parse(input)
}

pub fn empty_line<'a, E, F1, F2, O, O1>(
    mut inner: F1,
    terminator: F2,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F1: Parser<&'a str, O, E>,
    F2: Parser<&'a str, O1, E> + Clone + 'a,
    E: ParseError<&'a str> + 'a,
    O1: 'a,
{
    move |input: &'a str| {
        let (input, val) = inner.parse(input)?;
        let (input, _) = cmultispace0(terminator.clone()).parse(input)?;

        Ok((input, val))
    }
}

pub fn rm_bom<'a, F: 'a, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    // TODO: Add more BOMs
    preceded(opt(char('\u{feff}')), inner)
}

pub fn sym<'a>(input: &'a str) -> rc_common::IResult<&'a str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}
