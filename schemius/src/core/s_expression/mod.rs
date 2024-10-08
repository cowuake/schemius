pub mod s_list;
pub mod s_number;
pub mod s_procedure;

use cfg_if::cfg_if;

use super::{accessor::*, constants::tokens};
use std::{collections::LinkedList, fmt, result};

pub use self::{s_list::*, s_number::*, s_procedure::*};
type SAccessor<T> = ThreadSafeAccessor<T>;

cfg_if! {
    if #[cfg(feature = "true_list")] {
        pub type ListImplementation = LinkedList<SExpr>;
    } else {
        pub type ListImplementation = Vec<SExpr>;
    }
}

pub type PairImplementation = (Box<SExpr>, Box<SExpr>);
pub type VectorImplementation = Vec<SExpr>;

pub type SchemeBoolean = bool;
pub type SchemeChar = char;
pub type SchemeList = SAccessor<ListImplementation>;
pub type SchemeNumber = SNumber;
pub type SchemePair = SAccessor<PairImplementation>;
pub type SchemeProcedure = Procedure;
pub type SchemeSymbol = String;
pub type SchemeString = SAccessor<String>;
pub type SchemeVector = SAccessor<VectorImplementation>;

#[derive(Clone, Debug)]
pub enum SExpr {
    Boolean(SchemeBoolean),
    Char(SchemeChar),
    Symbol(SchemeSymbol),
    String(SchemeString),
    Number(SchemeNumber),
    Pair(SchemePair),
    List(SchemeList),
    Vector(SchemeVector),
    Procedure(SchemeProcedure),
    Unspecified,
    Ok,
}

macro_rules! impl_from_primitive {
    ($($source:ident, $target:ident)*) => {
    $(
        impl From<$source> for SExpr {
            fn from(val: $source) -> Self {
                SExpr::$target(val)
            }
        }
    )*}
}

macro_rules! impl_from_number {
    ($($source:ident, $target:ident)*) => {
    $(
        impl From<$source> for SExpr {
            fn from(val: $source) -> Self {
                SExpr::Number(SNumber::$target(val))
            }
        }
    )*}
}

impl_from_primitive! {
    bool, Boolean
    char, Char
}

impl_from_number! {
    NativeInt, Int
    NativeBigInt, BigInt
    NativeRational, Rational
    NativeComplex, Complex
    NativeFloat, Float
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SExpr::Symbol(ref val) => write!(f, "{}", val),
            SExpr::Char(val) => write!(f, "{}{}", tokens::PREFIX_CHAR, val),
            SExpr::Number(val) => write!(f, "{}", val),
            SExpr::Boolean(val) => {
                write!(f, "{}", if *val { tokens::TRUE } else { tokens::FALSE })
            }
            SExpr::String(ref val) => {
                write!(f, "{}{}{}", tokens::PREFIX_STRING, *val.access(), tokens::SUFFIX_STRING)
            }
            SExpr::Procedure(app) => match app {
                Procedure::SpecialForm(_) => write!(f, "#<special form>"),
                Procedure::Primitive(_) => write!(f, "#<primitive>"),
                Procedure::Compound(args, _, _) => write!(f, "#<procedure ({})>", args.join(", ")),
            },
            SExpr::Pair(val) => {
                let borrowed_val = val.access();
                write!(f, "({} {} {})", borrowed_val.0, tokens::DOT, borrowed_val.1)
            }
            SExpr::List(ref val) => write!(
                f,
                "({})",
                val.access().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
            ),
            SExpr::Vector(ref val) => write!(
                f,
                "#({})",
                val.access().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
            ),
            SExpr::Unspecified => writeln!(f),
            SExpr::Ok => write!(f, "ok"),
        }
    }
}

