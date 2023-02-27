//! This module provides an ergonomic way of defining new symbols and building up a library of
//! mathematical symbols to recognize and parse without custom specifications. Specifically, the
//! approach this module takes is to split symbols into several distinct types that share a common
//! structure, and then to implement generic Symbol conversions for those more specific types. This
//! gives us the flexibility of Symbol when we need it, but allows us to save a lot of boilerplate
//! when defining, for instance, every single trig function or letter.

use crate::ast::Symbol;

use std::collections::{BTreeMap, HashMap};
use std::string::ToString;
use strum::{EnumProperty, IntoEnumIterator};
use strum_macros::{Display, EnumIter, EnumProperty, EnumString};

/// A special function. These all share a couple traits. They have LaTeX commands that write them in
/// roman type, not italic as is standard for other functions; they otherwise have standard names
/// without other mathematical expressions; they often drop their parentheses. This last part is not
/// implemented currently, but might be in the future.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct SpecialFunction(String);

impl SpecialFunction {
    /// Produces a symbol for the square of the function.
    fn square(&self) -> Symbol {
        Symbol {
            unicode_repr: format!("{}²", self.0),
            ascii_repr: format!("{}^2", self.0),
            latex_repr: format!("\\{}^2", self.0),
            other_reprs: vec![],
        }
    }

    /// Produces a symbol for the inverse of the function.
    fn inv(&self) -> Symbol {
        Symbol {
            unicode_repr: format!("{}⁻¹", self.0),
            ascii_repr: format!("{}^-1", self.0),
            latex_repr: format!("\\{}^{{-1}}", self.0),
            other_reprs: vec![],
        }
    }
}

impl From<SpecialFunction> for Symbol {
    fn from(func: SpecialFunction) -> Self {
        Symbol {
            unicode_repr: func.0.clone(),
            ascii_repr: func.0.clone(),
            latex_repr: format!("\\{}", func.0),
            other_reprs: vec![],
        }
    }
}

/// A Greek letter with both uppercase and lowercase representations.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, EnumString, EnumProperty, Display, EnumIter)]
pub enum GreekLetter {
    #[strum(props(Lower = "α", Upper = "Α"))]
    Alpha,
    #[strum(props(Lower = "β", Upper = "Β"))]
    Beta,
    #[strum(props(Lower = "γ", Upper = "Γ"))]
    Gamma,
    #[strum(props(Lower = "δ", Upper = "Δ"))]
    Delta,
    #[strum(props(Lower = "ε", Upper = "Ε"))]
    Epsilon,
    #[strum(props(Lower = "ζ", Upper = "Ζ"))]
    Zeta,
    #[strum(props(Lower = "η", Upper = "Η"))]
    Eta,
    #[strum(props(Lower = "θ", Upper = "Θ"))]
    Theta,
    #[strum(props(Lower = "ι", Upper = "Ι"))]
    Iota,
    #[strum(props(Lower = "κ", Upper = "Κ"))]
    Kappa,
    #[strum(props(Lower = "λ", Upper = "Λ"))]
    Lambda,
    #[strum(props(Lower = "μ", Upper = "Μ"))]
    Mu,
    #[strum(props(Lower = "ν", Upper = "Ν"))]
    Nu,
    #[strum(props(Lower = "ξ", Upper = "Ξ"))]
    Xi,
    #[strum(props(Lower = "ο", Upper = "Ο"))]
    Omicron,
    #[strum(props(Lower = "π", Upper = "Π"))]
    Pi,
    #[strum(props(Lower = "ρ", Upper = "Ρ"))]
    Rho,
    #[strum(props(Lower = "σ", Upper = "Σ"))]
    Sigma,
    #[strum(props(Lower = "τ", Upper = "Τ"))]
    Tau,
    #[strum(props(Lower = "υ", Upper = "Υ"))]
    Upsilon,
    #[strum(props(Lower = "φ", Upper = "Φ"))]
    Phi,
    #[strum(props(Lower = "χ", Upper = "Χ"))]
    Chi,
    #[strum(props(Lower = "ψ", Upper = "Ψ"))]
    Psi,
    #[strum(props(Lower = "ω", Upper = "Ω"))]
    Omega,
}

/// The case of a greek letter: uppercase or lowercase.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, EnumIter)]
pub enum Case {
    /// Uppercase.
    Uppercase,
    /// Lowercase.
    Lowercase,
}

/// A Greek letter with a specified case.
pub struct CasedGreekLetter {
    /// The letter.
    pub letter: GreekLetter,

    /// The letter's case.
    pub case: Case,
}

