use formatter::Formatter;
use parsers::ASTParser;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate nom;

pub mod ast;
pub mod formatter;
pub mod formatters;
pub mod operators;
// pub mod parser;
pub mod delimiter;
pub mod parsers;
pub mod symbols;

// Converts the input to TeX if possible.
pub fn texify(input: &str) -> Option<String> {
    let tree = parsers::AsciiParser::default().parse(&input).ok()?;
    Some(formatters::latex::LatexFormatter::default().format(&tree))
}

// Converts the input to Unicode if possible.
pub fn unicodeify(input: &str) -> Option<String> {
    let tree = parsers::AsciiParser::default().parse(&input).ok()?;
    Some(formatters::unicode::UnicodeFormatter::default().format(&tree))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
