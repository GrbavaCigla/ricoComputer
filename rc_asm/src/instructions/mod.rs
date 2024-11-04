pub mod arithmetic;
pub mod branch;
pub mod io;
pub mod mov;
pub mod stop;
pub mod subroutine;

use std::collections::HashMap;
use rc_common::{Instruction, InstructionName, Word};
use miette::Result;

pub fn handle_instruction(lt: &HashMap<&str, u16>, inst: &Instruction) -> Result<(Word, Option<Word>)> {
    match inst.name {
        InstructionName::STOP => stop::handle_instruction(lt, inst),
        InstructionName::MOV => mov::handle_instruction(lt, inst),
        InstructionName::ADD
        | InstructionName::SUB
        | InstructionName::MUL
        | InstructionName::DIV => arithmetic::handle_instruction(lt, inst),
        InstructionName::BEQ | InstructionName::BGT => branch::handle_instruction(lt, inst),
        InstructionName::IN | InstructionName::OUT => io::handle_instruction(lt, inst),
        InstructionName::JSR => subroutine::handle_instruction(lt, inst),
        InstructionName::RTS => Ok((Word::from([inst.name as u8, 0, 0, 0]), None)),
        InstructionName::ORG => unreachable!(),
    }
}
