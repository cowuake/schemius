use std::{any, borrow::BorrowMut, time::Instant};

use super::{environment::Environment, evaluator::eval, s_expression::*};

pub struct Primitive;
pub struct SpecialForm;

impl Primitive {
    pub const EXIT: ProcedureSignature = r_exit;
    pub const SUM: ProcedureSignature = r_sum;
    pub const DIFF: ProcedureSignature = r_diff;
    pub const PROD: ProcedureSignature = r_prod;
    pub const QUOT: ProcedureSignature = r_quot;
    pub const EQUAL: ProcedureSignature = r_equal;
    pub const GT: ProcedureSignature = r_gt;
    pub const GE: ProcedureSignature = r_ge;
    pub const LT: ProcedureSignature = r_lt;
    pub const LE: ProcedureSignature = r_le;
    pub const EVAL: ProcedureSignature = r_eval;
    pub const APPLY: ProcedureSignature = r_apply;
    pub const CAR: ProcedureSignature = r_car;
    pub const CDR: ProcedureSignature = r_cdr;
    pub const CONS: ProcedureSignature = r_cons;
    pub const LIST: ProcedureSignature = r_list;
    pub const SET_CAR: ProcedureSignature = r_set_car;
    pub const DISPLAY: ProcedureSignature = r_display;
    pub const IS_CHAR: ProcedureSignature = r_is_char;
    pub const IS_STRING: ProcedureSignature = r_is_string;
    pub const IS_BOOLEAN: ProcedureSignature = r_is_boolean;
    pub const IS_NUMBER: ProcedureSignature = r_is_number;
    pub const IS_EXACT: ProcedureSignature = r_is_exact;
    pub const IS_PAIR: ProcedureSignature = r_is_pair;
    pub const IS_LIST: ProcedureSignature = r_is_list;
    pub const IS_VECTOR: ProcedureSignature = r_is_vector;
    pub const IS_PROCEDURE: ProcedureSignature = r_is_procedure;
    pub const ENVIRONMENT_BINDINGS: ProcedureSignature = r_environment_bindings;
    pub const STRING_SET: ProcedureSignature = r_string_set;
    pub const FLATTEN: ProcedureSignature = r_flatten;
    pub const UNFLATTEN: ProcedureSignature = r_unflatten;
}

impl SpecialForm {
    pub const BEGIN: SpecialFormSignature = r_begin;
    pub const COND: SpecialFormSignature = r_cond;
    pub const DEFINE: SpecialFormSignature = r_define;
    pub const IF: SpecialFormSignature = r_if;
    pub const LAMBDA: SpecialFormSignature = r_lambda;
    pub const LET: SpecialFormSignature = r_let;
    pub const LET_STAR: SpecialFormSignature = r_let_star;
    pub const NOT: SpecialFormSignature = r_not;
    pub const QUOTE: SpecialFormSignature = r_quote;
    pub const QUASIQUOTE: SpecialFormSignature = r_quasiquote;
    pub const SET: SpecialFormSignature = r_set;
    pub const TIME: SpecialFormSignature = r_time;
}

fn list_args(list: &[SExpr]) -> Result<Vec<String>, String> {
    let mut args: Vec<String> = vec![];

    for item in list[0..].iter() {
        match item {
            SExpr::Symbol(val) => args.push(val.clone()),
            _ => return Err(String::from("Exception: found non-symbol object in list")),
        }
    }

    Ok(args)
}

fn r_lambda(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.len() < 2 {
        return Err(format!("Exception in lambda: expected at least 2 arguments, found {}", args.len()));
    }

    let arg_names = match args[0] {
        SExpr::List(ref list) => match list_args(&list.borrow()) {
            Ok(names) => names,
            Err(e) => return Err(e),
        },
        _ => return Err(String::from("")),
    };

    let body = (args[1..]).to_vec();
    Ok(SExpr::Procedure(Procedure::Compound(arg_names, body, env.clone())))
}

