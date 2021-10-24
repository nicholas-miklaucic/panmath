//! Defines operators for each symbol and their precedence.

use crate::ast::Symbol;
use crate::symbols;

/// An operator with a given left and right precedence. Precedence is defined as an `Option<u8>`
/// where 0 is the entire expression's precedence and lower values means higher-priority. `None`
/// indicates that the operator doesn't support that mode of operation.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Op {
    /// The symbol used to define the operator.
    pub sym: Symbol,

    /// The left precedence.
    pub l_prec: Option<u8>,

    /// The right precedence.
    pub r_prec: Option<u8>,
}

impl Op {
    /// Makes a new `Op`, cloning the symbol used.
    pub fn new(sym: &Symbol, l_prec: Option<u8>, r_prec: Option<u8>) -> Op {
        Op {
            sym: sym.clone(),
            l_prec,
            r_prec,
        }
    }

    /// Given a string, returns a matched prefix of that string if the prefix matches one of the
    /// operator's representations and None otherwise.
    pub fn match_front(&self, input: &str) -> Option<&str> {
        self.sym.match_front(input)
    }
}

lazy_static! {
    // Unary operators: these take precedence over binary operators and can't bind things to the
    // left of them.
    pub static ref UNARY_PLUS: Op = Op::new(&symbols::PLUS, None, Some(1));
    pub static ref UNARY_MINUS: Op = Op::new(&symbols::MINUS, None, Some(1));
    pub static ref UNARY_PM: Op = Op::new(&symbols::PM, None, Some(1));

    // Binary operators. We give the right sides higher precedence when the operator is associative
    // so they associate rightwards: `a + b + c` is parsed as `a + (b + c)`.

    // unlike the others, this one needs right precedence: 2 ^ 3 ^ 4 = 2 ^ (3 ^ 4) and not the other
    // way round!
    pub static ref POWER: Op = Op::new(&symbols::POWER, Some(4), Some(3));
    pub static ref MULT: Op = Op::new(&symbols::MULT, Some(6), Some(5));
    pub static ref DIV: Op = Op::new(&symbols::DIV, Some(6), Some(5));
    pub static ref ADD: Op = Op::new(&symbols::PLUS, Some(7), Some(8));
    pub static ref SUB: Op = Op::new(&symbols::MINUS, Some(7), Some(8));
    pub static ref PM: Op = Op::new(&symbols::PM, Some(7), Some(8));

    // Comma is an operator as a hacky way of allowing expressions like max(1 + 2, 3 + 4). It should
    // be the weakest operator, as the example shows: no matter what operator is used in place +,
    // the postfix version should be 1 2 + 3 4 + , max
    pub static ref COMMA: Op = Op::new(&symbols::COMMA, Some(10), Some(11));

    /// The list of unary operators.
    pub static ref UNARY_OPS: Vec<Op> = {
        vec![
            UNARY_PLUS.clone(),
            UNARY_MINUS.clone(),
            UNARY_PM.clone(),
        ]
    };

    /// The list of binary operators.
    pub static ref BINARY_OPS: Vec<Op> = {
        vec![
            POWER.clone(),
            MULT.clone(),
            DIV.clone(),
            ADD.clone(),
            SUB.clone(),
            PM.clone(),
            COMMA.clone()
        ]
    };
}
