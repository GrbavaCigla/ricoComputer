use std::num::Wrapping;

use nom::{
    branch::alt,
    character::complete::{alpha1, alphanumeric1, char, space0},
    combinator::recognize,
    multi::many0_count,
    sequence::{delimited, pair, preceded, separated_pair, terminated},
};
use nom_supreme::tag::complete::tag;

use crate::types::{Declaration, IResult, Reference};

use super::common::number;

pub fn sym<'a>(input: &'a str) -> IResult<&'a str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn decl<'a>(input: &'a str) -> IResult<Declaration> {
    let (input, (symbol, value)) =
        separated_pair(terminated(sym, space0), char('='), preceded(space0, number))(input)?;

    Ok((
        input,
        Declaration {
            symbol: symbol.to_string(),
            value: match value.1 {
                true => (Wrapping(0_u16) - Wrapping(value.0)).0,
                false => value.0
            }
        },
    ))
}

pub fn ref_dir<'a>(input: &'a str) -> IResult<Reference> {
    let (input, name) = sym(input)?;
    Ok((input, Reference::Direct(name.to_string())))
}

pub fn ref_ind<'a>(input: &'a str) -> IResult<Reference> {
    let (input, name) = delimited(char('('), sym, char(')'))(input)?;
    Ok((input, Reference::Indirect(name.to_string())))
}

pub fn ref_addr<'a>(input: &'a str) -> IResult<Reference> {
    let (input, name) = preceded(char('#'), sym)(input)?;
    Ok((input, Reference::Addr(name.to_string())))
}

pub fn ref_val<'a>(input: &'a str) -> IResult<Reference> {
    let (input, val) = number(input)?;
    Ok((
        input,
        Reference::Value(match val.1 {
            true => (Wrapping(0_u16) - Wrapping(val.0)).0,
            false => val.0,
        }),
    ))
}

pub fn ref_div<'a>(input: &'a str) -> IResult<Reference> {
    alt((ref_dir, ref_ind, ref_val))(input)
}

pub fn ref_di<'a>(input: &'a str) -> IResult<Reference> {
    alt((ref_dir, ref_ind))(input)
}

pub fn ref_diva<'a>(input: &'a str) -> IResult<Reference> {
    alt((ref_dir, ref_ind, ref_val, ref_addr))(input)
}