fn r_define(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    // TODO: Improve this mess!
    match args.len() {
        1 => Ok(SExpr::Ok),
        2.. => match &args[0] {
            SExpr::Symbol(name) => match eval(&args[1], env.clone()) {
                Ok(val) => {
                    let value = match val {
                        SExpr::List(list) => SExpr::List(list.clone()),
                        other => other,
                    };

                    match env.lock().unwrap().define(name.clone(), value) {
                        Ok(_) => Ok(SExpr::Ok),
                        Err(()) => Err(format!("Exception: error defining {}", name)),
                    }
                }
                Err(e) => Err(e),
            },
            SExpr::List(list) => match list.borrow().len() {
                1.. => {
                    let lambda_name = &list.borrow()[0].to_string();
                    let mut lambda_args: Vec<SExpr> = vec![];
                    let lambda_body = &mut args[1..].to_vec();

                    if list.borrow().len() > 1 {
                        for arg in (&list.borrow()[1..]).iter() {
                            lambda_args.push(arg.clone());
                        }
                    }

                    lambda_args = vec![SExpr::List(SList::new(lambda_args))];
                    lambda_args.append(lambda_body);

                    let lambda_proc = match r_lambda(lambda_args, env.clone()) {
                        Ok(lambda) => lambda,
                        Err(e) => return Err(e),
                    };

                    match env.lock().unwrap().define(lambda_name.clone(), lambda_proc) {
                        Ok(_) => Ok(SExpr::Ok),
                        Err(_) => Err(String::from("")),
                    }
                }
                _ => Err(String::from("Exception: TODO")),
            },
            _ => Err(String::from("Exception: #<procedure define> cant take only a symbol and a list")),
        },
        _ => Err(String::from("Exception: #<procedure define> needs arguments")),
    }
}

