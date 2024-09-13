use super::{
    common::ws,
    declaration::{decl, ref_dir, ref_val},
    instruction::{inst0, inst1, inst_name},
};
use crate::{parsers::common::mws, types::{Reference, SyntaxTree}};
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::complete::{alpha0, line_ending, multispace0, multispace1},
    combinator::{all_consuming, peek},
    multi::{many_till, separated_list0},
    sequence::{delimited, pair, terminated, tuple},
    IResult,
};

pub fn parse(input: &str) -> IResult<&str, SyntaxTree> {

    let decl_line = terminated(ws(decl), line_ending);
    let mut decl_part = many_till(mws(decl_line), peek(tag_no_case("org")));
    println!("{:?}", decl_part(input));
    // let (input, declarations) = all_consuming(tuple((decl_parser)))(input)?;

    Ok((
        "",
        SyntaxTree {
            declarations: vec![],
        },
    ))
}
