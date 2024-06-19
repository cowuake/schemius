use super::{
    eval,
    s_list::SList,
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    Accessor, SExpr, SchemeList, SchemePair,
};

pub fn r_set_car(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.s_len() != 2 {
        return Err(format!("Exception in set-car!: expected 2 arguments, found {}", args.s_len()));
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
    if args.s_len() != 2 {
        return Err(format!("Exception in cons: expected 2 arguments, found {}", args.s_len()));
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
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in flatten: expected 1 argument, found {}", length));
    }

    args[0].flatten()
}

pub fn r_unflatten(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in unflatten: expected 1 argument, found {}", length));
    }

    args[0].unflatten()
}

pub fn r_car(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in car: expected 1 argument, found {}", length));
    }

    match &args[0] {
        SExpr::Pair(pair) => {
            let car = pair.borrow().0.clone();
            Ok(*car)
        }
        SExpr::List(list) => {
            if list.borrow().s_len() > 0 {
                let car = list.borrow().s_car().unwrap().clone();
                Ok(car)
            } else {
                Err(String::from("Exception: #<procedure car> cannot take a quoted empty list"))
            }
        }
        _ => Err(String::from("Exception: #<procedure car> cannot be applied to quoted symbol")),
    }
}

pub fn r_cdr(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.s_len() != 1 {
        return Err(String::from("Exception: #<special-form cdr> can take one only argument"));
    }

    match &args[0] {
        SExpr::Pair(pair) => {
            let cdr = pair.borrow().1.clone();
            Ok(*cdr)
        }
        SExpr::List(list) => {
            let list = list.borrow();
            match list.s_len() {
                1.. => {
                    let cdr = list.s_cdr().map(|x| x.clone()).collect();

                    Ok(SExpr::List(SchemeList::new(cdr)))
                }
                _ => {
                    Err(String::from("Exception: #<procedure cdr> cannot take a quoted empty list"))
                }
            }
        }
        _ => Err(String::from("Exception: #<procedure cdr> cannot be applied to quoted symbol")),
    }
}

pub fn r_reverse(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in #<reverse>: expected 1 argument, found {}", length));
    }

    match &args[0] {
        SExpr::List(list) => {
            let reversed = list.borrow().s_reverse();
            Ok(SExpr::List(SchemeList::new(reversed)))
        }
        _ => Err(String::from("Exception in #<reverse>: expected a list")),
    }
}
