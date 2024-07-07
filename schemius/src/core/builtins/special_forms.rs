use std::time::Instant;

use super::{
    eval, r_eval,
    s_list::SList,
    s_procedure::{Procedure, ProcedureArgs, ProcedureEnv, ProcedureOutput, SpecialFormOutput},
    Accessor, Environment, SExpr, SchemeEnvironment, SchemeList,
};

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

pub fn r_lambda(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length < 2 {
        return Err(format!(
            "Exception in lambda: expected at least 2 arguments, found {}",
            length
        ));
    }

    let arg_names = match args.s_car().unwrap() {
        SExpr::List(ref list) => match list_args(&list.access()) {
            Ok(names) => names,
            Err(e) => return Err(e),
        },
        _ => return Err(String::from("")),
    };

    let body = args[1..].to_vec();
    Ok(SExpr::Procedure(Procedure::Compound(arg_names, body, env.clone())))
}

pub fn r_define(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    // TODO: Improve this mess!
    match args.s_len() {
        1 => Ok(SExpr::Ok),
        2.. => match args.s_car().unwrap() {
            SExpr::Symbol(name) => match eval(args.s_cadr().unwrap(), env.clone()) {
                Ok(val) => {
                    let value = match val {
                        SExpr::List(list) => SExpr::List(list.clone()),
                        other => other,
                    };

                    match env.access_mut().define(name, &value) {
                        Ok(_) => Ok(SExpr::Ok),
                        Err(_) => Err(format!("Exception: error defining {}", name)),
                    }
                }
                Err(e) => Err(e),
            },
            SExpr::List(list) => {
                if list.access().s_len() == 0 {
                    return Err(String::from("Exception (TODO?): deal with empty lists"));
                }

                let lambda_name = list.access().s_car().unwrap().to_string();
                let mut lambda_args: Vec<SExpr> = vec![];
                let lambda_body = &mut args.s_cdr().unwrap();

                if list.access().s_len() > 1 {
                    for arg in &list.access()[1..] {
                        lambda_args.push(arg.clone());
                    }
                }

                lambda_args = vec![SExpr::List(SchemeList::new(lambda_args))];
                lambda_args.append(lambda_body);

                let lambda_proc = match r_lambda(lambda_args, env.clone()) {
                    Ok(lambda) => lambda,
                    Err(e) => return Err(e),
                };

                match env.access_mut().define(&lambda_name, &lambda_proc) {
                    Ok(_) => Ok(SExpr::Ok),
                    Err(_) => Err(String::from("")),
                }
            }
            _ => Err(String::from(
                "Exception: #<procedure define> cant take only a symbol and a list",
            )),
        },
        _ => Err(String::from("Exception: #<procedure define> needs arguments")),
    }
}

