mod declaration;
mod instruction;
mod syntax;

pub use declaration::{Declaration, Reference};
pub use instruction::{Instruction, InstructionName};
pub use syntax::SyntaxTree;
