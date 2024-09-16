use strum_macros::EnumString;

use super::declaration::Reference;

#[derive(Debug, EnumString, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum InstructionName {
    MOV = 0b0000,
    ADD = 0b0001,
    SUB = 0b0010,
    MUL = 0b0011,
    DIV = 0b0100,
    STOP = 0b1111,
    ORG,
}

#[derive(Debug)]
pub struct Instruction {
    pub name: InstructionName,
    pub arg1: Option<Reference>,
    pub arg2: Option<Reference>,
    pub arg3: Option<Reference>,
}
