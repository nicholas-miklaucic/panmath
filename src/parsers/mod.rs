//! Common interface for parsers that can create abstract syntax trees.

use crate::ast::AST;
pub mod ascii;
pub mod token;

pub use ascii::AsciiParser;

/// Code that can parse ASTs from a given input type.
pub trait ASTParser<I> {
    /// The error that parsing can raise.
    type ParseError;

    /// Attempts to parse the given input, returning an AST on success.
    fn parse(&self, input: &I) -> Result<AST, Self::ParseError>;
}
