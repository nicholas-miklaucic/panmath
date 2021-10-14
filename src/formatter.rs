//! This module defines the `Formatter` trait, representing a given output format that can serialize
//! `AST`s. This is where all of the formatting logic is held: `AST`s themselves only store the
//! structure of data, not how that structure is represented.

use crate::ast::*;

/// A serializer for `AST`s, controlling how they are displayed to a specific output type T.
pub trait Formatter {
    type Output;

    /// Formats a symbol.
    fn format_symbol(&mut self, sym: &Symbol) -> Self::Output;

    /// Formats a number literal, given as a string.
    fn format_number(&mut self, dec: &str) -> Self::Output;

    /// Formats a binary expression with two arguments.
    fn format_binary_expr(
        &mut self,
        op: &BinaryOp,
        arg1: &Box<AST>,
        arg2: &Box<AST>,
    ) -> Self::Output;

    /// Formats a unary expression with one argument.
    fn format_unary_expr(&mut self, op: &UnaryOp, arg: &Box<AST>) -> Self::Output;

    /// Formats a function with a name and an arbitrary number of arguments.
    fn format_function(&mut self, name: &Symbol, args: &Vec<AST>) -> Self::Output;

    /// Formats an `AST` into the specified output type.
    fn format(&mut self, ast: &AST) -> Self::Output {
        match ast {
            AST::Sym(sym) => self.format_symbol(sym),
            AST::Number(string) => self.format_number(string),
            AST::BinaryExpr(op, arg1, arg2) => self.format_binary_expr(op, arg1, arg2),
            AST::UnaryExpr(op, arg) => self.format_unary_expr(op, arg),
            AST::Function(name, args) => self.format_function(name, args),
        }
    }
}