impl From<CasedGreekLetter> for Symbol {
    fn from(cased: CasedGreekLetter) -> Self {
        let unicode = match cased.case {
            Case::Uppercase => cased.letter.get_str("Upper").unwrap(),
            Case::Lowercase => cased.letter.get_str("Lower").unwrap(),
        };

        let letter = cased.letter.to_string();
        let (ascii_start, ascii_rest) = letter.split_at(1);
        let ascii_name = match cased.case {
            Case::Uppercase => format!("{}{}", ascii_start.to_uppercase(), ascii_rest),
            Case::Lowercase => format!("{}{}", ascii_start.to_lowercase(), ascii_rest),
        };

        Symbol {
            unicode_repr: unicode.to_string(),
            ascii_repr: ascii_name.clone(),
            latex_repr: format!("\\{}", ascii_name),
            other_reprs: vec![],
        }
    }
}

// General implementation of Symbol for any identifier. Outputs might break if you put in special
// characters: this is intended to make it easy to get a symbol for x, not to encode some crazy
// LaTeX thing.
impl From<String> for Symbol {
    fn from(sym: String) -> Self {
        Symbol {
            unicode_repr: sym.clone(),
            ascii_repr: sym.clone(),
            latex_repr: sym.clone(),
            other_reprs: vec![],
        }
    }
}

/// See From<String>.
impl From<&str> for Symbol {
    fn from(sym: &str) -> Self {
        Symbol {
            unicode_repr: sym.to_string(),
            ascii_repr: sym.to_string(),
            latex_repr: sym.to_string(),
            other_reprs: vec![],
        }
    }
}

