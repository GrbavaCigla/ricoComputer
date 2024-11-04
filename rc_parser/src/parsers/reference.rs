use std::num::Wrapping;

use nom::{
    branch::alt,
    character::complete::char,
    sequence::{delimited, preceded},
};

use rc_common::{IResult, Reference};
use super::common::{number, sym};

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

pub fn ref_di<'a>(input: &'a str) -> IResult<Reference> {
    alt((ref_dir, ref_ind))(input)
}

pub fn ref_diva<'a>(input: &'a str) -> IResult<Reference> {
    alt((ref_dir, ref_ind, ref_val, ref_addr))(input)
}

pub fn ref_div<'a>(input: &'a str) -> IResult<Reference> {
    alt((ref_dir, ref_ind, ref_val))(input)
}
