use lazy_static::lazy_static;
use num::{Num, ToPrimitive};
use regex::Regex;

use super::{accessor::Accessor, s_expression::*};

pub fn read(line: &mut String) -> SExpr {
    let first_token = init(line);
    advance(line, &first_token)
}

fn init(line: &mut String) -> String {
    lazy_static! {
        static ref TOKEN_REGEX: Regex = Regex::new(
            r#"^\s*(,@|#\\\.|[\[('`,)\]]|#\(|"(?:\.|[^"])*"|;.*|[^\s\[('"`,;)\]]*)(.*)"#
        )
        .unwrap();
    }

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

fn advance(line: &mut String, string_token: &String) -> SExpr {
    let opening_token = string_token.as_str();

    match opening_token {
        "(" | "[" | "#(" => {
            let mut new_list: Vec<SExpr> = vec![];

            loop {
                let token: String = init(line);

                if (opening_token == "(" && token == ")") || (opening_token == "[" && token == "]")
                {
                    if new_list.len() == 3 && token != "]" {
                        if let SExpr::Symbol(sym) = &new_list[1] {
                            if sym.as_str() == "." {
                                return SExpr::Pair(SchemePair::new((
                                    Box::new(new_list[0].clone()),
                                    Box::new(new_list[2].clone()),
                                )));
                            }
                        }
                    }

                    return SExpr::List(SchemeList::new(new_list));
                } else if opening_token == "#(" && token == ")" {
                    return SExpr::Vector(SchemeList::new(new_list));
                } else {
                    new_list.push(advance(line, &token));
                }
            }
        }
        _ => parse_token(line, string_token),
    }
}

fn parse_token(line: &mut String, token: &str) -> SExpr {
    match token {
        "#t" => SExpr::Boolean(true),
        "#f" => SExpr::Boolean(false),
        "-nan.0" | "+nan.0" => SExpr::Number(SNumber::Float(NativeFloat::NAN)),
        "-inf.0" => SExpr::Number(SNumber::Float(NativeFloat::NEG_INFINITY)),
        "+inf.0" => SExpr::Number(SNumber::Float(NativeFloat::INFINITY)),
        token if token.starts_with('"') => {
            SExpr::String(SchemeString::new(token.get(1..token.len() - 1).unwrap().to_string()))
        }
        "'" | "`" | "," | ",@" => {
            let internal_token = init(line);
            let quoted = advance(line, &internal_token);
            let mut vec: Vec<SExpr> = vec![];

            let string_token = match token {
                "'" => "quote".to_string(),
                "`" => "quasiquote".to_string(),
                "," => "unquote".to_string(),
                ",@" => "unquote-splicing".to_string(),
                _ => token.to_string(),
            };

            vec.push(SExpr::Symbol(string_token));
            vec.push(quoted);
            SExpr::List(SchemeList::new(vec))
        }
        token if token.starts_with(r#"#\"#) => {
            match token.len() {
                3 => SExpr::Char(token.chars().last().unwrap()),
                _ => {
                    // Handle invalid character token
                    // Return an appropriate value or raise an error
                    // For now, returning SExpr::Symbol(token.clone())
                    SExpr::Symbol(token.to_string())
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
                        Some(true) | None => SExpr::Number(SNumber::Int(n)),
                        Some(false) => SExpr::Number(SNumber::Float(n as NativeFloat)),
                    },
                    _ => match NativeBigInt::from_str_radix(number, radix) {
                        Ok(n) => match is_exact {
                            Some(true) | None => SExpr::Number(SNumber::BigInt(n)),
                            Some(false) => SExpr::Number(SNumber::Float(n.to_f64().unwrap())),
                        },
                        _ => match NativeRational::from_str_radix(number, radix) {
                            Ok(q) => match is_exact {
                                Some(true) | None => SExpr::Number(SNumber::Rational(q)),
                                Some(false) => SExpr::Number(SNumber::Float(q.to_f64().unwrap())),
                            },
                            _ => match NativeFloat::from_str_radix(number, radix) {
                                Ok(f) => match is_exact {
                                    Some(true) => {
                                        if f <= NativeInt::MIN as NativeFloat
                                            || f >= NativeInt::MAX as NativeFloat
                                        {
                                            SExpr::Number(SNumber::Rational(
                                                NativeRational::from_float(f).unwrap(),
                                            ))
                                        } else {
                                            SExpr::Number(SNumber::Int(f.floor() as NativeInt))
                                        }
                                    }
                                    Some(false) | None => SExpr::Number(SNumber::Float(f)),
                                },
                                _ => SExpr::Symbol(token.to_string()),
                            },
                        },
                    },
                }
            } else {
                match token.parse::<NativeInt>() {
                    Ok(n) => SExpr::Number(SNumber::Int(n)),
                    _ => match token.parse::<NativeBigInt>() {
                        Ok(n) => SExpr::Number(SNumber::BigInt(n)),
                        _ => match token.parse::<NativeRational>() {
                            Ok(q) => SExpr::Number(SNumber::Rational(q)),
                            _ => match token.parse::<NativeFloat>() {
                                Ok(f) => SExpr::Number(SNumber::Float(f)),
                                _ => SExpr::Symbol(token.to_string()),
                            },
                        },
                    },
                }
            }
        }
    }
}
