//! A Formatter for Unicode output.

use crate::ast;

/// A formatter for Unicode that tries to use the Unicode math symbols wherever possible.
#[derive(Default)]
pub struct UnicodeFormatter {}

impl crate::formatter::Formatter for UnicodeFormatter {
    type Output = String;

    fn format_symbol(&mut self, sym: &ast::Symbol) -> Self::Output {
        sym.unicode_repr.clone()
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
        let left = self.format(&arg1.to_owned());
        let right = self.format(&arg2.to_owned());
        match op {
            ast::BinaryOp::Generic(ast::SymbolBinaryOp { symbol, fixity }) => {
                let sym = self.format_symbol(symbol);
                match fixity {
                    ast::Fixity::Prefix => format!("({} {} {})", sym, left, right),
                    ast::Fixity::Infix => format!("({} {} {})", left, sym, right),
                    ast::Fixity::Postfix => format!("({} {} {})", left, right, sym),
                }
            }
            ast::BinaryOp::Power => format!("{}^{}", left, right),
            ast::BinaryOp::Frac => format!("{} / {}", left, right),
            ast::BinaryOp::Log => format!("log_{} {}", left, right),
            ast::BinaryOp::Concat => format!("{}{}", left, right),
        }
    }

    fn format_unary_expr(&mut self, op: &ast::UnaryOp, arg: &Box<ast::AST>) -> Self::Output {
        let arg = self.format(&arg.to_owned());
        match op {
            ast::UnaryOp::Generic(sym) => {
                let sym = self.format_symbol(sym);
                format!("({} {})", sym, arg)
            }
        }
    }

    fn format_function(&mut self, name: &ast::Symbol, args: &Vec<ast::AST>) -> Self::Output {
        let name = self.format_symbol(name);
        let args: Vec<String> = args.iter().map(|ast| self.format(ast)).collect();
        format!("{}({})", name, args.join(", "))
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
        // assert_eq!(
        //     LatexFormatter {
        //         decimal_separator: ".".into(),
        //     }
        //     .format(&tree),
        //     "".to_string()
        // );
    }

    #[test]
    fn test_parse() {
        let parser = AsciiParser::default();
        let tree = parser.parse(&"2 / (sin mu + 1)".to_owned()).unwrap();
        assert_eq!(
            UnicodeFormatter::default().format(&tree),
            r"2 / (sin(μ) + 1)".to_string()
        );
        let tree = parser.parse(&"2 / sin mu * 1".to_owned()).unwrap();
        assert_eq!(
            UnicodeFormatter::default().format(&tree),
            r"(2 / sin(μ) · 1)".to_string()
        );
        let tree = parser.parse(&"2 / arccos mu + 1".to_owned()).unwrap();
        assert_eq!(
            UnicodeFormatter::default().format(&tree),
            r"(2 / arccos(μ) + 1)".to_string()
        );
    }
}
