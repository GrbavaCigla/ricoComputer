use std::str::FromStr;

use super::{
    common::{empty_line, mws, ws},
    declaration::{decl, ref_dir, ref_val},
    instruction::{inst, inst0, inst1, inst_name},
};
use crate::types::{Instruction, InstructionName, Reference, SyntaxTree};
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case},
    character::complete::{alpha0, line_ending, multispace0, multispace1},
    combinator::{all_consuming, eof, map_res, opt, peek, recognize},
    multi::{many0, many1, many_till, separated_list0},
    sequence::{delimited, pair, terminated, tuple},
    IResult, Parser,
};

pub fn parse(input: &str) -> IResult<&str, SyntaxTree> {
    let org_line = terminated(
        ws(inst1(map_res(tag_no_case("org"), str::parse), ref_val)),
        line_ending,
    );
    let org_part = empty_line(mws(org_line));

    let decl_line = terminated(ws(decl), empty_line(line_ending));
    let decl_part = mws(many0(decl_line));

    let instr_line = terminated(ws(inst), empty_line(alt((line_ending, eof))));
    let instr_part = mws(many1(instr_line));

    let mut syntax = all_consuming(tuple((decl_part, org_part, instr_part)));

    let (input, st_raw) = syntax(input)?;

    Ok((
        input,
        SyntaxTree {
            declarations: st_raw.0,
            org: st_raw.1,
            instructions: st_raw.2,
        },
    ))
}
