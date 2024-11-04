use std::collections::HashMap;

use rc_common::{Instruction, Reference, Word};

use miette::Result;

use crate::utils::get_symbol;

pub fn handle_instruction(
    lt: &HashMap<&str, u16>,
    inst: &Instruction,
) -> Result<(Word, Option<Word>)> {
    // TODO: Check this clone
    Ok((
        Word::from([inst.name as u8, 0, 0, 0]),
        Some(Word::from(get_symbol(lt, &match inst.arg1.clone().unwrap() {
            Reference::Direct(s) => s,
            _ => unreachable!()
        })?)),
    ))
}
