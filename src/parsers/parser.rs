use std::num::ParseIntError;

use super::{
    common::{empty_line, mws, ws},
    declaration::{decl, ref_val},
    instruction::{inst, inst1},
};
use crate::types::SyntaxTree;
use nom::{
    branch::alt,
    character::complete::line_ending,
    combinator::{all_consuming, eof, map_res},
    error::{FromExternalError, ParseError},
    multi::{many0, many1},
    sequence::{terminated, tuple},
    IResult,
};
use nom_supreme::tag::{complete::tag_no_case, TagError};

use strum::ParseError as StrumParseError;

pub fn parse<'a, E>(input: &'a str) -> IResult<&'a str, SyntaxTree, E>
where
    E: ParseError<&'a str>
        + FromExternalError<&'a str, ParseIntError>
        + FromExternalError<&'a str, StrumParseError>
        + TagError<&'a str, &'a str>
        + 'a,
{
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
