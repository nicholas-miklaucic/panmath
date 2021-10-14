//! An abstract syntax tree that represents pure mathematical expressions. Unlike most versions of a
//! tree for math, the goal is not to evaluate or transform mathematical expressions, but rather to
//! have a useful shared representation of various math typesetting approaches. This means that, for
//! instance, free variables are fine.

/// An abstract syntax tree representing a mathematical expression.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum AST {
    /// A generic symbol.
    Sym(Symbol),
    /// A number literal, represented as a string.
    Number(String),
    /// A binary expression with two elements.
    BinaryExpr(BinaryOp, Box<AST>, Box<AST>),
    /// A unary expression with a single element.
    UnaryExpr(UnaryOp, Box<AST>),
    /// A function with a name and an arbitrary number of arguments.
    Function(Symbol, Vec<AST>),
}

/// A generic symbol. Can have multiple different representations, with a preferred one
/// used for specific types of output but with all forms acceptable as input.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Symbol {
    /// The preferred Unicode representation.
    pub unicode_repr: String,

    /// The preferred ASCII representation.
    pub ascii_repr: String,

    /// The preferred LaTeX representation.
    pub latex_repr: String,

    /// Any other representations to recognize in input.
    pub other_reprs: Vec<String>,
}

impl Symbol {
    /// Convenience function to create a symbol from strs.
    pub fn new(unicode: &str, ascii: &str, latex: &str, others: Vec<&str>) -> Symbol {
        Symbol {
            unicode_repr: unicode.into(),
            ascii_repr: ascii.into(),
            latex_repr: latex.into(),
            other_reprs: others.iter().map(|&x| x.to_string()).collect(),
        }
    }

    /// Gets all of the allowed representations.
    pub fn reprs(&self) -> Vec<&str> {
        let mut reprs = vec![
            self.unicode_repr.as_str(),
            self.ascii_repr.as_str(),
            self.latex_repr.as_str(),
        ];
        reprs.extend_from_slice(
            self.other_reprs
                .iter()
                .map(|x| x.as_str())
                .collect::<Vec<&str>>()
                .as_slice(),
        );
        reprs
    }
}

/// A specific kind of binary operation: prefix, infix, or postfix. This determines where the
/// operator goes: before, in the middle, or after the arguments.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub enum Fixity {
    Prefix,
    Infix,
    Postfix,
}

/// A generic binary operator with some symbol and a fixity. This covers almost all of the standard
/// binary operations (comparison, arithmetic, logical operators), but not all: for example,
/// exponentiation is normally *not* displayed in LaTeX or Unicode using the caret symbol.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SymbolBinaryOp {
    /// The symbol used for the operator.
    pub symbol: Symbol,
    /// The fixity of the operator.
    pub fixity: Fixity,
}

/// A binary operator. These fall into two classes: "normal" operators like + and - where the only
/// differences are in symbol and fixity, and "special" operators like fractions and exponentiation
/// that have different representations.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum BinaryOp {
    /// A generic binary operation with a symbol and fixity.
    Generic(SymbolBinaryOp),
    /// Exponentiation.
    Power,
    /// Division, represented using fractions when possible.
    Frac,
    /// A logarithm with a specific base.
    Log,
}

/// A unary operator. For simple ones like the logical not and unary minus/plus, this is just a
/// symbol. In the future there might be more complex examples with custom parsing.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum UnaryOp {
    /// A generic unary operator with a given symbol.
    Generic(Symbol),
}

/// A function with an arbitrary number of arguments.
pub struct Function {
    /// The function name.
    name: Symbol,
    /// The function arguments.
    args: Vec<AST>,
}
