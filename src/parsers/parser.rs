use super::{
    common::{cmws, empty_line, eofl, mws, opt_comment_end, rm_bom, ws},
    declaration::{decl, ref_val},
    instruction::{inst, inst1, label},
};
use crate::types::{Error, SyntaxTree};
use nom::{
    character::complete::line_ending, combinator::{map_res, opt}, multi::{many0, many1}, sequence::{pair, terminated, tuple}
};
use nom_supreme::{final_parser::final_parser, tag::complete::tag_no_case};

pub fn parse(input: &str) -> Result<SyntaxTree, Error> {
    let label_prefix = mws(label);

    let org_line = terminated(
        ws(inst1(map_res(tag_no_case("org"), str::parse), ref_val)),
        opt_comment_end(line_ending),
    );
    let org_part = cmws(org_line, line_ending);

    let decl_line = terminated(ws(decl), empty_line(opt_comment_end(line_ending), line_ending));
    let decl_part = cmws(many0(decl_line), line_ending);

    // TODO: Add so you can add empty labels at the bottom
    let instr_line = terminated(ws(inst), empty_line(opt_comment_end(eofl), eofl));
    let instr_part = cmws(many1(pair(opt(label_prefix), instr_line)), eofl);

    let mut syntax = final_parser(rm_bom(tuple((decl_part, org_part, instr_part))));

    let st_raw = syntax(input)?;

    Ok(SyntaxTree {
        declarations: st_raw.0,
        org: st_raw.1,
        instructions: st_raw.2,
    })
}
