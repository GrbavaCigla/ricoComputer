use std::str::FromStr;

use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{char, space0, space1},
    combinator::{cond, map_res},
    error::ParseError,
    sequence::{delimited, preceded, separated_pair},
    IResult, Parser,
};

use crate::types::{Instruction, InstructionName, Reference};

use super::common::ws;

pub fn inst_name(input: &str) -> IResult<&str, InstructionName> {
    let instruction_names = (tag_no_case("org"), tag_no_case("stop"));

    map_res(alt(instruction_names), str::parse)(input)
}

pub fn inst0<'a, E, F>(
    mut instr: F,
) -> impl FnMut(&'a str) -> IResult<&'a str, Instruction, E>
where
    E: ParseError<&'a str>,
    F: Parser<&'a str, InstructionName, E>,
{
    move |input: &'a str| {
        let (input, instr_name) = instr.parse(input)?;

        Ok((input, Instruction {
            name: instr_name,
            arg1: None,
            arg2: None,
            arg3: None
        }))
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

        Ok((input, Instruction {
            name: instr_name,
            arg1: Some(ref1),
            arg2: None,
            arg3: None
        }))
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
    R: Parser<&'a str, Reference, E> + Clone,
{
    move |input: &'a str| {
        let (input, instr_name) = instr.parse(input)?;
        let (input, _) = space1.parse(input)?;
        let (input, ref1) = arg1.parse(input)?;

        // TODO: Replace this with ws
        let (input, _) = delimited(space0, char(','), space0).parse(input)?;
        let (input, ref2) = arg2.parse(input)?;

        Ok((input, Instruction {
            name: instr_name,
            arg1: Some(ref1),
            arg2: Some(ref2),
            arg3: None
        }))
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
    R: Parser<&'a str, Reference, E> + Clone,
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

        Ok((input, Instruction {
            name: instr_name,
            arg1: Some(ref1),
            arg2: Some(ref2),
            arg3: Some(ref3)
        }))
    }
}

