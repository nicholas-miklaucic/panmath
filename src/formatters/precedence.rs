//! Helpers to deal with precedence and parentheses.
use crate::{
    ast::{BinaryOp, SymbolBinaryOp, AST},
    operators::{self, Op},
};

/// Precedence >.
fn prec_gt(prec1: &Option<u8>, prec2: &Option<u8>) -> bool {
    prec1
        .map(|p| p > prec2.unwrap_or_default())
        .unwrap_or(false)
}

/// Returns a tuple indicating whether the left and right children need
/// parentheses to be displayed properly.
pub fn need_parens(bin_op: &BinaryOp, lchild: &AST, rchild: &AST) -> (bool, bool) {
    match &bin_op {
        BinaryOp::Generic(SymbolBinaryOp {
            op:
                Op {
                    sym: _,
                    l_prec,
                    r_prec,
                },
            fixity: _,
        }) => (
            match lchild {
                AST::Sym(_) => false,    // a + b is fine
                AST::Number(_) => false, // 2 + 3 is fine
                // for binary operations: if the corresponding
                // precedence is higher than the precedence of
                // this operator, then parenthese are needed.
                // For example, 2 ^ 3 + 1 ^ 2 is fine, but
                // (2 + 3) ^ (1 + 2) needs parentheses.
                // None is treated as never needing parentheses:
                // 2 ^ ±3 is unambiguous, if weird
                AST::BinaryExpr(bop, _, _) => match bop {
                    BinaryOp::Generic(SymbolBinaryOp {
                        op: l_op,
                        fixity: _,
                    }) => prec_gt(&l_op.r_prec, l_prec),
                    BinaryOp::Power => prec_gt(&operators::POWER.r_prec, l_prec),
                    BinaryOp::Frac => prec_gt(&operators::DIV.r_prec, l_prec),
                    BinaryOp::Log => prec_gt(&operators::POWER.r_prec, l_prec),
                    BinaryOp::Concat => false,
                }, // (a + 2)b needs parens
                AST::UnaryExpr(_, _) => false, // -ab doesn't need one, but future might
                AST::Function(_, _) => false,  // sin(x)a is fine
            },
            match rchild {
                AST::Sym(_) => false,    // a + b is fine
                AST::Number(_) => false, // 2 + 3 is fine
                // for binary operations: if the corresponding
                // precedence is higher than the precedence of
                // this operator, then parenthese are needed.
                // For example, 2 ^ 3 + 1 ^ 2 is fine, but
                // (2 + 3) ^ (1 + 2) needs parentheses.
                // None is treated as never needing parentheses:
                // 2 ^ ±3 is unambiguous, if weird
                AST::BinaryExpr(bop, _, _) => match bop {
                    BinaryOp::Generic(SymbolBinaryOp {
                        op: l_op,
                        fixity: _,
                    }) => prec_gt(&l_op.l_prec, r_prec),
                    BinaryOp::Power => prec_gt(&operators::POWER.l_prec, r_prec),
                    BinaryOp::Frac => prec_gt(&operators::DIV.l_prec, r_prec),
                    BinaryOp::Log => prec_gt(&operators::POWER.l_prec, r_prec),
                    BinaryOp::Concat => false,
                }, // (a + 2)b needs parens
                AST::UnaryExpr(_, _) => false, // -ab doesn't need one, but future might
                AST::Function(_, _) => false,  // sin(x)a is fine
            },
        ),
        // defer others to the normal symbol versions
        // LaTeX or other formats with special syntax can ignore as needed
        BinaryOp::Power => need_parens(
            &BinaryOp::Generic(SymbolBinaryOp {
                op: operators::POWER.to_owned(),
                fixity: crate::ast::Fixity::Infix,
            }),
            lchild,
            rchild,
        ),
        BinaryOp::Frac => need_parens(
            &BinaryOp::Generic(SymbolBinaryOp {
                op: operators::DIV.to_owned(),
                fixity: crate::ast::Fixity::Infix,
            }),
            lchild,
            rchild,
        ),
        // treat log the same as exp
        BinaryOp::Log => need_parens(
            &BinaryOp::Generic(SymbolBinaryOp {
                op: operators::POWER.to_owned(),
                fixity: crate::ast::Fixity::Infix,
            }),
            lchild,
            rchild,
        ),
        BinaryOp::Concat => (
            match lchild {
                AST::Sym(_) => false,             // ab is fine
                AST::Number(_) => false,          // 2a is fine
                AST::BinaryExpr(_, _, _) => true, // (a + 2)b needs parens
                AST::UnaryExpr(_, _) => false,    // -ab doesn't need one, but future might
                AST::Function(_, _) => false,     // sin(x)a is fine
            },
            match rchild {
                AST::Sym(_) => false,             // ab is fine
                AST::Number(_) => false,          // a2 is fine, if weird
                AST::BinaryExpr(_, _, _) => true, // b(2 + a) needs parens
                AST::UnaryExpr(_, _) => true,     // b(-a) needs parens
                AST::Function(_, _) => false,     // a sin(x) needs no parens
            },
        ),
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        ast::Fixity,
        ast::SymbolBinaryOp,
        operators,
        parsers::{ASTParser, AsciiParser},
    };

    use super::*;

    #[test]
    fn test_need_parens() {
        let parser = AsciiParser::default();
        // (2 - 3) + 1 / 2 is proper
        assert_eq!(
            need_parens(
                &BinaryOp::Generic(SymbolBinaryOp {
                    op: operators::ADD.to_owned(),
                    fixity: Fixity::Infix
                }),
                &parser.parse(&"2 - 3").unwrap(),
                &parser.parse(&"1 / 2").unwrap()
            ),
            (true, false)
        );

        // 2 + 3 * 1 ^ 2 needs parens on the left
        assert_eq!(
            need_parens(
                &BinaryOp::Generic(SymbolBinaryOp {
                    op: operators::MULT.to_owned(),
                    fixity: Fixity::Infix
                }),
                &parser.parse(&"2 + 3").unwrap(),
                &parser.parse(&"1 ^ 2").unwrap()
            ),
            (true, false)
        );
    }
}
