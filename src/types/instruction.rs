use std::str::FromStr;

use strum_macros::{Display, EnumString};

use super::declaration::Reference;

#[derive(Debug, EnumString, Clone, Display)]
pub enum InstructionName {
    STOP,
}

impl InstructionName {
    pub fn parse<F: FromStr>(str: &str) -> Result<F, F::Err> {
        FromStr::from_str(str)
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub name: InstructionName,
    pub arg1: Option<Reference>,
    pub arg2: Option<Reference>,
    pub arg3: Option<Reference>,
}