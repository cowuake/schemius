pub mod s_number;
pub mod s_procedure;
pub mod s_wrapper;

use std::fmt;

pub use self::{s_number::*, s_procedure::*, s_wrapper::*};

pub type List = SWrapper<Vec<SExpr>>;
pub type Pair = SWrapper<(Box<SExpr>, Box<SExpr>)>;
pub type SString = SWrapper<String>;
pub type Vector = SWrapper<Vec<SExpr>>;

#[derive(Clone, Debug)]
pub enum SExpr {
    Boolean(bool),
    Char(char),
    Symbol(String),
    String(SString),
    Number(SNumber),
    Pair(Pair),
    List(List),
    Vector(Vector),
    Procedure(Procedure),
    Unspecified,
    Ok,
}

impl fmt::Display for SExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SExpr::Symbol(ref val) => write!(f, "{}", val),
            SExpr::Char(val) => write!(f, "#\\{}", val),
            SExpr::Number(val) => write!(f, "{}", val),
            SExpr::Boolean(val) => write!(f, "#{}", if *val { "t" } else { "f" }),
            SExpr::String(ref val) => write!(f, "\"{}\"", val.borrow()),
            SExpr::Procedure(app) => match app {
                Procedure::SpecialForm(_) => write!(f, "#<special form>"),
                Procedure::Primitive(_) => write!(f, "#<primitive>"),
                Procedure::Compound(args, _, _) => write!(f, "#<procedure ({})>", args.join(", ")),
            },
            SExpr::Pair(val) => write!(f, "({} . {})", val.borrow().0, val.borrow().1),
            SExpr::List(ref val) => write!(f, "({})", val.borrow().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            SExpr::Vector(ref val) => write!(f, "#({})", val.borrow().iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")),
            SExpr::Unspecified => write!(f, "\n"),
            SExpr::Ok => write!(f, "ok"),
        }
    }
}

impl SExpr {
    pub fn is_symbol(&self, repr: Option<&str>) -> Result<bool, String> {
        match self {
            SExpr::Symbol(val) => match repr {
                Some(token) => {
                    if val.as_str() == token {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                None => Ok(true),
            },
            _ => Ok(false),
        }
    }

    pub fn is_char(&self) -> Result<bool, String> {
        match self {
            SExpr::Char(_) => Ok(true),
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

    pub fn matching_parens(&self) -> Option<Vec<(usize, usize)>> {
        match self {
            SExpr::List(list) => {
                if !list.borrow().first().unwrap().is_symbol(Some("(")).unwrap() {
                    return None;
                }

                let mut mapping: Vec<(usize, usize)> = vec![];

                while let Some(right) = list
                    .borrow()
                    .iter()
                    .enumerate()
                    .filter(|x| {
                        (mapping.is_empty() || mapping.iter().all(|pair: &(usize, usize)| pair.1 != x.0)) && x.1.is_symbol(Some(")")).unwrap()
                    })
                    .min_by(|x, y| (x.0).cmp(&y.0))
                    .map(|x| x.0)
                {
                    match list
                        .borrow()
                        .iter()
                        .enumerate()
                        .filter(|x| (mapping.is_empty() || mapping.iter().all(|pair| pair.0 != x.0)) && x.1.is_symbol(Some("(")).unwrap())
                        .filter(|x| x.0 < right)
                        .max_by(|x, y| (x.0).cmp(&y.0))
                        .map(|x| x.0)
                    {
                        Some(left) => mapping.push((left, right)),
                        None => break,
                    }
                }
                Some(mapping)
            }
            _ => None,
        }
    }

    pub fn find_symbol(&self, symbol: &str) -> Option<Vec<usize>> {
        match self.flatten() {
            Ok(list) => match list {
                SExpr::List(flattened) => {
                    if !flattened.borrow().first().unwrap().is_symbol(Some("(")).unwrap() {
                        return None;
                    }

                    let indexes: Vec<usize> =
                        flattened.borrow().iter().enumerate().filter(|(_, x)| x.is_symbol(Some(symbol)).unwrap()).map(|(i, _)| i - 1).collect();

                    if !indexes.is_empty() {
                        Some(indexes)
                    } else {
                        None
                    }
                }
                _ => None,
            },
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
                        if let Ok(res) = item.flatten() {
                            match res {
                                SExpr::List(internal) => internal.borrow().iter().for_each(|x| flattened.push(x.clone())),
                                _ => {}
                            }
                        }
                    }
                    other => flattened.push(other.clone()),
                });

                flattened.push(SExpr::Symbol(String::from(")")));

                Ok(SExpr::List(SWrapper::new(flattened.clone())))
            }
            SExpr::Pair(pair) => {
                SExpr::List(SWrapper::new(vec![*pair.borrow().0.clone(), SExpr::Symbol(".".to_string()), *pair.borrow().1.clone()])).flatten()
            }
            other => Ok(other.clone()),
        }
    }

    pub fn unflatten(&self) -> Result<SExpr, String> {
        // TODO: Deal with pairs, since flattening has been extended to them.
        match self {
            SExpr::List(list) => {
                if !list.borrow().first().unwrap().is_symbol(Some("(")).unwrap() {
                    return Ok(self.clone());
                }

                let unflattened = list.clone();
                unflattened.borrow_mut().remove(0);
                unflattened.borrow_mut().pop();

                loop {
                    let l_index;
                    let r_index;

                    match unflattened
                        .borrow()
                        .iter()
                        .enumerate()
                        .filter(|x| x.1.is_symbol(Some(")")).unwrap())
                        .min_by(|x, y| (x.0).cmp(&y.0))
                        .map(|x| x.0)
                    {
                        Some(r) => {
                            match unflattened
                                .borrow()
                                .iter()
                                .enumerate()
                                .filter(|x| x.1.is_symbol(Some("(")).unwrap())
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

                    let internal = SExpr::List(SWrapper::new(unflattened.borrow()[(l_index + 1)..r_index].to_vec()));
                    unflattened.borrow_mut().splice(l_index..(r_index + 1), [internal]);
                }

                Ok(SExpr::List(unflattened.clone()))
            }
            other => Ok(other.clone()),
        }
    }
}
