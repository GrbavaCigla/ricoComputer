use std::collections::HashMap;

use crate::types::Reference;
use miette::{miette, Result};


pub fn convert_ref(lt: &HashMap<&str, u16>, refer: Option<&Reference>) -> Result<u8> {
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

pub fn get_symbol(lt: &HashMap<&str, u16>, name: &str) -> Result<u16> {
    Ok(*lt
        .get(name)
        .ok_or(miette!("Symbol \"{}\" not declared.", name))?)
}