fn r_set(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.len() != 2 {
        return Err(format!("Exception in set!: expected 2 arguments, found {}", args.len()));
    }

    match &args[0] {
        SExpr::Symbol(name) => match eval(&args[1], env.clone()) {
            Ok(val) => {
                let value = match val {
                    SExpr::List(list) => SExpr::List(list.clone()),
                    other => other,
                };

                match env.lock().unwrap().set(name.clone(), value) {
                    Ok(_) => Ok(SExpr::Ok),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        },
        other => Err(format!("Exception: {} is not a symbol", other)),
    }
}

fn r_let(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.len() < 2 {
        return Err(format!("Exception in let: expected at least 2 arguments, found {}", args.len()));
    }

    let let_env = Environment::new_child(env.clone());

    match &args[0] {
        SExpr::List(list) => {
            for binding in list.borrow().iter() {
                match binding {
                    SExpr::List(binding) => match &binding.borrow()[0] {
                        SExpr::Symbol(symbol) => match eval(&binding.borrow()[1], env.clone()) {
                            Ok(expr) => let_env.lock().unwrap().define(symbol.clone(), expr).unwrap(),
                            Err(e) => return Err(e),
                        },
                        other => return Err(format!("Exception in let: {} is not a symbol", other)),
                    },
                    other => return Err(format!("Exception in let: {} is not a list", other)),
                }
            }
        }
        other => return Err(format!("Exception in let: {} is not a list", other)),
    }

    let mut res = SExpr::Unspecified;

    for body_item in &args[1..] {
        match eval(body_item, let_env.clone()) {
            Ok(something) => res = something,
            Err(e) => return Err(e),
        }
    }

    Ok(res)
}

fn r_let_star(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.len() < 2 {
        return Err(format!("Exception in let: expected at least 2 arguments, found {}", args.len()));
    }

    let mut inner_env = env;

    match &args[0] {
        SExpr::List(list) => {
            for binding in list.borrow().iter() {
                match binding {
                    SExpr::List(binding) => match &binding.borrow()[0] {
                        SExpr::Symbol(symbol) => match eval(&binding.borrow()[1], inner_env.clone()) {
                            Ok(expr) => {
                                inner_env = Environment::new_child(inner_env.clone());
                                inner_env = Environment::new_child(inner_env.clone());
                                inner_env.lock().unwrap().define(symbol.clone(), expr).unwrap();
                            }
                            Err(e) => return Err(e),
                        },
                        other => return Err(format!("Exception in let: {} is not a symbol", other)),
                    },
                    other => return Err(format!("Exception in let: {} is not a list", other)),
                }
            }
        }
        other => return Err(format!("Exception in let: {} is not a list", other)),
    }

    let mut res = SExpr::Unspecified;

    for body_item in &args[1..] {
        match eval(body_item, inner_env.clone()) {
            Ok(something) => res = something,
            Err(e) => return Err(e),
        }
    }

    Ok(res)
}

fn r_set_car(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 2 {
        return Err(format!("Exception in set-car!: expected 2 arguments, found {}", args.len()));
    }

    match &args[0] {
        SExpr::Symbol(_) => match eval(&args[0], env.clone()) {
            Ok(res) => match res {
                SExpr::List(list) => {
                    list.borrow_mut()[0] = args[1].clone();

                    Ok(SExpr::List(list.clone()))
                }
                SExpr::Pair(pair) => {
                    let old_cdr = pair.borrow().1.clone();
                    pair.replace((Box::new(args[1].clone()), old_cdr));

                    Ok(SExpr::Pair(pair.clone()))
                }
                other => Err(format!("Exception in set-car: {} is neither a list nor a pair", other)),
            },
            Err(e) => Err(e),
        },
        _ => Err(String::from("Exception in set-car!: must provide a symbol as the first argument")),
    }
}

fn r_if(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.len() != 2 && args.len() != 3 {
        return Err(format!("Exception in if: expected two or three arguments, found {}", args.len()));
    }

    match eval(&args[0], env.clone()) {
        Ok(condition) => match condition {
            SExpr::Boolean(false) => match args.len() {
                2 => Ok(SExpr::Ok),
                3 => Ok(args[2].clone()),
                _ => Err(String::from("Exception: wrong number of arguments for if")),
            },
            _ => Ok(args[1].clone()),
        },
        Err(e) => Err(e),
    }
}

fn r_not(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.len() != 1 {
        return Err(format!("Exception in not: expected one argument, found {}", args.len()));
    }

    match eval(&args[0], env.clone()) {
        Ok(test) => match test {
            SExpr::Boolean(result) => Ok(SExpr::Boolean(!result)),
            _ => Ok(SExpr::Boolean(false)),
        },
        Err(e) => Err(e),
    }
}

fn r_display(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(format!("Exception in display: expected 1 argument, found {}", args.len()));
    }

    match eval(&args[0], env.clone()) {
        Ok(val) => match val {
            SExpr::String(string) => Ok(SExpr::Symbol(string.borrow().to_string())), // Avoids double quotes
            expr => Ok(SExpr::Symbol(format!("{}", expr))),
        },
        Err(e) => Err(e),
    }
}

fn r_cond(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.is_empty() {
        return Err(format!("Exception in cond: expected at least 1 argument, found {}", args.len()));
    }

    let have_else_clause = args.len() > 3
        && match &args[args.len() - 2] {
            SExpr::Symbol(clause) => {
                if *clause == String::from("else") {
                    true
                } else {
                    false
                }
            }
            _ => false,
        };

    let iterator = if have_else_clause { &args[0..args.len() - 2] } else { &args };

    for block in iterator {
        match block {
            SExpr::List(list) => {
                if list.borrow().len() != 2 {
                    return Err(String::from("Exception: malformed args provided to #<procedure cond>"));
                }
                match eval(&list.borrow()[0], env.clone()) {
                    Ok(condition) => match condition {
                        SExpr::Boolean(val) => match val {
                            true => return Ok(list.borrow()[1].clone()),
                            false => continue,
                        },
                        _ => return Err(String::from("Exception: malformed condition provided to #<procedure cond>")),
                    },
                    Err(e) => return Err(e),
                }
            }
            _ => return Err(String::from("Exception: #<procedure cond> args must be lists")),
        }
    }

    if have_else_clause {
        Ok(args.last().unwrap().clone())
    } else {
        Ok(SExpr::Ok)
    }
}

fn r_exit(_: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    std::process::exit(0)
}

fn r_apply(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 2 {
        return Err(format!("Exception in apply: expected 2 arguments, found {}", args.len()));
    }

    let symbol = &args[0];
    let arg_list = &args[1];

    match eval(arg_list, env.clone()) {
        Ok(list) => match list {
            SExpr::List(args) => {
                let iterator = [symbol.clone()];
                let mut args = args.borrow().clone();
                args.splice(0..0, iterator);

                Ok(SExpr::List(SList::new(args.clone())))
            }
            _ => Err(String::from("Exception in apply: must provide a quoted list of arguments")),
        },
        Err(e) => Err(e),
    }
}

fn r_eval(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(format!("Exception in eval: expected 1 argument, found {}", args.len()));
    }

    eval(&args[0], env.clone())
}

