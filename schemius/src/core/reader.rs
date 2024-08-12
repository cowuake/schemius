use lazy_static::lazy_static;
use num::Num;
use regex::Regex;

use super::{accessor::Accessor, s_expression::*};

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
        match c {
            '(' | '[' => balance += 1,
            ')' | ']' => balance -= 1,
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
        "(" | "[" | "#(" => {
            let mut new_list = VectorImplementation::new();

            loop {
                let token: String = init(line);

                if (opening_token == "(" && token == ")") || (opening_token == "[" && token == "]")
                {
                    if new_list.len() == 3 && token != "]" {
                        if let SExpr::Symbol(sym) = &new_list.s_ref(1).unwrap() {
                            if sym.as_str() == "." {
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
                } else if opening_token == "#(" && token == ")" {
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
        "#t" => Ok(SExpr::Boolean(true)),
        "#f" => Ok(SExpr::Boolean(false)),
        "-nan.0" | "+nan.0" => Ok(SExpr::Number(SNumber::Float(NativeFloat::NAN))),
        "-inf.0" => Ok(SExpr::Number(SNumber::Float(NativeFloat::NEG_INFINITY))),
        "+inf.0" => Ok(SExpr::Number(SNumber::Float(NativeFloat::INFINITY))),
        token if token.starts_with('"') => {
            Ok(SExpr::String(SchemeString::new(token.get(1..token.len() - 1).unwrap().to_string())))
        }
        "'" | "`" | "," | ",@" => {
            let internal_token = init(line);
            let quoted = advance(line, &internal_token)?;
            let mut vec = ListImplementation::new();

            let string_token = match token {
                "'" => "quote".to_string(),
                "`" => "quasiquote".to_string(),
                "," => "unquote".to_string(),
                ",@" => "unquote-splicing".to_string(),
                _ => token.to_string(),
            };

            vec.push(SExpr::Symbol(string_token));
            vec.push(quoted);
            Ok(SExpr::List(SchemeList::new(vec)))
        }
        token if token.starts_with(r#"#\"#) => {
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
                && token.chars().nth(0) == Some('#')
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
                    ("#b", "#e") | ("#e", "#b") => (true, 2, Some(true)),
                    ("#b", "#i") | ("#i", "#b") => (true, 2, Some(false)),
                    ("#o", "#e") | ("#e", "#o") => (true, 8, Some(true)),
                    ("#o", "#i") | ("#i", "#o") => (true, 8, Some(false)),
                    ("#d", "#e") | ("#e", "#d") => (true, 10, Some(true)),
                    ("#d", "#i") | ("#i", "#d") => (true, 10, Some(false)),
                    ("#x", "#e") | ("#e", "#x") => (true, 16, Some(true)),
                    ("#x", "#i") | ("#i", "#x") => (true, 16, Some(false)),
                    _ => (false, 10, None),
                }
            } else if n_prefixes == 1 {
                match &token[0..2] {
                    "#b" => (true, 2, None),
                    "#o" => (true, 8, None),
                    "#d" => (true, 10, None),
                    "#x" => (true, 16, None),
                    "#e" => (true, 10, Some(true)),
                    "#i" => (true, 10, Some(false)),
                    _ => (false, 10, None),
                }
            } else {
                (false, 10, None)
            };

            if has_prefix {
                let number = if n_prefixes == 1 { &token[2..] } else { &token[4..] };
                match NativeInt::from_str_radix(number, radix) {
                    Ok(n) => match is_exact {
                        Some(true) | None => Ok(SExpr::Number(SNumber::Int(n))),
                        Some(false) => Ok(SExpr::Number(SNumber::Float(n as NativeFloat))),
                    },
                    _ => match NativeBigInt::from_str_radix(number, radix) {
                        Ok(n) => match is_exact {
                            Some(true) | None => Ok(SExpr::Number(SNumber::BigInt(n))),
                            Some(false) => Ok(SExpr::Number(SNumber::Float(n.to_float().unwrap()))),
                        },
                        _ => match NativeRational::from_str_radix(number, radix) {
                            Ok(q) => match is_exact {
                                Some(true) | None => Ok(SExpr::Number(SNumber::Rational(q))),
                                Some(false) => {
                                    Ok(SExpr::Number(SNumber::Float(q.to_float().unwrap())))
                                }
                            },
                            _ => match NativeFloat::from_str_radix(number, radix) {
                                Ok(f) => match is_exact {
                                    Some(true) => Ok(SExpr::Number(SNumber::Rational(
                                        NativeRational::from_float(f).unwrap(),
                                    ))),
                                    Some(false) | None => Ok(SExpr::Number(SNumber::Float(f))),
                                },
                                _ => Ok(SExpr::Symbol(token.to_string())),
                            },
                        },
                    },
                }
            } else {
                match token.parse::<NativeInt>() {
                    Ok(n) => Ok(SExpr::Number(SNumber::Int(n))),
                    _ => match token.parse::<NativeBigInt>() {
                        Ok(n) => Ok(SExpr::Number(SNumber::BigInt(n))),
                        _ => match token.parse::<NativeRational>() {
                            Ok(q) => Ok(SExpr::Number(SNumber::Rational(q))),
                            _ => match token.parse::<NativeFloat>() {
                                Ok(f) => Ok(SExpr::Number(SNumber::Float(f))),
                                _ => match token.parse::<NativeComplex>() {
                                    Ok(c) => Ok(SExpr::Number(SNumber::Complex(c))),
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

    SExpr::Number(SNumber::Complex(NativeComplex::from_polar(magnitude, angle)))
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
