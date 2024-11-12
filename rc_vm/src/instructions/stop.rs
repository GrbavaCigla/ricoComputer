
use crate::{utils::get_address, VMState};

pub fn run_instruction(memory: &[u16; 2_usize.pow(16)], instr_bytes: &[u8; 4]) -> (VMState, u16) {
    let msg = instr_bytes[1..]
        .iter()
        .filter(|x| get_address(memory, **x) != 0)
        .map(|x| x.to_string())
        .reduce(|acc, e| format!("{} {}", acc, e));

    match msg {
        Some(s) => println!("Program stopped with the following values: {}", s),
        None => println!("Program stopped."),
    }

    (VMState::STOPPED, 1)
}