pub fn r_set(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length != 2 {
        return Err(format!("Exception in set!: expected 2 arguments, found {}", length));
    }

    match args.s_car().unwrap() {
        SExpr::Symbol(name) => match eval(args.s_cadr().unwrap(), env.clone()) {
            Ok(val) => {
                let value = match val {
                    SExpr::List(list) => SExpr::List(list.clone()),
                    other => other,
                };

                match env.access_mut().set(&name, &value) {
                    Ok(_) => Ok(SExpr::Ok),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e),
        },
        other => Err(format!("Exception: {} is not a symbol", other)),
    }
}

pub fn r_let(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length < 2 {
        return Err(format!(
            "Exception in let: expected at least 2 arguments, found {}",
            args.s_len()
        ));
    }

    let let_env = Environment::new_child(env.clone());

    match args.s_car().unwrap() {
        SExpr::List(list) => {
            for binding in list.access().iter() {
                match binding {
                    SExpr::List(binding) => {
                        let borrowed_binding = binding.access();
                        match borrowed_binding.s_car().unwrap() {
                            SExpr::Symbol(symbol) => {
                                match eval(&borrowed_binding[1], env.clone()) {
                                    Ok(expr) => {
                                        let_env.access_mut().define(&symbol, &expr).unwrap()
                                    }
                                    Err(e) => return Err(e),
                                }
                            }
                            other => {
                                return Err(format!("Exception in let: {} is not a symbol", other))
                            }
                        }
                    }
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

pub fn r_let_star(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length < 2 {
        return Err(format!("Exception in let: expected at least 2 arguments, found {}", length));
    }

    let mut inner_env = env;

    match args.s_car().unwrap() {
        SExpr::List(list) => {
            for binding in list.access().iter() {
                match binding {
                    SExpr::List(binding) => {
                        let borrowed_binding = binding.access();
                        match &borrowed_binding[0] {
                            SExpr::Symbol(symbol) => {
                                match eval(&borrowed_binding[1], inner_env.clone()) {
                                    Ok(expr) => {
                                        inner_env = Environment::new_child(inner_env.clone());
                                        inner_env = Environment::new_child(inner_env.clone());
                                        inner_env.access_mut().define(&symbol, &expr).unwrap();
                                    }
                                    Err(e) => return Err(e),
                                }
                            }
                            other => {
                                return Err(format!("Exception in let: {} is not a symbol", other))
                            }
                        }
                    }
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

pub fn r_if(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length != 2 && length != 3 {
        return Err(format!("Exception in if: expected two or three arguments, found {}", length));
    }

    match eval(args.s_car().unwrap(), env.clone()) {
        Ok(condition) => match condition {
            SExpr::Boolean(false) => match length {
                2 => Ok(SExpr::Ok),
                3 => Ok(args.s_ref(2).unwrap().clone()),
                _ => Err(String::from("Exception: wrong number of arguments for if")),
            },
            _ => Ok(args.s_cadr().unwrap().clone()),
        },
        Err(e) => Err(e),
    }
}

pub fn r_not(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in not: expected one argument, found {}", length));
    }

    match eval(args.s_car().unwrap(), env.clone()) {
        Ok(test) => match test {
            SExpr::Boolean(result) => Ok(SExpr::Boolean(!result)),
            _ => Ok(SExpr::Boolean(false)),
        },
        Err(e) => Err(e),
    }
}

pub fn r_begin(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.is_empty() {
        return Err(format!(
            "Exception in begin: expected at least 1 argument, found {}",
            args.s_len()
        ));
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

pub fn r_quote(args: ProcedureArgs, _: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in ': expected 1 argument, found {}", length));
    }

    Ok(args.s_car().unwrap().clone())
}

pub fn r_quasiquote(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in `: expected 1 argument, found {}", length));
    }

    match args.s_car().unwrap() {
        SExpr::List(_) | SExpr::Pair(_) => {
            let flattened = args.s_car().unwrap().flatten();

            match flattened {
                Ok(expr) => match expr {
                    SExpr::List(list) => {
                        let s_list = SExpr::List(list.clone());
                        let quasiquotes = s_list.find_symbol("quasiquote");
                        let mapping = s_list.matching_brackets().unwrap();

                        let unquotes = s_list.find_symbol("unquote");
                        let unquotes_splicing = s_list.find_symbol("unquote-splicing");
                        let mut unquotes = if let Some(unquotes) = unquotes {
                            unquotes.iter().map(|x| (false, *x)).collect()
                        } else {
                            vec![]
                        };
                        let mut unquotes_splicing = if unquotes_splicing.is_some() {
                            unquotes_splicing.unwrap().iter().map(|x| (true, *x)).collect()
                        } else {
                            vec![]
                        };

                        unquotes.append(&mut unquotes_splicing);
                        unquotes.sort_by(|x, y| x.1.cmp(&y.1));

                        if quasiquotes.is_some() {
                            let quasiquotes = quasiquotes.unwrap();

                            unquotes.retain(|(_, index)| {
                                !mapping.iter().any(|(left, right, level)| {
                                    index > left
                                        && index < right
                                        && quasiquotes.iter().any(|quasi| *left == quasi + 1)
                                        && *level > 0
                                })
                            });
                        }

                        // After each and every unquoting indexes will be shifted by a certain offset
                        let mut offset: i32 = 0;
                        let mut borrowed_list = list.access_mut();

                        loop {
                            if unquotes.is_empty() {
                                break;
                            }

                            let paren_map = SExpr::List(list.clone()).matching_brackets();
                            let unquote_is_splicing = unquotes[0].0;

                            let apply_offset = |source: i32, offset: i32| match offset {
                                0.. => (source - offset) as usize,
                                _ => (source + offset) as usize,
                            };
                            let unquote_index = apply_offset(unquotes[0].1 as i32, offset);

                            let enclosing = match paren_map {
                                Some(ref paren_map) => {
                                    if !paren_map
                                        .iter()
                                        .enumerate()
                                        .any(|(_, (i, _, _))| *i == (unquote_index + 1))
                                    {
                                        None
                                    } else {
                                        paren_map.iter().find_map(|(opening, closing, _)| {
                                            Some((*opening, *closing))
                                        })
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
                                    let raw_expr =
                                        borrowed_list[(lparen_idx + 1)..rparen_idx].to_vec();

                                    // The expression... Must be a non-self-evaluating one!
                                    if raw_expr.s_len() == 1 {
                                        let suspect = raw_expr.first().unwrap();
                                        let mut incriminated = false;

                                        if let SExpr::Symbol(symbol) = suspect {
                                            if !env
                                                .access()
                                                .get(&symbol)
                                                .unwrap()
                                                .is_procedure()
                                                .unwrap()
                                            {
                                                incriminated = true;
                                            }
                                        } else {
                                            incriminated = true;
                                        }

                                        if incriminated {
                                            return Err(format!(
                                                "Exception: {} is not a procedure",
                                                raw_expr[0]
                                            ));
                                        }
                                    }

                                    let expr = SExpr::List(SchemeList::new(raw_expr));
                                    to_be_evaluated = expr.unflatten().unwrap();
                                    first_idx = lparen_idx - 2; // Index of the left parenthesis preceding the unquote symbol
                                    last_idx = rparen_idx + 2; // Index of the right matching parenthesis + 1
                                }
                                // Unquoting symbol or atom
                                None => {
                                    to_be_evaluated = list.access()[unquote_index + 1].clone();
                                    first_idx = unquote_index - 1; // Index of the left parenthesis preceding the unquote symbol
                                    last_idx = unquote_index + 3; // Index of the right parenthesis + 1
                                }
                            };

                            offset += (last_idx - first_idx - 1) as i32;
                            let evaluated: Result<SExpr, String> =
                                eval(&to_be_evaluated, env.clone());

                            if !unquote_is_splicing {
                                borrowed_list.splice(first_idx..last_idx, evaluated);
                            } else {
                                match evaluated {
                                    Ok(ref res) => match res {
                                        SExpr::List(internal) => {
                                            let borrowed_internal = internal.access();
                                            offset -= (borrowed_internal.s_len() - 1) as i32;

                                            for i in (first_idx..last_idx).rev() {
                                                borrowed_list.remove(i);
                                            }

                                            for i in (0..internal.access().s_len()).rev() {
                                                borrowed_list.splice(
                                                    first_idx..first_idx,
                                                    [borrowed_internal[i].clone()],
                                                );
                                            }
                                        }
                                        other => {
                                            return Err(format!(
                                                "Exception: ,@ followed by non-list {} -> {}",
                                                to_be_evaluated, other
                                            ))
                                        }
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

pub fn r_cond(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    if args.is_empty() {
        return Err(format!(
            "Exception in cond: expected at least 1 argument, found {}",
            args.s_len()
        ));
    }

    let length = args.s_len();
    let have_else_clause = length > 3
        && match args.s_ref(length - 2).unwrap() {
            SExpr::Symbol(clause) => *clause == "else",
            _ => false,
        };

    let iterator = if have_else_clause { &args[0..length - 2] } else { &args };

    for block in iterator {
        match block {
            SExpr::List(list) => {
                if list.access().s_len() != 2 {
                    return Err(String::from(
                        "Exception: malformed args provided to #<procedure cond>",
                    ));
                }
                let first = eval(&list.access()[0], env.clone());
                match first {
                    Ok(condition) => match condition {
                        SExpr::Boolean(val) => match val {
                            true => return Ok(list.access()[1].clone()),
                            false => continue,
                        },
                        _ => {
                            return Err(String::from(
                                "Exception: malformed condition provided to #<procedure cond>",
                            ))
                        }
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

pub fn r_time(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    let then = Instant::now();

    match r_eval(args, env.clone()) {
        Ok(_) => {
            let elapsed = then.elapsed();
            Ok(SExpr::Symbol(format!("{:?}", elapsed)))
        }
        Err(e) => Err(e),
    }
}

pub fn r_and(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    let n_args = args.s_len();
    if n_args == 0 {
        return Ok(SExpr::Boolean(true));
    }

    for arg in args.iter().take(n_args - 1) {
        let evaluated = eval(arg, env.clone());
        if evaluated.is_err() {
            return evaluated;
        } else {
            let result = evaluated.unwrap();
            if let SExpr::Boolean(false) = result {
                return Ok(result);
            }
        }
    }

    Ok(args.last().unwrap().clone())
}

pub fn r_or(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    let n_args = args.s_len();
    if n_args == 0 {
        return Ok(SExpr::Boolean(true));
    }

    for arg in args.iter().take(n_args - 1) {
        let evaluated = eval(arg, env.clone());
        if evaluated.is_err() {
            return evaluated;
        } else {
            let result = evaluated.unwrap();
            if let SExpr::Boolean(false) = result {
                continue;
            } else {
                return Ok(arg.clone());
            }
        }
    }

    Ok(args.last().unwrap().clone())
}
