use std::collections::HashMap;

mod common;
mod io;
mod mov;
mod stop;
mod arithmetic;
mod branch;

use crate::types::{Instruction, InstructionName, Word};
use crate::types::SyntaxTree;
use miette::Result;

pub fn assemble(syntax_tree: &SyntaxTree) -> Result<Vec<u8>> {
    let mut res = vec![
        // TODO: Use something else then unwrap, but this should never be None
        *syntax_tree
            .org
            .arg1
            .as_ref()
            .unwrap()
            .try_as_value_ref()
            .unwrap() as u8,
        0,
    ];

    let mut lookup_table: HashMap<&str, u16> = HashMap::new();
    for decl in syntax_tree.declarations.iter() {
        lookup_table.insert(&decl.symbol[..], decl.value);
    }

    for inst in syntax_tree.instructions.iter() {
        let dword = handle_instruction(&lookup_table, inst)?;
        res.push(dword.0 .0);
        res.push(dword.0 .1);
        if let Some(word) = dword.1 {
            res.push(word.0);
            res.push(word.1);
        }
    }

    Ok(res)
}

fn handle_instruction(lt: &HashMap<&str, u16>, inst: &Instruction) -> Result<(Word, Option<Word>)> {
    match inst.name {
        InstructionName::STOP => stop::handle_instruction(lt, inst),
        InstructionName::MOV => mov::handle_instruction(lt, inst),
        InstructionName::ADD
        | InstructionName::SUB
        | InstructionName::MUL
        | InstructionName::DIV => arithmetic::handle_instruction(lt, inst),
        InstructionName::BEQ | InstructionName::BGT => branch::handle_instruction(lt, inst),
        InstructionName::IN | InstructionName::OUT => io::handle_instruction(lt, inst),
        InstructionName::ORG => unreachable!(),
    }
}
