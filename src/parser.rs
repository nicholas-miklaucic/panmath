//! File that contains parsers for the AST tree.

use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_until};
use nom::character::complete::{self, char, i64, space0, space1};
use nom::combinator::{cond, fail, map, peek, recognize};
use nom::error::{Error, ParseError};
use nom::multi::separated_list0;
use nom::number::complete::double;
use nom::sequence::{delimited, pair, preceded, separated_pair, tuple};
use nom::{IResult, InputTake};

use crate::ast::{BinaryOp, Symbol, SymbolBinaryOp, UnaryOp, AST};
use crate::symbols::ALL_SYMBOLS;

/// Parses a predefined Symbol.
pub fn parse_symbol_from_list<'a, T: IntoIterator<Item = Symbol>>(
    symbols: T,
) -> impl Fn(&'a str) -> IResult<&'a str, Symbol, Error<&'a str>>
where
    T: IntoIterator<Item = Symbol> + Clone,
{
    move |input: &str| {
        for sym in symbols.clone() {
            for repr in sym.reprs().into_iter() {
                if let Ok((next, _name)) = tag::<&str, &str, nom::error::Error<&str>>(repr)(input) {
                    return Ok((next, sym.clone()));
                }
            }
        }
        fail(input)
    }
}

/// Parses a predefined Symbol.
pub fn parse_symbol_from_map<'a, U: Clone + std::cmp::Eq + std::hash::Hash>(
    symbols: HashMap<U, Symbol>,
) -> impl Fn(&'a str) -> IResult<&'a str, Symbol, Error<&'a str>> {
    parse_symbol_from_list(symbols.clone().into_values().collect::<Vec<Symbol>>())
}

/// Parses a special function. Unlike the generic function, this doesn't require parentheses and is unary.
pub fn parse_special_function(input: &str) -> IResult<&str, AST> {
    let (rest, (sym, expr)) = pair(
        parse_symbol_from_map(crate::symbols::SPECIAL_FUNCS.clone()),
        parse_expr,
    )(input)?;
    Ok((rest, AST::UnaryExpr(UnaryOp::Generic(sym), Box::new(expr))))
}

/// Parses a number.
pub fn parse_number(input: &str) -> IResult<&str, AST> {
    let (rest, dec) = recognize(double)(input)?;
    Ok((rest, AST::Number(dec.to_string())))
}

/// Parses a parenthetical expression.
pub fn parse_parens(input: &str) -> IResult<&str, AST> {
    delimited(tag("("), parse_expr, tag(")"))(input)
}

/// Parses an expression with potential spaces around it.
pub fn parse_spaces(input: &str) -> IResult<&str, AST> {
    delimited(space1, parse_expr, space0)(input)
}

/// Parses a fraction.
pub fn parse_frac(input: &str) -> IResult<&str, AST> {
    if !input.contains("/") {
        fail(input)
    } else {
        let (rest, num_str) = take_until("/")(input)?;
        let (_num_rest, num) = parse_expr(num_str)?;
        let (rest, (_slash, denom)) = pair(delimited(space0, tag("/"), space0), parse_expr)(rest)?;
        Ok((
            rest,
            AST::BinaryExpr(BinaryOp::Frac, Box::new(num), Box::new(denom)),
        ))
    }
}

/// Parses a symbol for a binary operation.
pub fn parse_symbol_binary_op(input: &str) -> IResult<&str, SymbolBinaryOp> {
    map(
        parse_symbol_from_list(crate::symbols::BINARY_OPS.clone()),
        |x| SymbolBinaryOp {
            symbol: x,
            fixity: crate::ast::Fixity::Infix,
        },
    )(input)
}

/// Matches a pattern, with the condition that it cannot be inside parentheses. Returns the index if it exists.
pub fn find_outer(pattern: &str, input: &str) -> Option<usize> {
    let mut layers_deep = 0;
    if input.len() < pattern.len() {
        return None;
    }
    for i in 0..input.len() - pattern.len() {
        match input.get(i..i + 1) {
            Some("(") => {
                layers_deep += 1;
            }
            Some(")") => {
                layers_deep -= 1;
            }
            Some(_) => {
                if input.get(i..i + pattern.len()) == Some(pattern) && layers_deep == 0 {
                    return Some(i);
                }
            }
            _ => continue,
        };
    }
    None
}

