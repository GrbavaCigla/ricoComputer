use crate::{utils::get_address, VMState};

pub fn run_instruction(memory: &[u16; 2_usize.pow(16)], instr_bytes: &[u8; 4]) -> (VMState, u16) {
    let arg: u16 = get_address(memory, instr_bytes[1]);
    let len: u16;

    if instr_bytes[2] == 0b1000 {
        len = memory[get_address(memory, instr_bytes[3]) as usize];
    } else if instr_bytes[2] >> 3 & 1 == 0 {
        len = ((instr_bytes[2] & 0b111) << 4) as u16 + instr_bytes[3] as u16;
    } else {
        todo!("Raise an error because instruction is invalid");
    }

    for (i, val) in memory[arg as usize..(arg + len) as usize]
        .iter()
        .enumerate()
    {
        println!("Output at address {}: {}", arg as usize + i, val);
    }

    (VMState::RUNNING, 1)
}
