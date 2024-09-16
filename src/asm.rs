use std::collections::HashMap;

use crate::types::{Instruction, InstructionName, Word};
use crate::types::{Reference, SyntaxTree};
use miette::miette;
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
    for (i, decl) in syntax_tree.declarations.iter().enumerate() {
        lookup_table.insert(&decl.symbol[..], decl.value);
    }

    for inst in syntax_tree.instructions.iter() {
        let dword = handle_instruction(&lookup_table, inst)?;
        res.push(dword.0.0);
        res.push(dword.0.1);
        if let Some(word) = dword.1 {
            res.push(word.0);
            res.push(word.1);
        }
    }

    Ok(res)
}

fn handle_instruction(lt: &HashMap<&str, u16>, inst: &Instruction) -> Result<(Word, Option<Word>)> {
    match inst.name {
        InstructionName::STOP => Ok((
            get_word([
                inst.name as u8,
                convert_ref(lt, inst.arg1.as_ref())?,
                convert_ref(lt, inst.arg2.as_ref())?,
                convert_ref(lt, inst.arg3.as_ref())?,
            ]),
            None,
        )),
        InstructionName::ORG => unreachable!(),
    }
}

fn convert_ref(lt: &HashMap<&str, u16>, refer: Option<&Reference>) -> Result<u8> {
    // TODO: Add error for illegal addresses, currently it only cuts off the rest
    match refer {
        Some(r) => match r {
            Reference::Direct(s) => Ok((*lt
                .get(&s[..])
                .ok_or(miette!("Symbol \"{}\" not declared.", s))?
                & 0b111) as u8),
            Reference::Indirect(s) => Ok((*lt
                .get(&s[..])
                .ok_or(miette!("Symbol \"{}\" not declared.", s))?
                & 0b111
                | 0b1000) as u8),
            Reference::Addr(_) => todo!(),
            Reference::Value(_) => todo!(),
        },
        None => Ok(0b0000),
    }
}

fn get_word(mut word: [u8; 4]) -> Word {
    for part in word.iter_mut() {
        *part &= 0b1111;
    }

    Word {
        0: (word[2] << 4) | word[3],
        1: (word[0] << 4) | word[1],
    }
}