/// Searches for a specific symbol as the root of the AST.
pub fn parse_specific_binary_op<'a>(symbol: Symbol, input: &'a str) -> IResult<&'a str, AST> {
    for repr in symbol.reprs() {
        if let Some(i) = find_outer(repr, input) {
            let (_, expr1) = parse_expr(input.get(0..i).unwrap())?;
            let (rest, expr2) = parse_expr(input.get(i + repr.len()..).unwrap())?;
            return Ok((
                rest,
                AST::BinaryExpr(
                    crate::ast::BinaryOp::Generic(SymbolBinaryOp {
                        symbol,
                        fixity: crate::ast::Fixity::Infix,
                    }),
                    Box::new(expr1),
                    Box::new(expr2),
                ),
            ));
        }
    }
    fail(input)
}

/// Parses the outermost layer of the expression if it matches a binary operator.
pub fn parse_binary_expr(input: &str) -> IResult<&str, AST> {
    for symbol in crate::symbols::BINARY_OPS.clone().into_iter() {
        let res = parse_specific_binary_op(symbol, input);
        if res.is_ok() {
            return res;
        }
    }
    fail(input)
}

/// Parses a generic expression.
pub fn parse_expr(input: &str) -> IResult<&str, AST> {
    // Order is very important here, because anything can be a symbol, and some
    // operations like division need to be parsed specially and caught before
    // the generic versions would apply.
    alt((
        parse_binary_expr,
        parse_special_function,
        parse_frac,
        parse_number,
        parse_parens,
        nom::Parser::map(parse_symbol_from_list(crate::symbols::MISC.clone()), |x| {
            AST::Sym(x)
        }),
        nom::Parser::map(
            parse_symbol_from_map(crate::symbols::GREEK_SYMBOLS.clone()),
            |x| AST::Sym(x),
        ),
        nom::Parser::map(
            parse_symbol_from_map(crate::symbols::LATIN_SYMBOLS.clone()),
            |x| AST::Sym(x),
        ),
        parse_spaces,
    ))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(parse_number("0123"), Ok(("", AST::Number("0123".into()))));
        assert_eq!(
            parse_number("123 + 234"),
            Ok((" + 234", AST::Number("123".into())))
        );
        assert_eq!(parse_number("40.2"), Ok(("", AST::Number("40.2".into()))));
        assert_eq!(
            parse_number("100200.0"),
            Ok(("", AST::Number("100200.0".into())))
        );
        assert_eq!(
            parse_number("123.045 + 234"),
            Ok((" + 234", AST::Number("123.045".into())))
        );
    }

    #[test]
    fn test_decimal() {}

    #[test]
    fn test_known_symbol() {
        let special: HashMap<String, Symbol> = crate::symbols::SPECIAL_FUNCS.clone();
        let misc: Vec<Symbol> = crate::symbols::MISC.clone();
        assert_eq!(
            parse_symbol_from_map(special.clone())("sin x"),
            Ok((" x", special.get("sin").unwrap().clone()))
        );
        assert_eq!(
            parse_symbol_from_list(misc.clone())("≠ b"),
            Ok((" b", crate::symbols::NEQ.clone()))
        );
    }

    // figure out how to parse this
    // #[test]
    // fn test_spaces() {
    //     assert_eq!(
    //         parse_expr("sin x degrees"),
    //         Ok((
    //             "",
    //             AST::UnaryExpr(
    //                 UnaryOp::Generic(crate::symbols::SPECIAL_FUNCS.get("sin").unwrap().clone()),
    //                 Box::new(AST::Number("40.2".into()))
    //             )
    //         ))
    //     );
    // }

    #[test]
    fn test_frac() {
        assert_eq!(
            parse_frac("123/234"),
            Ok((
                "",
                AST::BinaryExpr(
                    BinaryOp::Frac,
                    Box::new(AST::Number("123".into())),
                    Box::new(AST::Number("234".into()))
                )
            ))
        );
        assert_eq!(
            parse_number("123 + 234"),
            Ok((" + 234", AST::Number("123".into())))
        );
    }

    #[test]
    fn test_simple() {
        assert_eq!(
            parse_expr("sin 40.2"),
            Ok((
                "",
                AST::UnaryExpr(
                    UnaryOp::Generic(crate::symbols::SPECIAL_FUNCS.get("sin").unwrap().clone()),
                    Box::new(AST::Number("40.2".into()))
                )
            ))
        )
    }
}
