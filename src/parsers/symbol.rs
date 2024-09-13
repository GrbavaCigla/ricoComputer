use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, char, multispace0},
    combinator::recognize,
    multi::many0_count,
    sequence::{pair, preceded, separated_pair, terminated},
    IResult,
};

use crate::types::Declaration;

use super::primitives::number;

pub fn symbol(input: &str) -> IResult<&str, &str> {
    recognize(pair(
        alt((alpha1, tag("_"))),
        many0_count(alt((alphanumeric1, tag("_")))),
    ))(input)
}

pub fn declaration(input: &str) -> IResult<&str, Declaration> {
    let (input, (symbol, value)) = separated_pair(
        terminated(symbol, multispace0),
        char('='),
        preceded(multispace0, number),
    )(input)?;

    Ok((
        input,
        Declaration {
            symbol: symbol.to_string(),
            value: value,
        },
    ))
}
