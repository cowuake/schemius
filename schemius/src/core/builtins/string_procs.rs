use super::{
    s_list::SList,
    s_number::NativeInt,
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    Accessor, SExpr, SchemeNumber, SchemeString,
};

pub fn r_string(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.iter().any(|arg| !arg.is_char().unwrap()) {
        return Err("Exception in string: one or more arguments are not characters".to_string());
    }

    match args.s_len() {
        0 => Err(format!(
            "Exception in string: expected at least 1 argument, found {}",
            args.s_len()
        )),
        1 => Ok(SExpr::String(SchemeString::new(args[0].to_string()))),
        2.. => {
            let mut output = String::new();
            for arg in args {
                output.push(arg.as_char().unwrap());
            }
            Ok(SExpr::String(SchemeString::new(output)))
        }
    }
}

pub fn r_make_string(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 && length != 2 {
        return Err(format!(
            "Exception in make-string: expected 1 or 2 arguments, found {}",
            length
        ));
    }

    match &args[0] {
        SExpr::Number(n) => {
            let n = n.to_int().unwrap();
            let mut output = String::new();
            let character = if length == 2 {
                match &args[1] {
                    SExpr::Char(c) => *c,
                    other => {
                        return Err(format!("Exception in make-string: {} is not a char", other))
                    }
                }
            } else {
                ' '
            };

            for _ in 0..n {
                output.push(character);
            }

            Ok(SExpr::String(SchemeString::new(output)))
        }
        other => Err(format!("Exception in make-string: {} is not a number", other)),
    }
}

pub fn r_string_append(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let mut output = String::new();

    for arg in args {
        match arg {
            SExpr::String(string) => output.push_str(string.borrow().as_str()),
            other => return Err(format!("Exception in string-append: {} is not a string", other)),
        }
    }

    Ok(SExpr::String(SchemeString::new(output)))
}

pub fn r_string_ref(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 2 {
        return Err(format!("Exception in string-ref: expected 2 arguments, found {}", length));
    }

    match &args[0] {
        SExpr::String(string) => match &args[1] {
            SExpr::Number(index) => {
                let index = index.to_int().unwrap() as usize;
                let is_in_range = index < string.borrow().len();

                if is_in_range {
                    let character = string.borrow().chars().nth(index).unwrap();
                    Ok(SExpr::Char(character))
                } else {
                    Err("Exception in string-ref: index out of range".to_string())
                }
            }
            other => Err(format!("Exception in string-ref: {} is not a valid index", other)),
        },
        other => Err(format!("Exception in string-ref: {} is not a string", other)),
    }
}

pub fn r_string_set(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 3 {
        return Err(format!(
            "Exception in string-set!: expected 3 arguments, found {}",
            args.len()
        ));
    }

    match &args[0] {
        SExpr::String(string) => match &args[1] {
            SExpr::Number(index) => {
                let index = index.to_int().unwrap() as usize;
                let is_in_range = index < string.borrow().len();

                if is_in_range {
                    match &args[2] {
                        SExpr::Char(character) => {
                            let replacement = character.to_string();
                            string
                                .borrow_mut()
                                .replace_range(index..index + 1, replacement.as_str());

                            let output = string.borrow().clone();
                            Ok(SExpr::String(SchemeString::new(output)))
                        }
                        other => Err(format!("Exception in string-set!: {} is not a char", other)),
                    }
                } else {
                    Err("Exception in string-set!: index out of range".to_string())
                }
            }
            other => Err(format!("Exception in string-set!: {} is not a valid index", other)),
        },
        other => Err(format!("Exception in string-set!: {} is not a string", other)),
    }
}

pub fn r_string_upcase(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in string-upcase: expected 1 argument, found {}", length));
    }

    match &args[0] {
        SExpr::String(string) => {
            let output = string.borrow().to_uppercase();
            Ok(SExpr::String(SchemeString::new(output)))
        }
        other => Err(format!("Exception in string-upcase: {} is not a string", other)),
    }
}

pub fn r_string_downcase(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in string-downcase: expected 1 argument, found {}", length));
    }

    match &args[0] {
        SExpr::String(string) => {
            let output = string.borrow().to_lowercase();
            Ok(SExpr::String(SchemeString::new(output)))
        }
        other => Err(format!("Exception in string-downcase: {} is not a string", other)),
    }
}

pub fn r_string_length(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in string-length: expected 1 argument, found {}", length));
    }

    match &args[0] {
        SExpr::String(string) => {
            let length = string.borrow().len();
            Ok(SExpr::Number(SchemeNumber::Int(length as NativeInt)))
        }
        other => Err(format!("Exception in string-length: {} is not a string", other)),
    }
}
