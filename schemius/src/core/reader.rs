use lazy_static::lazy_static;
use num::Num;
use regex::Regex;

use super::{accessor::Accessor, constants::tokens, s_expression::*};

lazy_static! {
    static ref TOKEN_REGEX: Regex =
        Regex::new(r#"^\s*(,@|#\\\.|[\[('`,)\]]|#\(|"(?:\.|[^"])*"|;.*|[^\s\[('"`,;)\]]*)(.*)"#)
            .unwrap();
}

lazy_static! {
    static ref COMPLEX_POLAR_REGEX: Regex =
        Regex::new(r"^(\d*(\.\d+)?(/?\d*(\.\d+)?)?)@(\d*(\.\d+)?(/?\d*(\.\d+)?)?)$").unwrap();
}

pub fn read(line: &mut String) -> Result<SExpr, String> {
    if !has_balanced_parentheses(line) {
        return Err("Exception: Invalid syntax: Unbalanced parentheses.".to_string());
    }

    let first_token = init(line);
    advance(line, &first_token)
}

fn has_balanced_parentheses(s: &str) -> bool {
    let mut balance = 0;
    for c in s.chars() {
        match c.to_string().as_str() {
            tokens::OPEN_PAREN | tokens::OPEN_BRACKET => balance += 1,
            tokens::CLOSED_PAREN | tokens::CLOSED_BRACKET => balance -= 1,
            _ => {}
        }
        if balance < 0 {
            // If balance is negative, there are more ')' than '(' at some point.
            return false;
        }
    }
    balance == 0 // True if balanced, false otherwise.
}

fn init(line: &mut String) -> String {
    let current_line: String = line.clone();

    match TOKEN_REGEX.captures(&current_line) {
        Some(x) => {
            line.clear();
            line.push_str(x.get(2).unwrap().as_str());

            x.get(1).unwrap().as_str().to_string()
        }
        None => String::new(),
    }
}

fn advance(line: &mut String, string_token: &String) -> Result<SExpr, String> {
    let opening_token = string_token.as_str();

    match opening_token {
        tokens::OPEN_PAREN | tokens::OPEN_BRACKET | tokens::VECTOR_OPEN => {
            let mut new_list = VectorImplementation::new();

            loop {
                let token: String = init(line);

                if (opening_token == tokens::OPEN_PAREN && token == tokens::CLOSED_PAREN)
                    || (opening_token == tokens::OPEN_BRACKET && token == tokens::CLOSED_BRACKET)
                {
                    if new_list.len() == 3 && token != tokens::CLOSED_BRACKET {
                        if let SExpr::Symbol(sym) = &new_list.s_ref(1).unwrap() {
                            if sym.as_str() == tokens::DOT {
                                return Ok(SExpr::Pair(SchemePair::new((
                                    Box::new(new_list.s_ref(0).unwrap().clone()),
                                    Box::new(new_list.s_ref(2).unwrap().clone()),
                                ))));
                            }
                        }
                    }

                    return Ok(SExpr::List(SchemeList::new(ListImplementation::from_iter(
                        new_list,
                    ))));
                } else if opening_token == tokens::VECTOR_OPEN && token == tokens::CLOSED_PAREN {
                    return Ok(SExpr::Vector(SchemeVector::new(VectorImplementation::from(
                        new_list,
                    ))));
                } else {
                    new_list.push(advance(line, &token)?);
                }
            }
        }
        _ => parse_token(line, string_token),
    }
}

fn parse_token(line: &mut String, token: &str) -> Result<SExpr, String> {
    match token {
        tokens::TRUE => Ok(SExpr::Boolean(true)),
        tokens::FALSE => Ok(SExpr::Boolean(false)),
        tokens::NEGATIVE_NAN | tokens::POSITIVE_NAN => Ok(SExpr::from(NativeFloat::NAN)),
        tokens::NEGATIVE_INFINITY => Ok(SExpr::from(NativeFloat::NEG_INFINITY)),
        tokens::POSITIVE_INFINITY => Ok(SExpr::from(NativeFloat::INFINITY)),
        token if token.starts_with('"') => {
            Ok(SExpr::String(SchemeString::new(token.get(1..token.len() - 1).unwrap().to_string())))
        }
        tokens::QUOTE | tokens::QUASIQUOTE | tokens::UNQUOTE | tokens::UNQUOTE_SPLICING => {
            let internal_token = init(line);
            let quoted = advance(line, &internal_token)?;
            let mut vec = ListImplementation::new();

            let string_token = match token {
                tokens::QUOTE => tokens::QUOTE_EXPLICIT.to_string(),
                tokens::QUASIQUOTE => tokens::QUASIQUOTE_EXPLICIT.to_string(),
                tokens::UNQUOTE => tokens::UNQUOTE_EXPLICIT.to_string(),
                tokens::UNQUOTE_SPLICING => tokens::UNQUOTE_SPLICING_EXPLICIT.to_string(),
                _ => token.to_string(),
            };

            vec.push(SExpr::Symbol(string_token));
            vec.push(quoted);
            Ok(SExpr::List(SchemeList::new(vec)))
        }
        token if token.starts_with(tokens::PREFIX_CHAR) => {
            match token.len() {
                3 => Ok(SExpr::Char(token.chars().last().unwrap())),
                _ => {
                    // Handle invalid character token
                    // Return an appropriate value or raise an error
                    // For now, returning SExpr::Symbol(token.clone())
                    Ok(SExpr::Symbol(token.to_string()))
                }
            }
        }
        _ => {
            let n_prefixes = if token.len() > 2
                && token.chars().nth(0) == tokens::PREFIX.chars().next()
                && token.chars().nth(1).is_some_and(|c| c.is_alphabetic())
            {
                if token.len() > 4
                    && token.chars().nth(2) == Some('#')
                    && token.chars().nth(3).is_some_and(|c| c.is_alphabetic())
                {
                    2
                } else {
                    1
                }
            } else {
                0
            };

            let (has_prefix, radix, is_exact) = if n_prefixes == 2 {
                match (&token[0..2], &token[2..4]) {
                    (tokens::PREFIX_BINARY, tokens::PREFIX_EXACT)
                    | (tokens::PREFIX_EXACT, tokens::PREFIX_BINARY) => (true, 2, Some(true)),
                    (tokens::PREFIX_BINARY, tokens::PREFIX_INEXACT)
                    | (tokens::PREFIX_INEXACT, tokens::PREFIX_BINARY) => (true, 2, Some(false)),
                    (tokens::PREFIX_OCTAL, tokens::PREFIX_EXACT)
                    | (tokens::PREFIX_EXACT, tokens::PREFIX_OCTAL) => (true, 8, Some(true)),
                    (tokens::PREFIX_OCTAL, tokens::PREFIX_INEXACT)
                    | (tokens::PREFIX_INEXACT, tokens::PREFIX_OCTAL) => (true, 8, Some(false)),
                    (tokens::PREFIX_DECIMAL, tokens::PREFIX_EXACT)
                    | (tokens::PREFIX_EXACT, tokens::PREFIX_DECIMAL) => (true, 10, Some(true)),
                    (tokens::PREFIX_DECIMAL, tokens::PREFIX_INEXACT)
                    | (tokens::PREFIX_INEXACT, tokens::PREFIX_DECIMAL) => (true, 10, Some(false)),
                    (tokens::PREFIX_HEX, tokens::PREFIX_EXACT)
                    | (tokens::PREFIX_EXACT, tokens::PREFIX_HEX) => (true, 16, Some(true)),
                    (tokens::PREFIX_HEX, tokens::PREFIX_INEXACT)
                    | (tokens::PREFIX_INEXACT, tokens::PREFIX_HEX) => (true, 16, Some(false)),
                    _ => (false, 10, None),
                }
            } else if n_prefixes == 1 {
                match &token[0..2] {
                    tokens::PREFIX_BINARY => (true, 2, None),
                    tokens::PREFIX_OCTAL => (true, 8, None),
                    tokens::PREFIX_DECIMAL => (true, 10, None),
                    tokens::PREFIX_HEX => (true, 16, None),
                    tokens::PREFIX_EXACT => (true, 10, Some(true)),
                    tokens::PREFIX_INEXACT => (true, 10, Some(false)),
                    _ => (false, 10, None),
                }
            } else {
                (false, 10, None)
            };

            if has_prefix {
                let number = if n_prefixes == 1 { &token[2..] } else { &token[4..] };
                match NativeInt::from_str_radix(number, radix) {
                    Ok(n) => match is_exact {
                        Some(true) | None => Ok(SExpr::from(n)),
                        Some(false) => Ok(SExpr::from(n as NativeFloat)),
                    },
                    _ => match NativeBigInt::from_str_radix(number, radix) {
                        Ok(n) => match is_exact {
                            Some(true) | None => Ok(SExpr::from(n)),
                            Some(false) => Ok(SExpr::from(n.to_float().unwrap())),
                        },
                        _ => match NativeRational::from_str_radix(number, radix) {
                            Ok(q) => match is_exact {
                                Some(true) | None => Ok(SExpr::from(q)),
                                Some(false) => Ok(SExpr::from(q.to_float().unwrap())),
                            },
                            _ => match NativeFloat::from_str_radix(number, radix) {
                                Ok(f) => match is_exact {
                                    Some(true) => {
                                        Ok(SExpr::from(NativeRational::from_float(f).unwrap()))
                                    }
                                    Some(false) | None => Ok(SExpr::from(f)),
                                },
                                _ => Ok(SExpr::Symbol(token.to_string())),
                            },
                        },
                    },
                }
            } else {
                match token.parse::<NativeInt>() {
                    Ok(n) => Ok(SExpr::from(n)),
                    _ => match token.parse::<NativeBigInt>() {
                        Ok(n) => Ok(SExpr::from(n)),
                        _ => match token.parse::<NativeRational>() {
                            Ok(q) => Ok(SExpr::from(q)),
                            _ => match token.parse::<NativeFloat>() {
                                Ok(f) => Ok(SExpr::from(f)),
                                _ => match token.parse::<NativeComplex>() {
                                    Ok(c) => Ok(SExpr::from(c)),
                                    _ => match COMPLEX_POLAR_REGEX.captures(token) {
                                        Some(_) => Ok(parse_polar_complex(token)),
                                        None => Ok(SExpr::Symbol(token.to_string())),
                                    },
                                },
                            },
                        },
                    },
                }
            }
        }
    }
}

fn parse_polar_complex(token: &str) -> SExpr {
    let parts: Vec<NativeFloat> = token
        .split('@')
        .into_iter()
        .map(|x| match x.parse::<NativeRational>() {
            Ok(n) => n.to_float().unwrap(),
            _ => x.parse::<NativeFloat>().unwrap(),
        })
        .collect();

    let magnitude = parts[0];
    let angle = parts[1];

    SExpr::from(NativeComplex::from_polar(magnitude, angle))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read() {
        let mut line = "(+ 1 2)".to_string();
        let res = super::read(&mut line);
        assert!(res.is_ok());
    }

    #[test]
    fn test_read_unbalanced_parentheses() {
        let mut line = "(+ 1 2".to_string();
        let res = super::read(&mut line);
        assert!(res.is_err());
    }
}
