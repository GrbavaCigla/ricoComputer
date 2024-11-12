pub mod stop;
pub mod out;
// pub mod r#in;

use num_traits::FromPrimitive;
use rc_common::{InstructionName, Word};

use crate::VMState;

pub fn run_instruction(memory: &[u16; 2_usize.pow(16)], instr_word: Word, const_word: Option<Word>) -> (VMState, u16) {
    let word_bytes: [u8; 4] = instr_word.into();

    match FromPrimitive::from_u8(word_bytes[0]) {
        Some(InstructionName::MOV) => unimplemented!(),
        Some(InstructionName::ADD) => unimplemented!(),
        Some(InstructionName::SUB) => unimplemented!(),
        Some(InstructionName::MUL) => unimplemented!(),
        Some(InstructionName::DIV) => unimplemented!(),
        Some(InstructionName::BEQ) => unimplemented!(),
        Some(InstructionName::BGT) => unimplemented!(),
        Some(InstructionName::IN) => unimplemented!(),
        // Some(InstructionName::IN) => r#in::run_instruction(memory, &word_bytes),
        Some(InstructionName::OUT) => out::run_instruction(memory, &word_bytes),
        Some(InstructionName::JSR) => unimplemented!(),
        Some(InstructionName::RTS) => unimplemented!(),
        Some(InstructionName::STOP) => stop::run_instruction(memory, &word_bytes),
        
        // TODO: Match arithmetic const instructions.
        None => todo!(),
        
        // TODO: Raise an exception
        Some(_) => unreachable!(),
    }
}
