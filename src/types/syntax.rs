use super::{Declaration, Instruction, Label};

// TODO: Recheck type for value
#[derive(Debug)]
pub struct SyntaxTree {
    pub org: Instruction,
    pub declarations: Vec<Declaration>,
    pub instructions: Vec<Instruction>,
    pub labels: Vec<Label>
}