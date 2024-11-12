pub mod stop;

use num_traits::FromPrimitive;
use rc_common::{InstructionName, Word};

use crate::VMState;

pub fn run_instruction(memory: &[u16; 2_usize.pow(16)], instr_word: Word, const_word: Option<Word>) -> (VMState, u16) {
    let word_bytes: [u8; 4] = instr_word.into();

    match FromPrimitive::from_u8(word_bytes[0]) {
        // TODO: Match arithmetic const instructions.
        Some(InstructionName::MOV) => unimplemented!(),
        Some(InstructionName::ADD) => unimplemented!(),
        Some(InstructionName::SUB) => unimplemented!(),
        Some(InstructionName::MUL) => unimplemented!(),
        Some(InstructionName::DIV) => unimplemented!(),
        Some(InstructionName::BEQ) => unimplemented!(),
        Some(InstructionName::BGT) => unimplemented!(),
        Some(InstructionName::IN) => unimplemented!(),
        Some(InstructionName::OUT) => unimplemented!(),
        Some(InstructionName::JSR) => unimplemented!(),
        Some(InstructionName::RTS) => unimplemented!(),
        Some(InstructionName::STOP) => stop::run_instruction(memory, &word_bytes),

        // TODO: Raise an exception
        Some(_) => unreachable!(),
        None => unreachable!(),
    }
}
