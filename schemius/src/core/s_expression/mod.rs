pub mod s_list;
pub mod s_number;
pub mod s_procedure;

use super::accessor::*;
use std::{fmt, result};

pub use self::{s_list::*, s_number::*, s_procedure::*};
type SAccessor<T> = ThreadSafeAccessor<T>;

pub type ListImplementation = Vec<SExpr>;

pub type SchemeBoolean = bool;
pub type SchemeChar = char;
pub type SchemeList = SAccessor<ListImplementation>;
pub type SchemeNumber = SNumber;
pub type SchemePair = SAccessor<(Box<SExpr>, Box<SExpr>)>;
pub type SchemeProcedure = Procedure;
pub type SchemeSymbol = String;
pub type SchemeString = SAccessor<String>;
pub type SchemeVector = SAccessor<Vec<SExpr>>;

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

pub struct Bracket;

impl Bracket {
    pub const LEFT_ROUND: &'static str = "(";
    pub const LEFT_SQUARE: &'static str = "[";
    pub const RIGHT_ROUND: &'static str = ")";
    pub const RIGHT_SQUARE: &'static str = "]";
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SExpr::Symbol(ref val) => write!(f, "{}", val),
            SExpr::Char(val) => write!(f, "#\\{}", val),
            SExpr::Number(val) => write!(f, "{}", val),
            SExpr::Boolean(val) => write!(f, "#{}", if *val { "t" } else { "f" }),
            SExpr::String(ref val) => write!(f, "\"{}\"", *val.borrow()),
            SExpr::Procedure(app) => match app {
                Procedure::SpecialForm(_) => write!(f, "#<special form>"),
                Procedure::Primitive(_) => write!(f, "#<primitive>"),
                Procedure::Compound(args, _, _) => write!(f, "#<procedure ({})>", args.join(", ")),
            },
            SExpr::Pair(val) => {
                let borrowed_val = val.borrow();
                write!(f, "({} . {})", borrowed_val.0, borrowed_val.1)
            }
            SExpr::List(ref val) => write!(
                f,
                "({})",
                val.borrow().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
            ),
            SExpr::Vector(ref val) => write!(
                f,
                "#({})",
                val.borrow().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")
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
            SExpr::List(list) => list.borrow().clone(),
            _ => panic!("Exception: {} is not a list", self),
        })
    }

    pub fn quote(&self) -> Result<SExpr, String> {
        Ok(SExpr::List(SchemeList::new(vec![SExpr::Symbol("quote".to_string()), self.clone()])))
    }

    pub fn unquote(&self) -> Result<SExpr, String> {
        match self {
            SExpr::List(list) => {
                let borrowed_list = list.borrow();
                if borrowed_list.first().unwrap().is_quote().unwrap() {
                    Ok(borrowed_list[1].clone())
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
        if self.symbol_is(Bracket::LEFT_ROUND).unwrap()
            || self.symbol_is(Bracket::LEFT_SQUARE).unwrap()
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    #[allow(dead_code)]
    fn is_right_bracket(&self) -> Result<bool, String> {
        if self.symbol_is(Bracket::RIGHT_ROUND).unwrap()
            || self.symbol_is(Bracket::RIGHT_SQUARE).unwrap()
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
            SExpr::List(list) if list.borrow().s_car().unwrap().is_procedure()? => Ok(true),
            _ => Ok(false),
        }
    }

    pub fn is_quote(&self) -> Result<bool, String> {
        Ok(self.symbol_is("'").unwrap()
            || self.symbol_is("quote").unwrap()
            || self.symbol_is("`").unwrap()
            || self.symbol_is("quasiquote").unwrap())
    }

    pub fn is_quoted_list(&self) -> Result<bool, String> {
        match self {
            SExpr::List(list) => {
                let borrowed = list.borrow();
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
                if list.borrow().is_empty() {
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
                let list = list.borrow();
                if !list.first().unwrap().symbol_is("(").unwrap() {
                    return None;
                }

                let mut pairs: Vec<(usize, usize)> = vec![];

                while let Some(right) = list
                    .iter()
                    .enumerate()
                    .filter(|x| {
                        (pairs.is_empty() || pairs.iter().all(|(_, right)| right != &x.0))
                            && x.1.symbol_is(")").unwrap()
                    })
                    .min_by(|x, y| (x.0).cmp(&y.0))
                    .map(|x| x.0)
                {
                    match list
                        .iter()
                        .enumerate()
                        .filter(|x| {
                            (pairs.is_empty() || pairs.iter().all(|(left, _)| left != &x.0))
                                && x.1.symbol_is("(").unwrap()
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
        match self.flatten() {
            Ok(SExpr::List(flattened)) => {
                let borrowed_flattened = flattened.borrow();

                if borrowed_flattened.first().unwrap().symbol_is("(").unwrap() {
                    return None;
                }

                let indexes: Vec<usize> = borrowed_flattened
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

    pub fn flatten(&self) -> Result<SExpr, String> {
        match self {
            SExpr::List(list) => {
                let mut flattened = vec![];
                flattened.push(SExpr::Symbol(String::from("(")));

                list.borrow().iter().for_each(|item| match item {
                    SExpr::List(_) => {
                        if let Ok(SExpr::List(internal)) = item.flatten() {
                            internal.borrow().iter().for_each(|x| flattened.push(x.clone()))
                        }
                    }
                    other => flattened.push(other.clone()),
                });

                flattened.push(SExpr::Symbol(String::from(")")));

                Ok(SExpr::List(SchemeList::new(flattened.clone())))
            }
            SExpr::Pair(pair) => {
                let pair = pair.borrow();
                SExpr::List(SchemeList::new(vec![
                    *pair.0.clone(),
                    SExpr::Symbol(".".to_string()),
                    *pair.1.clone(),
                ]))
                .flatten()
            }
            other => Ok(other.clone()),
        }
    }

    pub fn unflatten(&self) -> Result<SExpr, String> {
        // TODO: Deal with pairs, since flattening has been extended to them.
        match self {
            SExpr::List(list) => {
                let cloned = list.clone();
                let mut unflattened = cloned.borrow_mut();

                if !unflattened.first().unwrap().symbol_is("(").unwrap() {
                    return Ok(self.clone());
                }

                unflattened.remove(0);
                unflattened.pop();

                loop {
                    let l_index;
                    let r_index;

                    match unflattened
                        .iter()
                        .enumerate()
                        .filter(|x| x.1.symbol_is(")").unwrap())
                        .min_by(|x, y| (x.0).cmp(&y.0))
                        .map(|x| x.0)
                    {
                        Some(r) => {
                            match unflattened
                                .iter()
                                .enumerate()
                                .filter(|x| x.1.symbol_is("(").unwrap())
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

                    let internal =
                        SExpr::List(SchemeList::new(unflattened[(l_index + 1)..r_index].to_vec()));
                    unflattened.splice(l_index..(r_index + 1), [internal]);
                }

                Ok(SExpr::List(SAccessor::new(unflattened.clone())))
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
        let sexpr = SExpr::List(SchemeList::new(vec![
            SExpr::Procedure(Procedure::Primitive(Primitive::SUM)),
            SExpr::Number(SNumber::Int(1)),
            SExpr::Number(SNumber::Int(2)),
        ]));
        assert!(sexpr.is_applyable().unwrap());
    }
}
