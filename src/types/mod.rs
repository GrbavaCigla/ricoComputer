mod declaration;
mod instruction;
mod syntax;
mod error;

pub use declaration::{Declaration, Reference};
pub use instruction::{Instruction, InstructionName};
pub use syntax::SyntaxTree;
pub use error::SyntaxError;

use nom_supreme::error::ErrorTree;

pub type Error<'a> = ErrorTree<&'a str>;
pub type IResult<'a, O> = nom::IResult<&'a str, O, Error<'a>>;