fn r_cons(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 2 {
        return Err(format!("Exception in cons: expected 2 arguments, found {}", args.len()));
    }

    let car = eval(&args[0], env.clone());
    if let Err(e) = car {
        return Err(e);
    }

    match eval(&args[1], env.clone()) {
        Ok(expr) => match expr {
            SExpr::List(list) => {
                let mut new_list = vec![];
                new_list.push(car.unwrap());
                list.borrow().iter().for_each(|x| new_list.push(x.clone()));

                Ok(SExpr::List(SList::new(new_list)))
            }
            cdr => {
                let pair = SPair::new((Box::new(car.unwrap()), Box::new(cdr)));

                Ok(SExpr::Pair(pair))
            }
        },
        Err(e) => Err(e),
    }
}

fn r_list(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let mut list: Vec<SExpr> = vec![];

    for arg in args {
        list.push(arg.clone());
    }

    Ok(SExpr::List(SList::new(list)))
}

fn r_begin(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.is_empty() {
        return Err(format!("Exception in begin: expected at least 1 argument, found {}", args.len()));
    }

    let splitted = args.split_last().unwrap();

    for v in splitted.1.iter() {
        match eval(v, env.clone()) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
    }

    Ok(splitted.0.clone())
}

fn r_flatten(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(format!("Exception in flatten: expected 1 argument, found {}", args.len()));
    }

    args[0].flatten()
}

fn r_unflatten(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(format!("Exception in unflatten: expected 1 argument, found {}", args.len()));
    }

    args[0].unflatten()
}

fn r_quote(args: ProcedureArgs, _: ProcedureEnv) -> SpecialFormOutput {
    if args.len() != 1 {
        return Err(format!("Exception in ': expected 1 argument, found {}", args.len()));
    }

    Ok(args[0].clone())
}

