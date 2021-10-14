#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod formatter;
pub mod formatters;
pub mod parser;
pub mod symbols;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
