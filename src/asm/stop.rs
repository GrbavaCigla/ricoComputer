use std::collections::HashMap;

use crate::types::{Instruction, Word};

use super::common::convert_ref;

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
            convert_ref(lt, inst.arg3.as_ref())?,
        ]),
        None,
    ))
}
