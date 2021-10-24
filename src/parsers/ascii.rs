//! Parser for plaintext math.

use std::collections::VecDeque;

use crate::ast::{BinaryOp, Fixity, Symbol, SymbolBinaryOp, UnaryOp, AST};
use crate::delimiter::{self, DelimDir, Delimiter};
use crate::operators::Op;
use crate::parsers::token::Token;

use super::token::Tokenizer;

/// Represents an error while parsing input expressions.
#[derive(Debug, Clone)]
pub enum ParseError {
    /// Indicates that parentheses are mismatched.
    MismatchedParentheses,
    /// Indicates that operators are missing operators.
    MissingOperands,
    /// Indicates an empty expression.
    EmptyExpr,
}

/// Parses the list of tokens into postfix.
pub fn parse_into_postfix(inputs: Vec<Token>) -> Result<VecDeque<Token>, ParseError> {
    // implements the shunting-yard algorithm
    // embarrassingly, my reference is Wikipedia
    // https://www.wikiwand.com/en/Shunting-yard_algorithm

    // use as a stack
    let mut operators = VecDeque::new();

    // use as a queue
    let mut output = VecDeque::new();

    for token in inputs.into_iter() {
        // println!("Before token {:#?}", token.clone());
        // println!("Operators: {:#?}", operators.clone());
        // println!("Output: {:#?}", output.clone());
        match token {
            Token::Operand(_) => output.push_back(token),
            Token::Operator(Op {
                sym: ref _sym1,
                l_prec: l_prec1,
                r_prec: _r_prec1,
            }) => {
                while let Some(op2) = operators.front() {
                    match op2 {
                        Token::Delim(Delimiter {
                            dir: delimiter::DelimDir::Left,
                            kind: _,
                        }) => {
                            // we can't bind things from beyond a left delimiter: at the + in
                            // 2 * (3 + 4), we only bind the 3
                            break;
                        }
                        // Because we're extending to the left from op1's position, we use op1's
                        // left precedence and op2's right precedence. If we're at the second ^ in
                        // 2 ^ 2 ^ 3, we don't bind the first ^, because ^ binds more strongly on
                        // the right than on the left.

                        // this < could be nonstrict, and nothing should change: if two precedences
                        // are equal, that should mean that they're completely interchangeable.
                        Token::Operator(Op {
                            sym: _sym,
                            l_prec: _l_prec,
                            r_prec,
                        }) => {
                            let does_bind = match (l_prec1, r_prec) {
                                // If both exist, see if rp is lower, meaning more precedent
                                (Some(lp), Some(rp)) => rp < &lp,
                                // The only standard example of an operator with None on the right
                                // side is ! (factorial). So here an example might be 2! * 3: no
                                // matter what *'s precedence is, the postfix becomes 2 ! 3 *, with
                                // ! pushed first.
                                (Some(_lp), None) => true,
                                // An example of an operator with no left precedence is -. If we
                                // consider the example 2 + -3, here no matter what + is the correct
                                // postfix is 2 3 - +, with the + not being inserted first.
                                (None, Some(_rp)) => false,
                                // This should never happen in valid math: an example of what this
                                // would look like is 2! sin 3 if sin were an operator.
                                (None, None) => return Err(ParseError::MissingOperands),
                            };

                            if does_bind {
                                output.push_back(operators.pop_front().unwrap());
                            } else {
                                break;
                            }
                        }
                        Token::Function(_) => {
                            // functions never bind through operators without parentheses: sin 2 + x
                            // should convert to 2 sin x +, because if people mean sin (2 + x) they
                            // should write it with parentheses
                            output.push_back(operators.pop_front().unwrap());
                        }
                        _ => {
                            // this should never happen, because the operator stack should only
                            // contain left delimiters, operators, and functions.
                            panic!("Unknown token on operator stack: {:?}", op2);
                        }
                    }
                }
                operators.push_front(token);
            }
            Token::Function(_) => operators.push_front(token),
            Token::Delim(Delimiter { dir, kind: lkind }) => match dir {
                DelimDir::Left => operators.push_front(token),
                DelimDir::Right => {
                    while let Some(op2) = operators.front() {
                        if let Token::Delim(Delimiter {
                            dir: DelimDir::Left,
                            kind,
                        }) = op2
                        {
                            if kind == &lkind {
                                // found matching pair
                                // get rid of left paren, it did its duty
                                operators.pop_front();
                                // if function, pop onto output
                                if let Some(Token::Function(_)) = operators.front() {
                                    output.push_back(operators.pop_front().unwrap())
                                }
                            } else {
                                // something like (1 + [2 + 3)] happened and parens are mismatched
                                return Err(ParseError::MismatchedParentheses);
                            }
                        } else {
                            // otherwise, push onto output
                            output.push_back(operators.pop_front().unwrap());
                        }
                    }
                }
            },
            Token::End => {
                break;
            }
        }
    }
    output.append(&mut operators);
    return Ok(output);
}

