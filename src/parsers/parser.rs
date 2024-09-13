use super::{
    common::ws,
    declaration::{decl, ref_dir, ref_val},
    instruction::{inst0, inst1, inst_name},
};
use crate::{
    parsers::common::mws,
    types::{InstructionName, Reference, SyntaxTree},
};
use nom::{
    branch::alt, bytes::complete::tag_no_case, character::complete::{alpha0, line_ending, multispace0, multispace1}, combinator::{all_consuming, map_res, peek}, complete::tag, multi::{many_till, separated_list0}, sequence::{delimited, pair, terminated, tuple}, IResult, Parser
};

pub fn parse(input: &str) -> IResult<&str, SyntaxTree> {
    // let org_inst = inst1(map_res(tag_no_case("org"), InstructionName::parse), ref_val);

    // let decl_line = terminated(ws(decl), line_ending);
    // // let instr_line = ws(inst0);
    
    // let mut syntax = many_till(mws(decl_line), org_inst);

    // println!("{:?}", syntax(input));
    // println!("{:#?}", inst1(map_res(tag_no_case("org"), InstructionName::parse), ref_val)("ORG 3"));
    println!("{:#?}", inst0(map_res(tag("org"), InstructionName::parse)).parse("org")?);
    // let (input, declarations) = all_consuming(tuple((decl_parser)))(input)?;

    Ok((
        "",
        SyntaxTree {
            declarations: vec![],
        },
    ))
}
