use strum_macros::{Display, EnumString};

use super::declaration::Reference;

#[derive(Debug, EnumString, Clone, Display)]
#[strum(ascii_case_insensitive)]
pub enum InstructionName {
    STOP,
    ORG,
}

#[derive(Debug)]
pub struct Instruction {
    pub name: InstructionName,
    pub arg1: Option<Reference>,
    pub arg2: Option<Reference>,
    pub arg3: Option<Reference>,
}