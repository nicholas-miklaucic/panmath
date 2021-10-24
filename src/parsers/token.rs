//! Defines a token structure and tokenizer.

use crate::{
    ast::Symbol,
    delimiter::{self, Delimiter},
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
            // the last token matched. If it's the start, an operator, delimiter, or function, then
            // unary operators are the only allowed operators (`sin *6` makes no sense, and `sin -6`
            // must mean sine of negative 6). If it's an operand, then it's the reverse: `12-34`
            // must mean 12 minus 34, because having two numbers juxtaposed isn't allowed.
            let curr_ops = match tokens.last() {
                Some(Token::Operand(_)) => operators::BINARY_OPS.clone(),
                _ => operators::UNARY_OPS.clone(),
            };

            // match operators next: they tend not to conflict with other
            // things, and the bigger words will get mangled by future
            // transformations
            for op in curr_ops.iter() {
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
        let expr = "1 + (2 * 3)";

        println!("{:#?}", Tokenizer::default().tokenize(expr));
        assert_eq!(0, 0);
    }
}