/// Given an AST, unpacks all outer , operators into a list.
fn comma_sep_to_list(tree: AST) -> Vec<AST> {
    match tree {
        AST::BinaryExpr(BinaryOp::Generic(SymbolBinaryOp { symbol, .. }), arg1, arg2)
            if symbol == crate::symbols::COMMA.clone() =>
        {
            let mut args1 = comma_sep_to_list(*arg1);
            let mut args2 = comma_sep_to_list(*arg2);
            args1.append(&mut args2);
            args1
        }
        _ => {
            vec![tree]
        }
    }
}

/// Turns a postfix-ordered list of tokens into an AST.
pub fn parse_into_tree(tokens: VecDeque<Token>) -> Result<AST, ParseError> {
    let mut exprs = VecDeque::new();

    for token in tokens.into_iter() {
        match token {
            Token::Operand(sym) => exprs.push_front(AST::Sym(sym)),
            Token::Operator(op) => {
                // TODO integrate this into type system so it isn't hacky, by adding arity to
                // operators themselves
                if crate::operators::UNARY_OPS.contains(&op) {
                    let new_expr = match exprs.pop_front() {
                        Some(tree) => AST::UnaryExpr(UnaryOp::Generic(op.sym), Box::new(tree)),
                        None => return Err(ParseError::MissingOperands),
                    };
                    exprs.push_front(new_expr);
                } else {
                    let new_expr = match (exprs.pop_front(), exprs.pop_front()) {
                        (Some(arg2), Some(arg1)) => {
                            // special-case special binary operations
                            if op == crate::operators::POWER.clone() {
                                AST::BinaryExpr(BinaryOp::Power, Box::new(arg1), Box::new(arg2))
                            } else if op == crate::operators::DIV.clone() {
                                AST::BinaryExpr(BinaryOp::Frac, Box::new(arg1), Box::new(arg2))
                            } else {
                                AST::BinaryExpr(
                                    BinaryOp::Generic(SymbolBinaryOp {
                                        symbol: op.sym,
                                        fixity: Fixity::Infix,
                                    }),
                                    Box::new(arg1),
                                    Box::new(arg2),
                                )
                            }
                        }
                        _ => return Err(ParseError::MissingOperands),
                    };
                    exprs.push_front(new_expr);
                }
            }
            Token::Function(func) => match exprs.pop_front() {
                Some(tree) => exprs.push_front(AST::Function(func, comma_sep_to_list(tree))),
                None => return Err(ParseError::MissingOperands),
            },
            // if there's a delimiter here, it must be a left delimiter that never got cleaned up by
            // its associated right pair, so parens are mismatched
            Token::Delim(_) => return Err(ParseError::MismatchedParentheses),
            Token::End => {
                break;
            }
        }
    }

    // now we have one or many expressions to concatenate together
    let output = exprs
        .into_iter()
        .rev()
        .reduce(|acc, new| AST::BinaryExpr(BinaryOp::Concat, Box::new(acc), Box::new(new)));

    match output {
        None => Err(ParseError::EmptyExpr),
        Some(tree) => Ok(tree),
    }
}

/// A parser for ASCII.
#[derive(Debug, Clone, Default)]
pub struct AsciiParser {
    /// The tokenizer to use.
    tokenizer: Tokenizer,
}

impl<T> super::ASTParser<T> for AsciiParser
where
    T: ToString,
{
    type ParseError = ParseError;

    fn parse(&self, input: &T) -> Result<AST, Self::ParseError> {
        let input = input.to_string();
        let tokens = self.tokenizer.tokenize(&input);
        let postfix = parse_into_postfix(tokens)?;
        dbg!(
            "{}",
            postfix
                .clone()
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        dbg!(
            "{:#?}",
            parse_into_tree(postfix.clone()).unwrap_or(AST::Sym(Symbol::from("oops")))
        );
        parse_into_tree(postfix)
    }
}

#[cfg(test)]
mod tests {
    use crate::parsers::token::Tokenizer;

    use super::*;

    // #[test]
    // fn test_basic() {
    //     let tokens = Tokenizer::default().tokenize("2 + 3");
    //     assert_eq!(parse_into_postfix(tokens).unwrap(), vec![]);
    // }

    #[test]
    fn test_simple_frac() {
        let tokens = Tokenizer::default().tokenize("1 + (2 * 3)");
        println!("{:#?}", parse_into_postfix(tokens).unwrap());
        assert_eq!(0, 0);
    }
}