lazy_static! {
    /// All of the Greek letters, as Symbols that intelligently parse and display. They are keyed by
    /// their ASCII representation, which is capitalized if the letter is uppercase and lowercase
    /// otherwise. `Pi` maps to Π, and `pi` maps to π.
    pub static ref GREEK_SYMBOLS: HashMap<String, Symbol> = {
        let mut syms: HashMap<String, Symbol> = HashMap::new();
        for letter in GreekLetter::iter() {
            for case in Case::iter() {
                let sym: Symbol = CasedGreekLetter { letter, case }.into();
                syms.insert(sym.ascii_repr.clone(), sym);
            }
        }
        syms
    };

    /// All of the Latin symbols that come pre-defined. They're indexed by their ASCII
    /// representation, which is the only one they have: pretty straightforward.
    pub static ref LATIN_SYMBOLS: HashMap<String, Symbol> = {
        let mut syms: HashMap<String, Symbol> = HashMap::new();
        let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
        for letter in alphabet {
            syms.insert(letter.to_string(), letter.to_string().into());
        }
        syms
    };

    /// The special functions that come predefined. These are indexed by their normal name. The
    /// current special functions are:
    ///  - `exp`, `log`, `ln`, `lg`
    ///  - The standard trig functions `sin`, `cos`, `tan`, `sec`, `csc`, `cot`
    ///  - The inverse functions defined in `amsmath`: `arcsin`, `arccos`, `arctan`
    ///  - The hyperbolic trig functions defined in `amsmath`: `sinh`, `cosh`, `tanh`, `coth`. Don't
    ///  ask me why they have four defined, not three or six!
    ///  - `max`, `min`
    ///  - `Pr`
    ///  - `gcd`
    ///  - `det`, `dim`, `ker`
    ///  - `inf`, `sup`
    /// `amsmath` is very inconsistent, as you can see. I've only included the operators that
    /// might be used in plaintext: limits, for example, aren't parseable using standard function
    /// syntax.
    pub static ref SPECIAL_FUNCS: BTreeMap<String, Symbol> = {
        let mut syms: BTreeMap<String, Symbol> = BTreeMap::new();
        let names = vec![
            "exp", "log", "ln", "lg",
            "sin", "cos", "tan", "sec", "csc", "cot",
            "arcsin", "arccos", "arctan",
            "sinh", "cosh", "tanh", "coth",
            "max", "min",
            "Pr",
            "gcd",
            "det", "dim", "ker",
            "inf", "sup"
        ];
        for name in names {
            let spf = SpecialFunction(name.to_string());
            syms.insert(format!("{}^2", name), spf.square());
            syms.insert(format!("{}^-1", name), spf.inv());
            syms.insert(name.to_string(), spf.into());
        }
        syms
    };

    /**
    Unfortunately, the rest of the symbols are a lot more idiosyncratic, without the clearer
    patterns that allowed me to save a lot of boilerplate. The goal of these miscellaneous
    symbols is to cover the bases of the most common abbreviations and ASCII versions of common
    symbols, not to be complete or perfect. This will be updated over time to reflect usage.
    Because of that, these are provided as individual variables, so you can make sure that
    you'll get compile errors if you use symbols that don't exist and you can get tab
    completion.
     */

    /// The ≤ (less than or equal to) symbol.
    pub static ref LE: Symbol = Symbol::new("≤", "<=", r"\le", vec![" le"]);
    /// The ≥ (greater than or equal to) symbol.
    pub static ref GE: Symbol = Symbol::new("≥", ">=", r"\ge", vec![" ge"]);
    /// The ≠ (not equal to) symbol.
    pub static ref NEQ: Symbol = Symbol::new("≠", "!=", r"\neq", vec!["=/=", "/=", " neq"]);
    /// The + symbol.
    pub static ref PLUS: Symbol = Symbol::new("+", "+", "+", vec!["plus"]);
    /// The - symbol.
    pub static ref MINUS: Symbol = Symbol::new("-", "−", "-", vec!["minus"]);
    /// The ± (plus or minus) symbol.
    pub static ref PM: Symbol = Symbol::new("±", "+/-", r"\pm", vec!["+-", " pm"]);
    /// The exponentiation symbol. This is not the binary XOR function, and is
    /// also not used generically: exponentiation is special-cased.
    pub static ref POWER: Symbol = Symbol::new("^", "^", r"\^{}", vec![]);

    /// The division symbol. This is not the set difference or quotient group,
    /// and generally using fractions is preferred.
    pub static ref DIV: Symbol = Symbol::new("/", "/", r"/", vec![]);

    // The ∞ (infinity) symbol.
    pub static ref INF: Symbol = Symbol::new("∞", " inf", r"\infty", vec!["infinity", "oo"]);
    /// The ∈ (element of) symbol.
    // the question is whether to add E here so a E A becomes a ∈ A. I think it's about 50/50 in the
    // server on whether people do this or not, so I've left it out.
    pub static ref ELEM: Symbol = Symbol::new("∈", " in", r"\in", vec![" elem"]);
    /// The ∼ (distributed as) symbol.
    pub static ref SYM: Symbol = Symbol::new("∼", "~", r"\sym", vec![]);
    /// The ≅ (approximately equal to) symbol.
    pub static ref APPROX: Symbol = Symbol::new("≅", "~=", r"\approx", vec![]);
    /// The multiplication symbol, using a dot instead of the times operator.
    pub static ref MULT: Symbol = Symbol::new("·", "*", r"\cdot", vec![" times", "\times", "×"]);
    /// The ° (degrees) symbol.
    pub static ref DEGREE: Symbol = Symbol::new("°", "o", r"^{\circ}", vec![" deg", " degrees"]);
    /// The left parenthesis `(``.
    pub static ref LEFT_PAR: Symbol = Symbol::new("(", "(", r"\left(", vec![]);
    /// The right parenthesis `)``.
    pub static ref RIGHT_PAR: Symbol = Symbol::new(")", ")", r"\right)", vec![]);
    /// The left bracket `[``.
    pub static ref LEFT_BRACKET: Symbol = Symbol::new("[", "[", r"\left[", vec![]);
    /// The right bracket `]``.
    pub static ref RIGHT_BRACKET: Symbol = Symbol::new("]", "]", r"\right]", vec![]);

    // The comma symbol, needed for variadic functions.
    pub static ref COMMA: Symbol = Symbol::from(",");

    // TODO add more

    /// The delimiters.
    pub static ref DELIMS: Vec<Symbol> = {
        vec![
            LEFT_PAR.clone(),
            RIGHT_PAR.clone()
        ]
    };


    /// The miscellaneous symbols.
    pub static ref MISC: Vec<Symbol> = {
        vec![
            LE.clone(),
            GE.clone(),
            NEQ.clone(),
            PM.clone(),
            INF.clone(),
            ELEM.clone(),
            SYM.clone(),
            APPROX.clone(),
            MULT.clone(),
            DEGREE.clone(),
        ]
    };


    /// All of the symbols that come pre-defined. This order controls the preference for parsing: if
    /// multiple symbols share a representation, the one that comes first matches.
    pub static ref ALL_SYMBOLS: Vec<Symbol> = {
        let mut symbols = vec![];
        for (_k, sym) in GREEK_SYMBOLS.clone().drain() {
            symbols.push(sym);
        }
        for (_k, sym) in LATIN_SYMBOLS.clone().drain() {
            symbols.push(sym);
        }
        for (_k, sym) in SPECIAL_FUNCS.clone().into_iter() {
            symbols.push(sym);
        }
        symbols.extend_from_slice(&*MISC.as_slice());
        symbols
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_greek_letters() {
        assert_eq!(GREEK_SYMBOLS.len(), 48);
    }

    #[test]
    fn test_all_latin_letters() {
        assert_eq!(LATIN_SYMBOLS.len(), 52);
    }

    #[test]
    fn test_greek_letters() {
        let sym1: Symbol = CasedGreekLetter {
            letter: GreekLetter::Phi,
            case: Case::Lowercase,
        }
        .into();

        assert_eq!(sym1.unicode_repr, "φ");
        assert_eq!(sym1.ascii_repr, "phi");
        assert_eq!(sym1.latex_repr, r"\phi");

        let sym2: Symbol = CasedGreekLetter {
            letter: GreekLetter::Sigma,
            case: Case::Uppercase,
        }
        .into();

        assert_eq!(sym2.unicode_repr, "Σ");
        assert_eq!(sym2.ascii_repr, "Sigma");
        assert_eq!(sym2.latex_repr, r"\Sigma");
    }
}