fn r_quasiquote(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.len() != 1 {
        return Err(format!("Exception in `: expected 1 argument, found {}", args.len()));
    }

    match &args[0] {
        SExpr::List(_) | SExpr::Pair(_) => {
            let flattened = args[0].flatten();

            match flattened {
                Ok(expr) => match expr {
                    SExpr::List(list) => {
                        let quasiquotes = SExpr::List(list.clone()).find_symbol("quasiquote");
                        let mapping = SExpr::List(list.clone()).matching_brackets().unwrap();

                        let unquotes = SExpr::List(list.clone()).find_symbol("unquote");
                        let unquotes_splicing = SExpr::List(list.clone()).find_symbol("unquote-splicing");
                        let mut unquotes = if unquotes.is_some() { unquotes.unwrap().iter().map(|x| (false, *x)).collect() } else { vec![] };
                        let mut unquotes_splicing =
                            if unquotes_splicing.is_some() { unquotes_splicing.unwrap().iter().map(|x| (true, *x)).collect() } else { vec![] };

                        unquotes.append(&mut unquotes_splicing);
                        unquotes.sort_by(|x, y| x.1.cmp(&y.1));

                        if quasiquotes.is_some() {
                            let quasiquotes = quasiquotes.unwrap();

                            unquotes.retain(|(_, index)| {
                                !mapping.iter().any(|(left, right, level)| {
                                    index > left && index < right && quasiquotes.iter().any(|quasi| *left == quasi + 1) && *level > 0
                                })
                            });
                        }

                        // After each and every unquoting indexes will be shifted by a certain offset
                        let mut offset: i32 = 0;

                        loop {
                            if unquotes.is_empty() {
                                break;
                            }

                            let paren_map = SExpr::List(list.clone()).matching_brackets();
                            let unquote_is_splicing = unquotes[0].0;
                            let unquote_index;

                            let apply_offset = |source: i32, offset: i32| match offset {
                                0.. => (source - offset) as usize,
                                _ => (source + offset) as usize,
                            };
                            unquote_index = apply_offset(unquotes[0].1 as i32, offset);

                            let enclosing = match paren_map {
                                Some(ref paren_map) => {
                                    if !paren_map.iter().enumerate().any(|(_, (i, _, _))| *i == (unquote_index + 1)) {
                                        None
                                    } else {
                                        paren_map.iter().find_map(|(opening, closing, _)| Some((*opening, *closing)))
                                    }
                                }
                                None => None,
                            };

                            let to_be_evaluated;
                            let first_idx;
                            let last_idx;

                            match enclosing {
                                // Unquoting expression (list)
                                Some((lparen_idx, rparen_idx)) => {
                                    // The final expression does not need enclosing parentheses
                                    let raw_expr = list.borrow()[(lparen_idx + 1)..rparen_idx].to_vec();

                                    // The expression... Must be a non-self-evaluating one!
                                    if raw_expr.len() == 1 {
                                        let suspect = raw_expr.first().unwrap();
                                        let mut incriminated = false;

                                        if let SExpr::Symbol(symbol) = suspect {
                                            if !env.lock().unwrap().get(&symbol).unwrap().is_procedure().unwrap() {
                                                incriminated = true;
                                            }
                                        } else {
                                            incriminated = true;
                                        }

                                        if incriminated {
                                            return Err(format!("Exception: {} is not a procedure", raw_expr[0]));
                                        }
                                    }

                                    let expr = SExpr::List(SList::new(raw_expr));
                                    to_be_evaluated = expr.unflatten().unwrap();
                                    first_idx = lparen_idx - 2; // Index of the left parenthesis preceding the unquote symbol
                                    last_idx = rparen_idx + 2; // Index of the right matching parenthesis + 1
                                }
                                // Unquoting symbol or atom
                                None => {
                                    to_be_evaluated = list.borrow()[unquote_index + 1].clone();
                                    first_idx = unquote_index - 1; // Index of the left parenthesis preceding the unquote symbol
                                    last_idx = unquote_index + 3; // Index of the right parenthesis + 1
                                }
                            };

                            offset += (last_idx - first_idx - 1) as i32;
                            let evaluated: Result<SExpr, String> = eval(&to_be_evaluated, env.clone());

                            if !unquote_is_splicing {
                                list.borrow_mut().splice(first_idx..last_idx, evaluated);
                            } else {
                                match evaluated {
                                    Ok(ref res) => match res {
                                        SExpr::List(internal) => {
                                            offset -= (internal.borrow().len() - 1) as i32;

                                            for i in (first_idx..last_idx).into_iter().rev() {
                                                list.borrow_mut().remove(i);
                                            }

                                            for i in (0..(internal.borrow().len())).into_iter().rev() {
                                                list.borrow_mut().splice(first_idx..first_idx, [internal.borrow()[i].clone()]);
                                            }
                                        }
                                        other => return Err(format!("Exception: ,@ followed by non-list {} -> {}", to_be_evaluated, other)),
                                    },
                                    Err(e) => return Err(e),
                                }
                            }

                            unquotes.remove(0);
                        }

                        Ok(SExpr::List(list.clone()).unflatten().unwrap())
                    }
                    other => Ok(other.clone()),
                },
                Err(e) => Err(e),
            }
        }
        other => Ok(other.clone()),
    }
}

fn r_car(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(format!("Exception in car: expected 1 argument, found {}", args.len()));
    }

    match eval(&args[0], env.clone())? {
        SExpr::List(vec) => {
            if vec.borrow().len() > 0 {
                Ok(vec.borrow()[0].clone())
            } else {
                Err(String::from("Exception: #<procedure car> cannot take a quoted empty list"))
            }
        }
        _ => Err(String::from("Exception: #<procedure car> cannot be applied to quoted symbol")),
    }
}

fn r_cdr(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(String::from("Exception: #<special-form cdr> can take one only argument"));
    }

    match eval(&args[0], env.clone())? {
        SExpr::List(vec) => match vec.borrow().len() {
            1.. => {
                let mut cdr = vec.borrow().clone();
                cdr.remove(0);

                Ok(SExpr::List(SList::new(cdr)))
            }
            _ => Err(String::from("Exception: #<procedure cdr> cannot take a quoted empty list")),
        },
        _ => Err(String::from("Exception: #<procedure cdr> cannot be applied to quoted symbol")),
    }
}

macro_rules! fn_compute_sum_prod {
    ($($fn:ident: $op:tt, $neutral:literal)*) => {
    $(
        fn $fn(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
            match args.len() {
                0 => Ok(SExpr::Number(SNumber::Int($neutral))),
                1.. => {
                    let mut res = SNumber::Int($neutral);

                    for arg in &args[0..] {
                        match arg {
                            SExpr::Number(n) => res = &res $op &n,
                            num => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), num)),
                        }
                    }

                    Ok(SExpr::Number(res))
                },
                _ => Err(String::from("???")),
            }
        }
    )*}
}

