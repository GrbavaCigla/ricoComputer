use std::collections::HashMap;

use crate::types::{Instruction, Reference, Word};
use miette::Result;

use super::common::{convert_ref, get_symbol};


pub fn handle_instruction(
    lt: &HashMap<&str, u16>,
    inst: &Instruction,
) -> Result<(Word, Option<Word>)> {
    let inst_ref = inst.arg2.as_ref().unwrap_or(&Reference::Value(1));
    let half = match inst_ref {
        Reference::Direct(_) | Reference::Indirect(_) => {
            (convert_ref(lt, Some(inst_ref))? | 0b1000_0000) & 0b1000_1111
        }
        Reference::Value(v) => 0b0111_1111 & *v as u8,
        Reference::Addr(s) => 0b0111_1111 & get_symbol(lt, s)? as u8,
    };

    Ok((
        Word::from([
            inst.name as u8,
            convert_ref(lt, inst.arg1.as_ref())?,
            0b1111 & (half >> 4) as u8,
            0b1111 & half as u8,
        ]),
        None,
    ))
}