impl SExpr {
    pub fn as_int(&self) -> Result<NativeInt, String> {
        match self {
            SExpr::Number(n) => Ok(n.to_int()?),
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn as_char(&self) -> Result<SchemeChar, String> {
        match self {
            SExpr::Char(val) => Ok(*val as SchemeChar),
            _ => panic!("Exception: {} is not a character", self),
        }
    }

    pub fn as_symbol(&self) -> Result<SchemeSymbol, String> {
        match self {
            SExpr::Symbol(val) => Ok(val.clone()),
            _ => Err(format!("Exception: {} is not a symbol", self)),
        }
    }

    pub fn as_list(&self) -> Result<ListImplementation, String> {
        Ok(match self {
            SExpr::List(list) => list.access().clone(),
            _ => panic!("Exception: {} is not a list", self),
        })
    }

    pub fn quote(&self) -> Result<SExpr, String> {
        Ok(SExpr::List(SchemeList::new(ListImplementation::from_iter([
            SExpr::Symbol(tokens::QUOTE_EXPLICIT.to_string()),
            self.clone(),
        ]))))
    }

    pub fn unquote(&self) -> Result<SExpr, String> {
        match self {
            SExpr::List(list) => {
                let borrowed_list = list.access();
                if borrowed_list.s_car().unwrap().is_quote().unwrap() {
                    Ok(borrowed_list.s_ref(1).unwrap().clone())
                } else {
                    Err(format!("Exception: {} is not a quoted expression", self))
                }
            }
            _ => Err(format!("Exception: {} is not a list", self)),
        }
    }

    pub fn symbol_is(&self, repr: &str) -> Result<bool, String> {
        match self {
            SExpr::Symbol(val) => {
                if val.as_str() == repr {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }

    #[allow(dead_code)]
    fn is_left_bracket(&self) -> Result<bool, String> {
        if self.symbol_is(tokens::OPEN_PAREN).unwrap()
            || self.symbol_is(tokens::OPEN_BRACKET).unwrap()
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[allow(dead_code)]
    fn is_right_bracket(&self) -> Result<bool, String> {
        if self.symbol_is(tokens::CLOSED_PAREN).unwrap()
            || self.symbol_is(tokens::CLOSED_BRACKET).unwrap()
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn is_char(&self) -> Result<bool, String> {
        match self {
            SExpr::Char(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_symbol(&self) -> Result<bool, String> {
        match self {
            SExpr::Symbol(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_applyable(&self) -> Result<bool, String> {
        match self {
            SExpr::List(list) if list.access().s_car().unwrap().is_procedure()? => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_quote_pure(&self) -> Result<bool, String> {
        Ok(self.symbol_is(tokens::QUOTE)? || self.symbol_is(tokens::QUOTE_EXPLICIT)?)
    }

    pub fn is_quasiquote(&self) -> Result<bool, String> {
        Ok(self.symbol_is(tokens::QUASIQUOTE)? || self.symbol_is(tokens::QUASIQUOTE_EXPLICIT)?)
    }

    pub fn is_quote(&self) -> Result<bool, String> {
        Ok(self.is_quote_pure()? || self.is_quasiquote()?)
    }

    pub fn is_unquote_pure(&self) -> Result<bool, String> {
        Ok(self.symbol_is(tokens::UNQUOTE)? || self.symbol_is(tokens::UNQUOTE_EXPLICIT)?)
    }

    pub fn is_unquote_splicing(&self) -> Result<bool, String> {
        Ok(self.symbol_is(tokens::UNQUOTE_SPLICING)?
            || self.symbol_is(tokens::UNQUOTE_SPLICING_EXPLICIT)?)
    }

    pub fn is_unquote(&self) -> Result<bool, String> {
        Ok(self.is_unquote_pure()? || self.is_unquote_splicing()?)
    }

    pub fn is_quoted_list(&self) -> Result<bool, String> {
        match self {
            SExpr::List(list) => {
                let borrowed = list.access();
                if borrowed.s_len() > 0 {
                    let car = borrowed.s_car().unwrap();
                    match car {
                        SExpr::Symbol(_) => Ok(car.is_quote().unwrap()),
                        _ => Ok(false),
                    }
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }

    pub fn is_string(&self) -> Result<bool, String> {
        match self {
            SExpr::String(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_boolean(&self) -> Result<bool, String> {
        match self {
            SExpr::Boolean(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_number(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_exact(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(n) => {
                if n.is_exact() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn is_integer(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(n) => {
                if n.is_integer() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn is_real(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(n) => {
                if n.is_real() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn is_rational(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(n) => {
                if n.is_rational() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn is_complex(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(n) => {
                if n.is_complex() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn is_zero(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(n) => {
                if n.is_zero() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn is_nan(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(n) => {
                if n.is_nan() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn is_infinite(&self) -> Result<bool, String> {
        match self {
            SExpr::Number(n) => {
                if n.is_infinite() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Err(format!("Exception: {} is not a number", self)),
        }
    }

    pub fn is_pair(&self) -> Result<bool, String> {
        match self {
            SExpr::Pair(_) => Ok(true),
            SExpr::List(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_atom(&self) -> Result<bool, String> {
        Ok(!self.is_pair()?)
    }

    pub fn is_list(&self) -> Result<bool, String> {
        match self {
            SExpr::List(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_vector(&self) -> Result<bool, String> {
        match self {
            SExpr::Vector(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_procedure(&self) -> Result<bool, String> {
        match self {
            SExpr::Procedure(_) => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_null(&self) -> result::Result<bool, String> {
        match self {
            SExpr::List(list) => {
                if list.access().is_empty() {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            _ => Ok(false),
        }
    }
}

#[cfg(test)]
mod s_expression_tests {
    use crate::core::builtins::Primitive;

    use super::*;

    #[test]
    fn test_sexpr_as_char() {
        let sexpr = SExpr::Char('a');
        assert_eq!(sexpr.as_char().unwrap(), 'a');
    }

    #[test]
    fn test_sexpr_as_symbol() {
        let sexpr = SExpr::Symbol("symbol".to_string());
        assert_eq!(sexpr.as_symbol().unwrap(), "symbol");
    }

    #[test]
    fn test_sexpr_as_int() {
        let sexpr = SExpr::from(42);
        assert_eq!(sexpr.as_int().unwrap(), 42);
    }

    #[test]
    fn test_sexpr_quote() {
        let expression = SExpr::from(42);
        let quoted = expression.quote().unwrap();
        assert!(quoted.is_list().unwrap() && quoted.is_quoted_list().unwrap());
    }

    #[test]
    fn test_sexpr_unquote() {
        let internal = SExpr::from(42);
        let expression = internal.quote().unwrap();
        let unquoted = expression.unquote().unwrap();
        assert!(unquoted.is_number().unwrap());
    }

    #[test]
    fn test_sexpr_is_procedure() {
        let sexpr = SExpr::Procedure(Procedure::Primitive(Primitive::SUM));
        assert!(sexpr.is_procedure().unwrap());
    }

    #[test]
    fn test_sexpr_is_applyable() {
        let sexpr = SExpr::List(SchemeList::new(ListImplementation::from_iter([
            SExpr::Procedure(Procedure::Primitive(Primitive::SUM)),
            SExpr::from(1),
            SExpr::from(2),
        ])));
        assert!(sexpr.is_applyable().unwrap());
    }

    #[test]
    fn test_sexpr_is_quote() {
        let quote = SExpr::Symbol(tokens::QUOTE.to_string());
        let quote_explicit = SExpr::Symbol(tokens::QUOTE_EXPLICIT.to_string());
        let quasiquote = SExpr::Symbol(tokens::QUASIQUOTE.to_string());
        let quasiquote_explicit = SExpr::Symbol(tokens::QUASIQUOTE_EXPLICIT.to_string());
        let symbol = SExpr::Symbol("symbol".to_string());
        let number = SExpr::from(42);

        assert!(quote.is_quote().unwrap());
        assert!(quote.is_quote_pure().unwrap());
        assert!(!quote.is_quasiquote().unwrap());

        assert!(quote_explicit.is_quote().unwrap());
        assert!(quote_explicit.is_quote_pure().unwrap());
        assert!(!quote_explicit.is_quasiquote().unwrap());

        assert!(quasiquote.is_quote().unwrap());
        assert!(quasiquote.is_quasiquote().unwrap());
        assert!(!quasiquote.is_quote_pure().unwrap());

        assert!(quasiquote_explicit.is_quote().unwrap());
        assert!(quasiquote_explicit.is_quasiquote().unwrap());
        assert!(!quasiquote_explicit.is_quote_pure().unwrap());

        assert!(!symbol.is_quote().unwrap());
        assert!(!symbol.is_quote_pure().unwrap());
        assert!(!symbol.is_quasiquote().unwrap());

        assert!(!number.is_quasiquote().unwrap());
        assert!(!number.is_quasiquote().unwrap());
        assert!(!number.is_quasiquote().unwrap());
    }

    #[test]
    fn test_sexpr_is_unquote() {
        let unquote = SExpr::Symbol(tokens::UNQUOTE.to_string());
        let unquote_explicit = SExpr::Symbol(tokens::UNQUOTE_EXPLICIT.to_string());
        let unquote_splicing = SExpr::Symbol(tokens::UNQUOTE_SPLICING.to_string());
        let unquote_splicing_explicit =
            SExpr::Symbol(tokens::UNQUOTE_SPLICING_EXPLICIT.to_string());
        let symbol = SExpr::Symbol("symbol".to_string());
        let number = SExpr::from(42);

        assert!(unquote.is_unquote().unwrap());
        assert!(unquote.is_unquote_pure().unwrap());
        assert!(!unquote.is_unquote_splicing().unwrap());

        assert!(unquote_explicit.is_unquote().unwrap());
        assert!(unquote_explicit.is_unquote_pure().unwrap());
        assert!(!unquote_explicit.is_unquote_splicing().unwrap());

        assert!(unquote_splicing.is_unquote().unwrap());
        assert!(unquote_splicing.is_unquote_splicing().unwrap());
        assert!(!unquote_splicing.is_unquote_pure().unwrap());

        assert!(unquote_splicing_explicit.is_unquote().unwrap());
        assert!(unquote_splicing_explicit.is_unquote_splicing().unwrap());
        assert!(!unquote_splicing_explicit.is_unquote_pure().unwrap());

        assert!(!symbol.is_unquote().unwrap());
        assert!(!symbol.is_unquote_pure().unwrap());
        assert!(!symbol.is_unquote_splicing().unwrap());

        assert!(!number.is_unquote().unwrap());
        assert!(!number.is_unquote_pure().unwrap());
        assert!(!number.is_unquote_splicing().unwrap());
    }

    #[test]
    fn test_sexpr_is_atom() {
        let sexpr = SExpr::from(3);
        let is_atom = sexpr.is_atom().unwrap();
        assert!(is_atom)
    }

    #[test]
    fn test_sexpr_from_int() {
        let sexpr = SExpr::from(42);
        assert!(sexpr.is_number().unwrap());
    }

    #[test]
    fn test_sexpr_from_big_int() {
        let sexpr = SExpr::from(NativeBigInt::from(42));
        assert!(sexpr.is_number().unwrap());
    }

    #[test]
    fn test_sexpr_from_rational() {
        let sexpr = SExpr::from(NativeRational::new(NativeBigInt::from(1), NativeBigInt::from(2)));
        assert!(sexpr.is_number().unwrap());
    }

    #[test]
    fn test_sexpr_from_float() {
        let sexpr = SExpr::from(NativeFloat::from(1.0));
        assert!(sexpr.is_number().unwrap());
    }

    #[test]
    fn test_sexpr_from_complex() {
        let sexpr = SExpr::from(NativeComplex::new(NativeFloat::from(1.0), NativeFloat::from(2.0)));
        assert!(sexpr.is_number().unwrap());
    }

    #[test]
    fn test_sexpr_from_char() {
        let sexpr = SExpr::from('a');
        assert!(sexpr.is_char().unwrap());
    }

    #[test]
    fn test_sexpr_from_bool() {
        let sexpr = SExpr::from(true);
        assert!(sexpr.is_boolean().unwrap());
    }
}
