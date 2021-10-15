use formatter::Formatter;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate nom;

pub mod ast;
pub mod formatter;
pub mod formatters;
pub mod parser;
pub mod symbols;

/// Converts the input to TeX if possible.
pub fn texify(input: &str) -> Option<String> {
    let (_rest, tree) = parser::parse_expr(input).ok()?;
    Some(formatters::latex::LatexFormatter::default().format(&tree))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
