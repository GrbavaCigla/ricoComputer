use std::collections::HashMap;

use crate::types::{Instruction, Word};

use miette::Result;

pub fn handle_instruction(
    lt: &HashMap<&str, u16>,
    inst: &Instruction,
) -> Result<(Word, Option<Word>)> {
    todo!()
}
