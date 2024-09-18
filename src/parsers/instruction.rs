use nom::{
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::map_res,
    sequence::delimited,
    Parser,
};
use nom_supreme::tag::complete::tag_no_case;

use crate::types::{Error, IResult, Instruction, InstructionName, Reference};

use super::declaration::{ref_di, ref_div, ref_diva};

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

pub fn inst2<'a, F, R1, R2>(
    mut instr: F,
    mut arg1: R1,
    mut arg2: R2,
) -> impl FnMut(&'a str) -> IResult<Instruction>
where
    F: Parser<&'a str, InstructionName, Error<'a>>,
    R1: Parser<&'a str, Reference, Error<'a>>,
    R2: Parser<&'a str, Reference, Error<'a>>,
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

pub fn inst3<'a, F, R1, R2, R3>(
    mut instr: F,
    mut arg1: R1,
    mut arg2: R2,
    mut arg3: R3,
) -> impl FnMut(&'a str) -> IResult<Instruction>
where
    F: Parser<&'a str, InstructionName, Error<'a>>,
    R1: Parser<&'a str, Reference, Error<'a>>,
    R2: Parser<&'a str, Reference, Error<'a>>,
    R3: Parser<&'a str, Reference, Error<'a>>,
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
        // STOP
        inst3(inst_name("stop"), ref_di, ref_di, ref_di),
        inst2(inst_name("stop"), ref_di, ref_di),
        inst1(inst_name("stop"), ref_di),
        inst0(inst_name("stop")),
        
        // MOV
        inst2(inst_name("mov"), ref_di, ref_diva),

        // Arithmetic
        inst3(
            alt((
                inst_name("add"),
                inst_name("sub"),
                inst_name("mul"),
                inst_name("div"),
            )),
            ref_di,
            ref_di,
            ref_diva,
        ),
        inst3(
            alt((
                inst_name("add"),
                inst_name("sub"),
                inst_name("mul"),
                inst_name("div"),
            )),
            ref_di,
            ref_diva,
            ref_di,
        ),

        // I/O
        inst2(alt((inst_name("in"), inst_name("out"))), ref_di, ref_diva),
        inst1(alt((inst_name("in"), inst_name("out"))), ref_di),

        // Branch
        inst3(alt((inst_name("beq"), inst_name("bgt"))), ref_di, ref_div, ref_di),
        inst3(alt((inst_name("beq"), inst_name("bgt"))), ref_div, ref_di, ref_di)
    ))(input)
}
