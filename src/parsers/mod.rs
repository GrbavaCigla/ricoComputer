use nom::{bytes::complete::{take_till, take_until}, character::complete::{line_ending, not_line_ending}, IResult};

use crate::types::SyntaxTree;

pub mod primitives;
pub mod symbol;

pub fn parse(input: &str) -> IResult<&str, SyntaxTree> {
    println!("{:#?}", not_line_ending(input)?);

    Ok((
        "",
        SyntaxTree {
            declarations: vec![],
        },
    ))
}
