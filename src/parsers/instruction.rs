use std::num::ParseIntError;

use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    error::{ErrorKind, FromExternalError, ParseError},
    sequence::delimited,
    Err, IResult, Parser,
};
use nom_supreme::tag::{complete::tag_no_case, TagError};
use strum::ParseError as StrumParseError;

use crate::types::{Instruction, InstructionName, Reference};

use super::declaration::ref_div;

pub fn inst_name<'a, E>(
    name: &'a str,
) -> impl FnMut(&'a str) -> IResult<&'a str, InstructionName, E>
where
    E: FromExternalError<&'a str, StrumParseError>
        + ParseError<&'a str>
        + TagError<&'a str, &'a str>,
{
    move |input: &'a str| {
        let (input, name) = tag_no_case(name).parse(input)?;
        match str::parse(name) {
            Ok(o2) => Ok((input, o2)),
            Err(e) => Err(Err::Error(E::from_external_error(
                input,
                ErrorKind::MapRes,
                e,
            ))),
        }
    }
}

pub fn inst0<'a, E, F>(mut instr: F) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, InstructionName, E>,
{
    move |input: &'a str| {
        let (input, instr_name) = instr.parse(input)?;

        Ok((
            input,
            Instruction {
                name: instr_name,
                arg1: None,
                arg2: None,
                arg3: None,
            },
        ))
    }
}

pub fn inst1<'a, E, F, R>(
    mut instr: F,
    mut arg1: R,
) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, InstructionName, E>,
    R: Parser<&'a str, Reference, E>,
{
    move |input: &'a str| {
        let (input, instr_name) = instr.parse(input)?;
        let (input, _) = space1.parse(input)?;
        let (input, ref1) = arg1.parse(input)?;

        Ok((
            input,
            Instruction {
                name: instr_name,
                arg1: Some(ref1),
                arg2: None,
                arg3: None,
            },
        ))
    }
}

pub fn inst2<'a, E, F, R>(
    mut instr: F,
    mut arg1: R,
    mut arg2: R,
) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, InstructionName, E>,
    R: Parser<&'a str, Reference, E>,
{
    move |input: &'a str| {
        let (input, instr_name) = instr.parse(input)?;
        let (input, _) = space1.parse(input)?;
        let (input, ref1) = arg1.parse(input)?;

        // TODO: Replace this with ws
        let (input, _) = delimited(space0, char(','), space0).parse(input)?;
        let (input, ref2) = arg2.parse(input)?;

        Ok((
            input,
            Instruction {
                name: instr_name,
                arg1: Some(ref1),
                arg2: Some(ref2),
                arg3: None,
            },
        ))
    }
}

pub fn inst3<'a, E, F, R>(
    mut instr: F,
    mut arg1: R,
    mut arg2: R,
    mut arg3: R,
) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, InstructionName, E>,
    R: Parser<&'a str, Reference, E>,
{
    move |input: &'a str| {
        let (input, instr_name) = instr.parse(input)?;
        let (input, _) = space1.parse(input)?;
        let (input, ref1) = arg1.parse(input)?;

        // TODO: Replace this with ws
        let (input, _) = delimited(space0, char(','), space0).parse(input)?;
        let (input, ref2) = arg2.parse(input)?;

        // TODO: Replace this with ws
        let (input, _) = delimited(space0, char(','), space0).parse(input)?;
        let (input, ref3) = arg3.parse(input)?;

        Ok((
            input,
            Instruction {
                name: instr_name,
                arg1: Some(ref1),
                arg2: Some(ref2),
                arg3: Some(ref3),
            },
        ))
    }
}

pub fn inst<'a, E>(input: &'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str>
        + FromExternalError<&'a str, StrumParseError>
        + FromExternalError<&'a str, ParseIntError>
        + TagError<&'a str, &'a str>,
{
    alt((
        inst0(inst_name("org")),
        inst0(inst_name("stop")),
        inst1(inst_name("stop"), ref_div),
        inst2(inst_name("stop"), ref_div, ref_div),
        inst3(inst_name("stop"), ref_div, ref_div, ref_div),
    ))(input)
}
