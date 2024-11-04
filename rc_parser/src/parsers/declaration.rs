use std::num::Wrapping;

use nom::{
    character::complete::{char, space0},
    sequence::{preceded, separated_pair, terminated},
};
use rc_common::{Declaration, IResult};

use super::common::{sym, number};

pub fn decl<'a>(input: &'a str) -> IResult<Declaration> {
    let (input, (symbol, value)) =
        separated_pair(terminated(sym, space0), char('='), preceded(space0, number))(input)?;

    Ok((
        input,
        Declaration {
            symbol: symbol.to_string(),
            value: match value.1 {
                true => (Wrapping(0_u16) - Wrapping(value.0)).0,
                false => value.0,
            },
        },
    ))
}