macro_rules! fn_compute_diff_quot {
    ($($fn:ident: $op:tt, $neutral:literal)*) => {
    $(
        fn $fn(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
            match args.len() {
                0 => Err(String::from("Exception: too few arguments")),
                1.. => {
                    let mut res = match &args[0] {
                            SExpr::Number(num) => num.clone(),
                            num => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), num)),
                    };

                    if args.len() > 1 {
                        for arg in &args[1..] {
                            match arg {
                                SExpr::Number(n) => res = &res $op &n,
                                num => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), num)),
                            }
                        }
                    } else {
                        res = SNumber::Int($neutral) $op res;
                    }

                    Ok(SExpr::Number(res))
                }
                _ => Err(String::from("???")),
            }
        }
    )*}
}

fn_compute_sum_prod! {
    r_sum: +, 0
    r_prod: *, 1
}

fn_compute_diff_quot! {
    r_diff: -, 0
    r_quot: /, 1
}

macro_rules! fn_compare {
    ($($fn:ident: $op:tt)*) => {
        $(
            fn $fn(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
                if args.len() < 2 {
                    return Err(format!("Exception in {}: expected at least 2 arguments, found {}", stringify!($op), args.len()));
                }

                let mut result = true;

                for (lhs, rhs) in args.iter().zip(args.iter().skip(1)) {
                    match lhs {
                        SExpr::Number(left) => {
                            match rhs {
                                SExpr::Number(right) => {
                                    if !(left $op right) {
                                        result = false;
                                        break;
                                    }
                                },
                                other => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), other)),
                            }
                        },
                        other => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), other)),
                    }
                }

                Ok(SExpr::Boolean(result))
            }
        )*}
}

fn_compare! {
    r_equal: ==
    r_gt: >
    r_ge: >=
    r_lt: <
    r_le: <=
}

macro_rules! fn_is {
    ($($fn:ident, $source_fn:ident, $name:literal)*) => {
        $(
            fn $fn(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
                if args.len() != 1 {
                    return Err(format!("Exception in {}: expected 1 argument, found {}", $name, args.len()));
                }

                match args[0].$source_fn() {
                    Ok(res) => Ok(SExpr::Boolean(res)),
                    Err(e) => Err(e)
                }
            }
        )*}
}

fn_is! {
    r_is_char, is_char, "char?"
    r_is_string, is_string, "string?"
    r_is_boolean, is_boolean, "boolean?"
    r_is_number, is_number, "number?"
    r_is_exact, is_exact, "exact?"
    r_is_list, is_list, "list?"
    r_is_pair, is_pair, "pair?"
    r_is_vector, is_vector, "vector?"
    r_is_procedure, is_procedure, "procedure?"
    // r_is_symbol, is_symbol, "symbol?"
}

fn r_string_set(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 3 {
        return Err(format!("Exception in string-set!: expected 3 arguments, found {}", args.len()));
    }

    match &args[0] {
        SExpr::String(string) => match &args[1] {
            SExpr::Number(index) => {
                let index = index.to_int().unwrap() as usize;
                let is_in_range = index <= string.borrow().len();

                if is_in_range {
                    match &args[2] {
                        SExpr::Char(character) => {
                            string.borrow_mut().replace_range(index..index + 1, character.to_string().as_str());

                            Ok(SExpr::String(string.clone()))
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

fn r_environment_bindings(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if !args.is_empty() {
        return Err(format!("Exception in environment-bindings: expected 0 arguments, found {}", args.len()));
    }

    let env_guard = env.lock().unwrap();
    let mut bindings = env_guard.get_bindings().clone();
    bindings.sort_by(|a, b| (a.0).cmp(b.0));

    let mut output: String = "".to_owned();
    bindings.iter().for_each(|b| output.push_str(format!("({}, {})\n", b.0, b.1).as_str()));
    output.remove(output.len() - 1);

    Ok(SExpr::Symbol(output))
}

fn r_time(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    let then = Instant::now();

    match r_eval(args, env.clone()) {
        Ok(_) => {
            let elapsed = then.elapsed();
            Ok(SExpr::Symbol(format!("{:?}", elapsed)))
        }
        Err(e) => Err(e),
    }
}
