mod declaration;
mod instruction;
mod syntax;
mod error;
mod word;
mod vm;

pub use declaration::{Declaration, Reference};
pub use instruction::{Instruction, InstructionName};
pub use syntax::SyntaxTree;
pub use error::{SyntaxError, SyntaxErrorContext};
pub use word::Word;
pub use vm::VM;

use nom_supreme::error::ErrorTree;

pub type Error<'a> = ErrorTree<&'a str>;
pub type IResult<'a, O> = nom::IResult<&'a str, O, Error<'a>>;
