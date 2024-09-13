use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, space0},
    combinator::recognize,
    multi::many0_count,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};

use crate::types::{Declaration, Reference};

use super::common::number;

pub fn sym(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn decl(input: &str) -> IResult<&str, Declaration> {
    let (input, (symbol, value)) =
        separated_pair(terminated(sym, space0), char('='), preceded(space0, number))(input)?;

    Ok((
        input,
        Declaration {
            symbol: symbol.to_string(),
            value: value,
        },
    ))
}

pub fn ref_dir(input: &str) -> IResult<&str, Reference> {
    let (input, name) = sym(input)?;
    Ok((input, Reference::Direct(name.to_string())))
}

pub fn ref_ind(input: &str) -> IResult<&str, Reference> {
    let (input, name) = delimited(char('('), sym, char(')'))(input)?;
    Ok((input, Reference::Indirect(name.to_string())))
}

pub fn ref_addr(input: &str) -> IResult<&str, Reference> {
    let (input, name) = preceded(char('#'), sym)(input)?;
    Ok((input, Reference::Addr(name.to_string())))
}

pub fn ref_val(input: &str) -> IResult<&str, Reference> {
    let (input, name) = number(input)?;
    Ok((input, Reference::Value(name)))
}

pub fn ref_div(input: &str) -> IResult<&str, Reference> {
    alt((ref_dir, ref_ind, ref_val))(input)
}