use strum_macros::EnumString;

use super::declaration::Reference;

#[derive(Debug, EnumString, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum InstructionName {
    MOV = 0b0000,
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
