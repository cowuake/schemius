pub mod tokens {
    pub const EOF: &str = "EOF";
    pub const OPEN_PAREN: &str = "(";
    pub const CLOSED_PAREN: &str = ")";
    pub const OPEN_BRACKET: &str = "[";
    pub const CLOSED_BRACKET: &str = "]";
    pub const PREFIX: &str = "#";
    pub const VECTOR_OPEN: &str = "#(";
    pub const QUOTE: &str = "'";
    pub const QUOTE_EXPLICIT: &str = "quote";
    pub const QUASIQUOTE: &str = "`";
    pub const QUASIQUOTE_EXPLICIT: &str = "quasiquote";
    pub const DOT: &str = ".";
    pub const UNQUOTE: &str = ",";
    pub const UNQUOTE_EXPLICIT: &str = "unquote";
    pub const UNQUOTE_SPLICING: &str = ",@";
    pub const UNQUOTE_SPLICING_EXPLICIT: &str = "unquote-splicing";
    pub const TRUE: &str = "#t";
    pub const FALSE: &str = "#f";
    pub const PREFIX_CHAR: &str = "#\\";
    pub const PREFIX_STRING: &str = "\"";
    pub const SUFFIX_STRING: &str = "\"";
    pub const PREFIX_COMMENT: &str = ";";
    pub const PREFIX_BINARY: &str = "#b";
    pub const PREFIX_OCTAL: &str = "#o";
    pub const PREFIX_HEX: &str = "#x";
    pub const PREFIX_DECIMAL: &str = "#d";
    pub const PREFIX_EXACT: &str = "#e";
    pub const PREFIX_INEXACT: &str = "#i";
    pub const POSITIVE_INFINITY: &str = "+inf.0";
    pub const NEGATIVE_INFINITY: &str = "-inf.0";
    pub const POSITIVE_NAN: &str = "+nan.0";
    pub const NEGATIVE_NAN: &str = "-nan.0";
}

pub mod numbers {
    pub const AVOGADRO: f64 = 6.0221515e23;
    pub const BOLTZMANN: f64 = 1.380650e23;
    pub const EULER: f64 = 2.718281828459045;
    pub const GOLDEN_RATIO: f64 = 1.618033988749895;
    pub const GRAVITATIONAL_CONSTANT: f64 = 6.67300e-11;
    pub const PI: f64 = 3.141592653589793;
    pub const PLANCK: f64 = 6.626068e-34;
}
