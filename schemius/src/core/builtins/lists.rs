use super::{
    eval,
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    Accessor, SExpr, SchemeList, SchemePair,
};

pub fn r_set_car(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
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
                other => {
                    Err(format!("Exception in set-car: {} is neither a list nor a pair", other))
                }
            },
            Err(e) => Err(e),
        },
        _ => {
            Err(String::from("Exception in set-car!: must provide a symbol as the first argument"))
        }
    }
}

pub fn r_cons(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
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

                Ok(SExpr::List(SchemeList::new(new_list)))
            }
            cdr => {
                let pair = SchemePair::new((Box::new(car.unwrap()), Box::new(cdr)));

                Ok(SExpr::Pair(pair))
            }
        },
        Err(e) => Err(e),
    }
}

pub fn r_list(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let mut list: Vec<SExpr> = vec![];

    for arg in args {
        list.push(arg.clone());
    }

    Ok(SExpr::List(SchemeList::new(list)))
}

pub fn r_flatten(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(format!("Exception in flatten: expected 1 argument, found {}", args.len()));
    }

    args[0].flatten()
}

pub fn r_unflatten(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(format!("Exception in unflatten: expected 1 argument, found {}", args.len()));
    }

    args[0].unflatten()
}

pub fn r_car(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
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

pub fn r_cdr(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(String::from("Exception: #<special-form cdr> can take one only argument"));
    }

    match eval(&args[0], env.clone())? {
        SExpr::List(vec) => match vec.borrow().len() {
            1.. => {
                let mut cdr = vec.borrow().clone();
                cdr.remove(0);

                Ok(SExpr::List(SchemeList::new(cdr)))
            }
            _ => Err(String::from("Exception: #<procedure cdr> cannot take a quoted empty list")),
        },
        _ => Err(String::from("Exception: #<procedure cdr> cannot be applied to quoted symbol")),
    }
}
