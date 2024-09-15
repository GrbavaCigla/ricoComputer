use strum_macros::EnumString;

use super::declaration::Reference;

#[derive(Debug, EnumString, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum InstructionName {
    ORG,
    STOP = 0b1111,
}

#[derive(Debug)]
pub struct Instruction {
    pub name: InstructionName,
    pub arg1: Option<Reference>,
    pub arg2: Option<Reference>,
    pub arg3: Option<Reference>,
}