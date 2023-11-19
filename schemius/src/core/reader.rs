use lazy_static::lazy_static;
use regex::Regex;

use super::s_expression::*;

pub fn read(line: &mut String) -> SExpr {
    let first_token = init(line);
    advance(line, &first_token)
}

fn init(line: &mut String) -> String {
    lazy_static! {
        static ref TOKEN_REGEX: Regex = Regex::new(r#"^\s*(,@|#\\\.|[\[('`,)\]]|#\(|"(?:\.|[^"])*"|;.*|[^\s\[('"`,;)\]]*)(.*)"#).unwrap();
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

                if (opening_token == "(" && token == ")") || (opening_token == "[" && token == "]") {
                    if new_list.len() == 3 && token != "]" {
                        if let SExpr::Symbol(sym) = &new_list[1] {
                            if sym.as_str() == "." {
                                return SExpr::Pair(SPair::new((Box::new(new_list[0].clone()), Box::new(new_list[2].clone()))));
                            }
                        }
                    }

                    return SExpr::List(SList::new(new_list));
                } else if opening_token == "#(" && token == ")" {
                    return SExpr::Vector(SList::new(new_list));
                } else {
                    new_list.push(advance(line, &token));
                }
            }
        }
        _ => parse_token(line, string_token),
    }
}

fn parse_token(line: &mut String, token: &String) -> SExpr {
    if token == "#t" {
        return SExpr::Boolean(true);
    } else if token == "#f" {
        return SExpr::Boolean(false);
    } else if token.starts_with('"') {
        return SExpr::String(SString::new(token.get(1..token.len() - 1).unwrap().to_string()));
    } else if token == "'" || token == "`" || token == "," || token == ",@" {
        let internal_token = init(line);
        let quoted = advance(line, &internal_token);
        let mut vec: Vec<SExpr> = vec![];

        let string_token = {
            if token == "'" {
                "quote".to_string()
            } else if token == "`" {
                "quasiquote".to_string()
            } else if token == "," {
                "unquote".to_string()
            } else if token == ",@" {
                "unquote-splicing".to_string()
            } else {
                token.to_string()
            }
        };

        vec.push(SExpr::Symbol(string_token));
        vec.push(quoted);
        return SExpr::List(SList::new(vec));
    } else if token.starts_with(r#"#\"#) {
        if token.len() == 3 {
            return SExpr::Char(token.chars().last().unwrap());
        }
    } else {
        if let Ok(n) = token.parse::<NativeInt>() {
            return SExpr::Number(SNumber::Int(n));
        }
        if let Ok(n) = token.parse::<NativeBigInt>() {
            return SExpr::Number(SNumber::BigInt(n));
        }
        if let Ok(n) = token.parse::<NativeRational>() {
            return SExpr::Number(SNumber::Rational(n));
        }
        if let Ok(f) = token.parse::<NativeFloat>() {
            return SExpr::Number(SNumber::Float(f));
        }
    }

    SExpr::Symbol(token.clone())
}
