use std::collections::HashMap;

use rc_common::{Instruction, Reference, Word};

use miette::Result;

use crate::utils::{convert_ref, get_symbol};

// TODO: Support null arguments
pub fn handle_instruction(
    lt: &HashMap<&str, u16>,
    inst: &Instruction,
) -> Result<(Word, Option<Word>)> {
    Ok((
        Word::from([
            inst.name as u8,
            convert_ref(lt, inst.arg1.as_ref())?,
            convert_ref(lt, inst.arg2.as_ref())?,
            convert_ref(lt, inst.arg3.as_ref())? & match inst.arg3.as_ref().unwrap() {
                Reference::Direct(_) => 0b1000,
                Reference::Indirect(_) => 0b0111,
                _ => unreachable!()
            },
        ]),
        match inst.arg3.as_ref().unwrap() {
            Reference::Direct(d) => Some(Word::from(get_symbol(lt, d)?)),
            Reference::Indirect(_) => None,
            _ => unreachable!()
        },
    ))
}
