mod declaration;
mod instruction;
mod syntax;

pub use declaration::{Declaration, Reference};
pub use instruction::{Instruction, InstructionName};
use nom_supreme::error::ErrorTree;
pub use syntax::SyntaxTree;

pub type Error<'a> = ErrorTree<&'a str>;
pub type IResult<'a, O> = nom::IResult<&'a str, O, Error<'a>>;
