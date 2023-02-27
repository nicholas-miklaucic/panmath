//! A Formatter for LaTeX.

use crate::{
    ast::{self, SymbolBinaryOp},
    formatters::precedence::need_parens,
    operators::Op,
};

/// A formatter for LaTeX.
#[derive(Default)]
pub struct LatexFormatter {}

impl crate::formatter::Formatter for LatexFormatter {
    type Output = String;

    fn format_symbol(&mut self, sym: &ast::Symbol) -> Self::Output {
        sym.latex_repr.clone()
    }

    fn format_number(&mut self, dec: &str) -> Self::Output {
        dec.to_string()
    }

    fn format_binary_expr(
        &mut self,
        op: &ast::BinaryOp,
        arg1: &Box<ast::AST>,
        arg2: &Box<ast::AST>,
    ) -> Self::Output {
        let (left_p, right_p) = need_parens(op, arg1, arg2);
        let left_no_paren = self.format(&arg1.to_owned());
        let left = if left_p {
            format!("({})", left_no_paren)
        } else {
            format!("{}", left_no_paren)
        };
        let right_no_paren = self.format(&arg2.to_owned());
        let right = if right_p {
            format!("({})", right_no_paren)
        } else {
            format!("{}", right_no_paren)
        };
        match op {
            ast::BinaryOp::Generic(SymbolBinaryOp { op, fixity }) => {
                let symbol = self.format_symbol(&op.sym);
                match fixity {
                    ast::Fixity::Prefix => format!("{} {} {}", symbol, left, right),
                    ast::Fixity::Infix => format!("{} {} {}", left, symbol, right),
                    ast::Fixity::Postfix => format!("{} {} {}", left, right, symbol),
                }
            }
            // superscript takes care of parenthesis need
            ast::BinaryOp::Power => format!("{}^{{{}}}", left, right_no_paren),
            // fractions never need parentheses for their outer arguments
            ast::BinaryOp::Frac => format!("\\frac{{ {} }}{{ {} }}", left_no_paren, right_no_paren),
            // log subscript means no paren is needed
            ast::BinaryOp::Log => {
                format!("\\log_{{ {} }} \\left( {} \\right)", left_no_paren, right)
            }
            ast::BinaryOp::Concat => format!(r"{}{}", left, right),
        }
    }

    fn format_unary_expr(&mut self, op: &ast::UnaryOp, arg: &Box<ast::AST>) -> Self::Output {
        let arg = self.format(&arg.to_owned());
        match op {
            ast::UnaryOp::Generic(sym) => {
                let sym = self.format_symbol(sym);
                format!("{} {}", sym, arg)
            }
        }
    }

    fn format_function(&mut self, name: &ast::Symbol, args: &Vec<ast::AST>) -> Self::Output {
        let name = self.format_symbol(name);
        let args: Vec<String> = args.iter().map(|ast| self.format(ast)).collect();
        format!("{}\\left({}\\right)", name, args.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::formatter::Formatter;
    use crate::parsers::{ASTParser, AsciiParser};

    #[test]
    fn test_formatting() {
        let tree = ast::AST::BinaryExpr(
            ast::BinaryOp::Frac,
            Box::new(ast::AST::Function(
                ast::Symbol::from("f"),
                vec![
                    ast::AST::Number("100".to_string()),
                    ast::AST::Sym(ast::Symbol::from("x")),
                ],
            )),
            Box::new(ast::AST::UnaryExpr(
                ast::UnaryOp::Generic(ast::Symbol::from("-")),
                Box::new(ast::AST::Number("12.34".to_string())),
            )),
        );
        assert_eq!(
            LatexFormatter {}.format(&tree),
            r"\frac{ f\left(100, x\right) }{ - 12.34 }".to_string()
        );
    }

    #[test]
    fn test_parse() {
        let parser = AsciiParser::default();
        let tree = parser.parse(&"2 / (sin mu + 1)".to_owned()).unwrap();
        assert_eq!(
            LatexFormatter::default().format(&tree),
            r"\frac{ 2 }{ \sin\left(\mu\right) + 1 }".to_string()
        );
        let tree = parser.parse(&"mu ^ (3 * (4 + 5))".to_owned()).unwrap();
        assert_eq!(
            LatexFormatter::default().format(&tree),
            r"\mu^{3 \cdot (4 + 5)}".to_string()
        );
        let tree = parser.parse(&"cos^2(A) + sin^2(B)".to_owned()).unwrap();
        assert_eq!(
            LatexFormatter::default().format(&tree),
            r"\cos^2\left(A\right) + \sin^2\left(B\right)".to_string()
        );
        let tree = parser.parse(&"2 / arccos mu + 1".to_owned()).unwrap();
        assert_eq!(
            LatexFormatter::default().format(&tree),
            r"\frac{ 2 }{ \arccos\left(\mu\right) } + 1".to_string()
        );
    }
}
