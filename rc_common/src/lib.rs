mod types;
pub mod error;

pub use types::*;

use nom_supreme::error::ErrorTree;

pub type Error<'a> = ErrorTree<&'a str>;
pub type IResult<'a, O> = nom::IResult<&'a str, O, Error<'a>>;