//! Defines a token structure and tokenizer.

use std::fmt::Display;

use crate::{
    ast::Symbol,
    delimiter::{self, DelimDir, DelimKind, Delimiter},
    operators::{self, Op},
    symbols,
};

/// A token in a math expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    /// A generic operand, written as a symbol.
    Operand(Symbol),

    /// An operator, written as a symbol and with given left and right precedence.
    Operator(Op),

    /// A function with a specific name.
    Function(Symbol),

    /// A delimiter.
    Delim(Delimiter),

    /// End-of-expression.
    End,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Operand(sym) => write!(f, "{}", sym.unicode_repr),
            Token::Operator(op) => write!(f, "{}", op.sym.unicode_repr),
            Token::Function(sym) => write!(f, "{}", sym.unicode_repr),
            Token::Delim(delimiter) => write!(f, "{}", delimiter),
            Token::End => write!(f, "{}", "eof"),
        }
    }
}

/// A tokenizer that parses strings into a list of tokens.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
// options TBD
pub struct Tokenizer {}

impl Tokenizer {
    /// Tokenizes an expression into a list of tokens.
    pub fn tokenize(&self, input: &str) -> Vec<Token> {
        let mut rest = input.clone();
        let mut tokens = vec![];
        let mut curr_unknown = String::new();
        'parse: while !rest.is_empty() {
            // first, some cleanup to get rid of whitespace
            match rest.chars().next() {
                Some(c) if c.is_whitespace() => {
                    // push previous unknown token onto list
                    if !curr_unknown.is_empty() {
                        tokens.push(Token::Operand(curr_unknown.into()));
                        curr_unknown = String::new();
                    }
                    rest = &rest[1..];
                }
                _ => {}
            };

            // match delimiters
            for delim in delimiter::DELIMS.iter() {
                if let Some(repr) = delim.get_symbol().match_front(rest) {
                    rest = &rest[repr.len()..];
                    // push previous unknown token onto list
                    if !curr_unknown.is_empty() {
                        tokens.push(Token::Operand(curr_unknown.into()));
                        curr_unknown = String::new();
                    }
                    tokens.push(Token::Delim(*delim));
                    // continue outer parsing loop
                    continue 'parse;
                }
            }

            // This part is very thorny: we need to handle unary plus/minus operators correctly. The
            // weird thing is that this depends on the state of the parsing so far: specifically,
            // the last token matched. If it's the start, an operator, a left delimiter, or a
            // function, then unary operators are the only allowed operators (`sin *6` makes no
            // sense, and `sin -6` must mean sine of negative 6). If it's an operand or right
            // delimiter, then it's the reverse: `12-34` must mean 12 minus 34, because having two
            // numbers juxtaposed isn't allowed.
            let curr_ops = match tokens.last() {
                Some(Token::Operand(_)) => operators::BINARY_OPS.clone(),
                Some(Token::Delim(Delimiter { dir, kind: _ })) if dir == &DelimDir::Right => {
                    operators::BINARY_OPS.clone()
                }
                _ => {
                    // If there's an unrecognized symbol being built up, then we can't search for
                    // unary operators: if we're in the middle of a-b, we should realize that - is a
                    // binary operator
                    if curr_unknown.is_empty() {
                        operators::UNARY_OPS.clone()
                    } else {
                        operators::BINARY_OPS.clone()
                    }
                }
            };
            // match operators next: they tend not to conflict with other
            // things, and the bigger words will get mangled by future
            // transformations
            for op in curr_ops.iter() {
                println!(
                    "{} | {} | [{}]",
                    &op.sym.ascii_repr,
                    rest.clone(),
                    tokens
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
                if let Some(repr) = op.match_front(rest) {
                    rest = &rest[repr.len()..];
                    // push previous unknown token onto list
                    if !curr_unknown.is_empty() {
                        tokens.push(Token::Operand(curr_unknown.into()));
                        curr_unknown = String::new();
                    }
                    tokens.push(Token::Operator(op.clone()));
                    // continue outer parsing loop
                    continue 'parse;
                }
            }

            // now match known functions
            for (_name, sym) in symbols::SPECIAL_FUNCS.iter() {
                if let Some(repr) = sym.match_front(rest) {
                    rest = &rest[repr.len()..];
                    // push previous unknown token onto list
                    if !curr_unknown.is_empty() {
                        tokens.push(Token::Operand(curr_unknown.into()));
                        curr_unknown = String::new();
                    }
                    tokens.push(Token::Function(sym.clone()));
                    // continue outer parsing loop
                    continue 'parse;
                }
            }

            // now match known non-Latin letter symbols
            for sym in symbols::ALL_SYMBOLS.iter() {
                if !symbols::LATIN_SYMBOLS.contains_key(&sym.ascii_repr) {
                    if let Some(repr) = sym.match_front(rest) {
                        rest = &rest[repr.len()..];
                        // push previous unknown token onto list
                        if !curr_unknown.is_empty() {
                            tokens.push(Token::Operand(curr_unknown.into()));
                            curr_unknown = String::new();
                        }
                        tokens.push(Token::Operand(sym.clone()));
                        // continue outer parsing loop
                        continue 'parse;
                    }
                }
            }

            // if unknown, add to current unknown symbol
            curr_unknown.push(rest.chars().next().unwrap().into());
            rest = &rest[1..];
        }

        // add end of expression symbol
        // push previous unknown token onto list
        if !curr_unknown.is_empty() {
            tokens.push(Token::Operand(curr_unknown.into()));
        }
        tokens.push(Token::End);
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizing() {
        let expr = "(1 + 2) ^ mu";

        println!("{:#?}", Tokenizer::default().tokenize(expr));
        assert_eq!(0, 1);
    }
}
