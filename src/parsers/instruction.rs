use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::map_res,
    sequence::delimited,
    Parser,
};
use nom_supreme::tag::complete::tag_no_case;

use super::declaration::ref_div;
use crate::types::{Error, IResult, Instruction, InstructionName, Reference};

pub fn inst_name<'a>(name: &'static str) -> impl FnMut(&'a str) -> IResult<InstructionName> {
    move |input: &'a str| map_res(tag_no_case(name), str::parse).parse(input)
}

pub fn inst0<'a, F>(mut instr: F) -> impl FnMut(&'a str) -> IResult<Instruction>
where
    F: Parser<&'a str, InstructionName, Error<'a>>,
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

pub fn inst1<'a, F, R>(mut instr: F, mut arg1: R) -> impl FnMut(&'a str) -> IResult<Instruction>
where
    F: Parser<&'a str, InstructionName, Error<'a>>,
    R: Parser<&'a str, Reference, Error<'a>>,
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

pub fn inst2<'a, F, R>(
    mut instr: F,
    mut arg1: R,
    mut arg2: R,
) -> impl FnMut(&'a str) -> IResult<Instruction>
where
    F: Parser<&'a str, InstructionName, Error<'a>>,
    R: Parser<&'a str, Reference, Error<'a>>,
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

pub fn inst3<'a, F, R>(
    mut instr: F,
    mut arg1: R,
    mut arg2: R,
    mut arg3: R,
) -> impl FnMut(&'a str) -> IResult<Instruction>
where
    F: Parser<&'a str, InstructionName, Error<'a>>,
    R: Parser<&'a str, Reference, Error<'a>>,
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

pub fn inst<'a>(input: &'a str) -> IResult<Instruction> {
    alt((
        inst0(inst_name("org")),
        inst0(inst_name("stop")),
        inst1(inst_name("stop"), ref_div),
        inst2(inst_name("stop"), ref_div, ref_div),
        inst3(inst_name("stop"), ref_div, ref_div, ref_div),
    ))(input)
}
