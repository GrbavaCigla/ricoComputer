use num_derive::{FromPrimitive, ToPrimitive};
use strum_macros::EnumString;

use super::Reference;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString, FromPrimitive, ToPrimitive)]
#[strum(ascii_case_insensitive)]
pub enum InstructionName {
    MOV = 0b0000,
    ADD = 0b0001,
    SUB = 0b0010,
    MUL = 0b0011,
    DIV = 0b0100,
    BEQ = 0b0101,
    BGT = 0b0110,
    IN = 0b0111,
    OUT = 0b1000,
    JSR = 0b1101,
    RTS = 0b1110,
    STOP = 0b1111,
    ORG,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    pub name: InstructionName,
    pub arg1: Option<Reference>,
    pub arg2: Option<Reference>,
    pub arg3: Option<Reference>,
}
