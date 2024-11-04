pub mod instructions;
mod utils;

use std::collections::HashMap;

use instructions::handle_instruction;
use rc_common::{Reference, SyntaxTree};
use miette::{miette, Result};

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

    let start_address = match &syntax_tree.org.arg1 {
        Some(_r @ Reference::Value(v)) => *v,
        None | Some(_) => return Err(miette!("Cannot get start address from ORG instruction.")),
    };

    for (i, inst) in syntax_tree.instructions.iter().enumerate() {
        if let Some(l) = inst.0 {
            lookup_table.insert(l, i as u16 * 2 + start_address);
        }
    }

    for inst in syntax_tree.instructions.iter() {
        let dword = handle_instruction(&lookup_table, &inst.1)?;
        res.push(dword.0 .0);
        res.push(dword.0 .1);
        if let Some(word) = dword.1 {
            res.push(word.0);
            res.push(word.1);
        }
    }

    Ok(res)
}