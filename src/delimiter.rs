//! File to deal with arbitrary delimiter pairs.

use crate::{ast::Symbol, symbols};

/// The delimiter direction: either left or right, simply enough.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DelimDir {
    /// Left: (, [, etc.
    Left,
    /// Right: ), ], etc.
    Right,
}

/// The kind of delimiter.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DelimKind {
    /// A parenthesis: ()
    Paren,

    /// A bracket: []
    Bracket,
}

/// A delimiter with a symbol that can either be left or right.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Delimiter {
    /// Whether the delimiter is left or right.
    pub dir: DelimDir,
    /// What kind of delimiter it is.
    pub kind: DelimKind,
}

impl Delimiter {
    /// Gets a copy of the symbol that matches this delimiter.
    pub fn get_symbol(&self) -> Symbol {
        match (self.dir, self.kind) {
            (DelimDir::Left, DelimKind::Paren) => symbols::LEFT_PAR.clone(),
            (DelimDir::Left, DelimKind::Bracket) => symbols::LEFT_BRACKET.clone(),
            (DelimDir::Right, DelimKind::Paren) => symbols::RIGHT_PAR.clone(),
            (DelimDir::Right, DelimKind::Bracket) => symbols::RIGHT_BRACKET.clone(),
        }
    }
}

pub static LPAR: Delimiter = Delimiter {
    dir: DelimDir::Left,
    kind: DelimKind::Paren,
};
pub static RPAR: Delimiter = Delimiter {
    dir: DelimDir::Right,
    kind: DelimKind::Paren,
};
pub static LBRACKET: Delimiter = Delimiter {
    dir: DelimDir::Left,
    kind: DelimKind::Bracket,
};
pub static RBRACKET: Delimiter = Delimiter {
    dir: DelimDir::Right,
    kind: DelimKind::Bracket,
};

pub static DELIMS: [Delimiter; 4] = [LPAR, RPAR, LBRACKET, RBRACKET];
