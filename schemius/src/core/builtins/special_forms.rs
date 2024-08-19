use std::time::Instant;

use super::{
    eval, r_eval,
    s_list::SList,
    s_procedure::{Procedure, ProcedureArgs, ProcedureEnv, ProcedureOutput, SpecialFormOutput},
    Accessor, Environment, ListImplementation, SExpr, SchemeEnvironment, SchemeList,
};

fn list_args(list: &ListImplementation) -> Result<Vec<String>, String> {
    let mut args: Vec<String> = vec![];

    for item in list.iter() {
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

    let body = args.s_cdr().unwrap();
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
                let mut lambda_args = ListImplementation::new();
                let mut proc_args = ListImplementation::new();
                let lambda_body = &mut args.s_cdr().unwrap();

                if list.access().s_len() > 1 {
                    for arg in &list.access().s_cdr().unwrap() {
                        proc_args.push(arg.clone());
                    }
                }

                lambda_args =
                    ListImplementation::from_iter([SExpr::List(SchemeList::new(proc_args))]);
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
                                match eval(&borrowed_binding.s_ref(1).unwrap(), env.clone()) {
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

    for body_item in &args.s_cdr().unwrap() {
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
                        match &borrowed_binding.s_car().unwrap() {
                            SExpr::Symbol(symbol) => {
                                match eval(&borrowed_binding.s_ref(1).unwrap(), inner_env.clone()) {
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

    for body_item in &args.s_cdr().unwrap() {
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

    let tail = args.last().unwrap();
    let mut head = args.clone();
    head.pop();

    for v in head.iter() {
        match eval(v, env.clone()) {
            Ok(_) => {}
            Err(e) => return Err(e),
        };
    }

    Ok(tail.clone())
}

pub fn r_quote(args: ProcedureArgs, _: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in ': expected 1 argument, found {}", length));
    }

    Ok(args.s_car().unwrap().clone())
}

fn r_unquote(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in ,: expected 1 argument, found {}", length));
    }

    Ok(eval(args.s_car().unwrap(), env).unwrap())
}

pub fn r_quasiquote(args: ProcedureArgs, env: ProcedureEnv) -> SpecialFormOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in `: expected 1 argument, found {}", length));
    }

    let arg = args.s_car().unwrap();

    if !args.s_car().unwrap().is_list().unwrap() {
        return Err("Exception in `: expected a list".to_string());
    }

    match arg {
        SExpr::List(list) => {
            let mut new_list = ListImplementation::new();
            let current_list = list.access();

            for item in current_list.iter() {
                match item {
                    SExpr::List(inner_list) => {
                        if inner_list.access().s_len() > 0 {
                            if inner_list.access().s_car().unwrap().is_unquote()? {
                                let unquoted = r_unquote(
                                    ListImplementation::from_iter([inner_list
                                        .access()
                                        .s_cadr()
                                        .unwrap()
                                        .clone()]),
                                    env.clone(),
                                )?;

                                if inner_list.access().s_car().unwrap().is_unquote_pure()? {
                                    new_list.push(unquoted);
                                } else if inner_list
                                    .access()
                                    .s_car()
                                    .unwrap()
                                    .is_unquote_splicing()?
                                {
                                    match unquoted {
                                        SExpr::List(list) => {
                                            let borrowed_list = list.access();
                                            for item in borrowed_list.iter() {
                                                new_list.push(item.clone());
                                            }
                                        }
                                        _ => new_list.push(unquoted),
                                    }
                                }
                            } else if inner_list.access().s_car().unwrap().is_quasiquote()? {
                                new_list.push(SExpr::List(inner_list.clone()));
                            } else {
                                new_list.push(
                                    r_quasiquote(
                                        ListImplementation::from_iter([SExpr::List(
                                            inner_list.clone(),
                                        )]),
                                        env.clone(),
                                    )
                                    .unwrap(),
                                );
                            }
                        } else {
                            new_list.push(SExpr::List(inner_list.clone()));
                        }
                    }
                    _ => new_list.push(item.clone()),
                }
            }

            Ok(SExpr::List(SchemeList::new(new_list)))
        }
        _ => Ok(arg.clone()),
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

    let iterator =
        if have_else_clause { args.clone().extract_range(0, length - 2) } else { args.clone() };

    for block in iterator {
        match block {
            SExpr::List(list) => {
                if list.access().s_len() != 2 {
                    return Err(String::from(
                        "Exception: malformed args provided to #<procedure cond>",
                    ));
                }
                let first = eval(&list.access().s_car().unwrap(), env.clone());
                match first {
                    Ok(condition) => match condition {
                        SExpr::Boolean(val) => match val {
                            true => return Ok(list.access().s_ref(1).unwrap().clone()),
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

#[cfg(test)]
mod special_forms_tests {
    use crate::core::builtins::EnvAccessor;

    use super::*;

    #[test]
    fn test_special_form_unquote() {
        let env = EnvAccessor::new(Environment::new());
        let list = SExpr::List(SchemeList::new(ListImplementation::from_iter([
            SExpr::from(1),
            SExpr::from(2),
            SExpr::from(3),
        ])));
        let list_name = "l";

        let def = env.access_mut().define(list_name, &list);
        assert!(def.is_ok());

        let args = ListImplementation::from_iter([SExpr::Symbol(list_name.to_string())]);
        let res = r_unquote(args, env);

        assert!(res.is_ok());
        let res = res.unwrap().as_list().unwrap();
        let list = list.as_list().unwrap();

        assert_eq!(res.s_ref(0).unwrap().as_int(), list.s_ref(0).unwrap().as_int());
        assert_eq!(res.s_ref(1).unwrap().as_int(), list.s_ref(1).unwrap().as_int());
        assert_eq!(res.s_ref(2).unwrap().as_int(), list.s_ref(2).unwrap().as_int());
    }
}
