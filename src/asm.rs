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
        InstructionName::STOP => Ok((
            Word::from([
                inst.name as u8,
                convert_ref(lt, inst.arg1.as_ref())?,
                convert_ref(lt, inst.arg2.as_ref())?,
                convert_ref(lt, inst.arg3.as_ref())?,
            ]),
            None,
        )),
        InstructionName::MOV => Ok((
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
        )),
        InstructionName::ADD
        | InstructionName::SUB
        | InstructionName::MUL
        | InstructionName::DIV => Ok((
            Word::from([
                // (inst.name as u8),
                match (inst.arg2.as_ref().unwrap(), inst.arg3.as_ref().unwrap()) {
                    (Reference::Value(_), Reference::Direct(_) | Reference::Indirect(_))
                    | (Reference::Direct(_) | Reference::Indirect(_), Reference::Value(_))
                    | (Reference::Addr(_), Reference::Direct(_) | Reference::Indirect(_))
                    | (Reference::Direct(_) | Reference::Indirect(_), Reference::Addr(_)) => {
                        0b1000 | inst.name as u8
                    }
                    _ => inst.name as u8,
                },
                convert_ref(lt, inst.arg1.as_ref())?,
                convert_ref(lt, inst.arg2.as_ref())?,
                convert_ref(lt, inst.arg3.as_ref())?,
            ]),
            match (inst.arg2.as_ref().unwrap(), inst.arg3.as_ref().unwrap()) {
                (Reference::Value(v), Reference::Direct(_) | Reference::Indirect(_))
                | (Reference::Direct(_) | Reference::Indirect(_), Reference::Value(v)) => {
                    Some(Word::from(*v))
                }
                (Reference::Addr(s), Reference::Direct(_) | Reference::Indirect(_))
                | (Reference::Direct(_) | Reference::Indirect(_), Reference::Addr(s)) => {
                    Some(Word::from(get_symbol(lt, &s[..])?))
                }
                _ => None,
            },
        )),
        InstructionName::ORG => unreachable!(),
    }
}

fn convert_ref(lt: &HashMap<&str, u16>, refer: Option<&Reference>) -> Result<u8> {
    // TODO: Add error for illegal addresses, currently it only cuts off the rest
    match refer {
        Some(r) => match r {
            Reference::Direct(s) => Ok((get_symbol(lt, &s[..])? & 0b111) as u8),
            Reference::Indirect(s) => Ok((get_symbol(lt, &s[..])? & 0b111 | 0b1000) as u8),
            Reference::Addr(_) | Reference::Value(_) => Ok(0b0000),
        },
        None => Ok(0b0000),
    }
}

fn get_symbol(lt: &HashMap<&str, u16>, name: &str) -> Result<u16> {
    Ok(*lt
        .get(name)
        .ok_or(miette!("Symbol \"{}\" not declared.", name))?)
}
