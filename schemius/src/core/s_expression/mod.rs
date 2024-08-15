pub mod s_list;
pub mod s_number;
pub mod s_procedure;

use cfg_if::cfg_if;

use super::{accessor::*, constants::tokens};
use std::{collections::LinkedList, fmt, result, vec};

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

    pub fn is_quote(&self) -> Result<bool, String> {
        Ok(self.symbol_is(tokens::QUOTE).unwrap()
            || self.symbol_is(tokens::QUOTE_EXPLICIT).unwrap()
            || self.symbol_is(tokens::QUASIQUOTE).unwrap()
            || self.symbol_is(tokens::QUASIQUOTE_EXPLICIT).unwrap())
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

    pub fn matching_brackets(&self) -> Option<Vec<(usize, usize, usize)>> {
        match self {
            SExpr::List(list) => {
                let list = list.access();
                if !list.s_car().unwrap().symbol_is(tokens::OPEN_PAREN).unwrap() {
                    return None;
                }

                let mut pairs: Vec<(usize, usize)> = vec![];

                while let Some(right) = list
                    .iter()
                    .enumerate()
                    .filter(|x| {
                        (pairs.is_empty() || pairs.iter().all(|(_, right)| right != &x.0))
                            && x.1.symbol_is(tokens::CLOSED_PAREN).unwrap()
                    })
                    .min_by(|x, y| (x.0).cmp(&y.0))
                    .map(|x| x.0)
                {
                    match list
                        .iter()
                        .enumerate()
                        .filter(|x| {
                            (pairs.is_empty() || pairs.iter().all(|(left, _)| left != &x.0))
                                && x.1.symbol_is(tokens::OPEN_PAREN).unwrap()
                        })
                        .filter(|x| x.0 < right)
                        .max_by(|x, y| (x.0).cmp(&y.0))
                        .map(|x| x.0)
                    {
                        Some(left) => pairs.push((left, right)),
                        None => break,
                    }
                }

                let mut mapping: Vec<(usize, usize, usize)> = vec![];
                pairs
                    .iter()
                    .map(|(left, right)| {
                        if *left == 0 && *right == list.s_len() - 1 {
                            (left, right, 0)
                        } else {
                            (
                                left,
                                right,
                                pairs.iter().filter(|(l, r)| l < left && r > left).count(),
                            )
                        }
                    })
                    .for_each(|(left, right, level)| mapping.push((*left, *right, level)));

                Some(mapping)
            }
            _ => None,
        }
    }

    pub fn find_symbol(&self, symbol: &str) -> Option<Vec<usize>> {
        match self.with_explicit_parens() {
            Ok(SExpr::List(list)) => {
                let borrowed_list = list.access();

                if borrowed_list.s_car().unwrap().symbol_is(tokens::OPEN_PAREN).unwrap() {
                    return None;
                }

                let indexes: Vec<usize> = borrowed_list
                    .iter()
                    .enumerate()
                    .filter(|(_, x)| x.symbol_is(symbol).unwrap())
                    .map(|(i, _)| i - 1)
                    .collect();

                if !indexes.is_empty() {
                    Some(indexes)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn with_explicit_parens(&self) -> Result<SExpr, String> {
        match self {
            SExpr::List(list) => {
                let mut new_list = ListImplementation::new();
                new_list.push(SExpr::Symbol(String::from(tokens::OPEN_PAREN)));

                list.access().iter().for_each(|item| match item {
                    SExpr::List(_) => {
                        if let Ok(SExpr::List(internal)) = item.with_explicit_parens() {
                            internal.access().iter().for_each(|x| new_list.push(x.clone()))
                        }
                    }
                    other => new_list.push(other.clone()),
                });

                new_list.push(SExpr::Symbol(String::from(tokens::CLOSED_PAREN)));

                Ok(SExpr::List(SchemeList::new(new_list.clone())))
            }
            SExpr::Pair(pair) => {
                let pair = pair.access();
                SExpr::List(SchemeList::new(ListImplementation::from_iter([
                    *pair.0.clone(),
                    SExpr::Symbol(tokens::DOT.to_string()),
                    *pair.1.clone(),
                ])))
                .with_explicit_parens()
            }
            other => Ok(other.clone()),
        }
    }

    pub fn without_explicit_parens(&self) -> Result<SExpr, String> {
        // TODO: Deal with pairs, since flattening has been extended to them.
        match self {
            SExpr::List(list) => {
                let mut list_without_parens = VectorImplementation::new();
                list.access().iter().for_each(|expr| list_without_parens.push(expr.clone()));

                if !list_without_parens.first().unwrap().symbol_is(tokens::OPEN_PAREN).unwrap() {
                    return Ok(self.clone());
                }

                list_without_parens.remove(0);
                list_without_parens.pop();

                loop {
                    let l_index;
                    let r_index;

                    match list_without_parens
                        .iter()
                        .enumerate()
                        .filter(|x| x.1.symbol_is(tokens::CLOSED_PAREN).unwrap())
                        .min_by(|x, y| (x.0).cmp(&y.0))
                        .map(|x| x.0)
                    {
                        Some(r) => {
                            match list_without_parens
                                .iter()
                                .enumerate()
                                .filter(|x| x.1.symbol_is(tokens::OPEN_PAREN).unwrap())
                                .filter(|x| x.0 < r)
                                .max_by(|x, y| (x.0).cmp(&y.0))
                                .map(|x| x.0)
                            {
                                Some(l) => {
                                    l_index = l;
                                    r_index = r;
                                }
                                None => break,
                            }
                        }
                        None => break,
                    }

                    let internal = SExpr::Vector(SchemeVector::new(
                        list_without_parens[(l_index + 1)..r_index].to_vec(),
                    ));
                    list_without_parens.splice(l_index..(r_index + 1), [internal]);
                }

                Ok(SExpr::Vector(SAccessor::new(list_without_parens.clone())))
            }
            other => Ok(other.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::builtins::Primitive;

    use super::*;

    #[test]
    fn test_sexpr_as_char() {
        let sexpr = SExpr::Char('a');
        assert_eq!(sexpr.as_char().unwrap(), 'a');
    }

    #[test]
    fn test_sexpr_as_int() {
        let sexpr = SExpr::Number(SNumber::Int(42));
        assert_eq!(sexpr.as_int().unwrap(), 42);
    }

    #[test]
    fn test_sexpr_quote() {
        let expression = SExpr::Number(SNumber::Int(42));
        let quoted = expression.quote().unwrap();
        assert!(quoted.is_list().unwrap() && quoted.is_quoted_list().unwrap());
    }

    #[test]
    fn test_sexpr_unquote() {
        let internal = SExpr::Number(SNumber::Int(42));
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
            SExpr::Number(SNumber::Int(1)),
            SExpr::Number(SNumber::Int(2)),
        ])));
        assert!(sexpr.is_applyable().unwrap());
    }

    #[test]
    fn test_sexpr_is_atom() {
        let sexpr = SExpr::Number(SNumber::Int(NativeInt::from(3)));
        let is_atom = sexpr.is_atom().unwrap();
        assert!(is_atom)
    }
}
