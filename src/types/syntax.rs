use super::{Declaration, Instruction};

// TODO: Recheck type for value
#[derive(Debug)]
pub struct SyntaxTree<'a> {
    pub org: Instruction,
    pub declarations: Vec<Declaration>,
    pub instructions: Vec<(Option<&'a str>, Instruction)>,
}