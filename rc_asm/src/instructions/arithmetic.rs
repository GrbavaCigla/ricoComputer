use std::collections::HashMap;

use crate::utils::{convert_ref, get_symbol};

use rc_common::{Instruction, Reference, Word};

use miette::Result;

// TODO: There is a better way to handle Arithmetic instruction types, but I am to lazy to refactor it now
pub fn handle_instruction(
    lt: &HashMap<&str, u16>,
    inst: &Instruction,
) -> Result<(Word, Option<Word>)> {
    Ok((
        Word::from([
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
    ))
}
