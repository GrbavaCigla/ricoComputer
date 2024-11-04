use std::collections::HashMap;

use rc_common::{Instruction, Reference, Word};

use crate::utils::{convert_ref, get_symbol};

use miette::Result;

pub fn handle_instruction(
    lt: &HashMap<&str, u16>,
    inst: &Instruction,
) -> Result<(Word, Option<Word>)> {
    Ok((
        Word::from([
            inst.name as u8,
            convert_ref(lt, inst.arg1.as_ref())?,
            convert_ref(lt, inst.arg2.as_ref())?,
            match inst.arg2.as_ref().unwrap() {
                Reference::Direct(_) | Reference::Indirect(_) => 0b0000,
                Reference::Addr(_) | Reference::Value(_) => 0b1000,
            },
        ]),
        match inst.arg2.as_ref().unwrap() {
            Reference::Direct(_) | Reference::Indirect(_) => None,
            Reference::Value(v) => Some(Word::from(*v)),
            Reference::Addr(s) => Some(Word::from(get_symbol(lt, &s[..])?)),
        },
    ))
}
