use super::{
    common::{empty_line, mws, rm_bom, ws},
    declaration::{decl, ref_val},
    instruction::{inst, inst1},
};
use crate::types::{Error, SyntaxTree};
use nom::{
    branch::alt,
    character::complete::line_ending,
    combinator::{eof, map_res},
    multi::{many0, many1},
    sequence::{terminated, tuple},
};
use nom_supreme::{final_parser::final_parser, tag::complete::tag_no_case};

pub fn parse(input: &str) -> Result<SyntaxTree, Error> {
    let org_line = terminated(
        ws(inst1(map_res(tag_no_case("org"), str::parse), ref_val)),
        line_ending,
    );
    let org_part = empty_line(mws(org_line));

    let decl_line = terminated(ws(decl), empty_line(line_ending));
    let decl_part = mws(many0(decl_line));

    let instr_line = terminated(ws(inst), empty_line(alt((line_ending, eof))));
    let instr_part = mws(many1(instr_line));

    let mut syntax = final_parser(rm_bom(tuple((decl_part, org_part, instr_part))));

    let st_raw = syntax(input)?;

    Ok(SyntaxTree {
        declarations: st_raw.0,
        org: st_raw.1,
        instructions: st_raw.2,
        labels: vec![]
    })
}
