use super::{
    s_list::SList,
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    Accessor, SExpr, SchemeList, SchemePair,
};

pub fn r_set_car(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.s_len() != 2 {
        return Err(format!("Exception in set-car!: expected 2 arguments, found {}", args.s_len()));
    }

    match args.s_car().unwrap() {
        SExpr::List(list) => {
            list.borrow_mut()[0] = args[1].clone();

            Ok(SExpr::Unspecified)
        }
        SExpr::Pair(pair) => {
            let old_cdr = pair.borrow().1.clone();
            pair.replace((Box::new(args[1].clone()), old_cdr));

            Ok(SExpr::Unspecified)
        }
        other => Err(format!("Exception in set-car: {} is neither a list nor a pair", other)),
    }
}

pub fn r_cons(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.s_len() != 2 {
        return Err(format!("Exception in cons: expected 2 arguments, found {}", args.s_len()));
    }

    let car = args.s_car().unwrap().clone();

    match &args[1] {
        SExpr::List(list) => {
            let mut new_list = vec![];
            new_list.push(car);
            list.borrow().iter().for_each(|x| new_list.push(x.clone()));

            Ok(SExpr::List(SchemeList::new(new_list)))
        }
        cdr => {
            let pair = SchemePair::new((Box::new(car), Box::new(cdr.clone())));

            Ok(SExpr::Pair(pair))
        }
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

    match &args.s_car().unwrap() {
        SExpr::Pair(pair) => {
            let car = pair.borrow().0.clone();
            Ok(*car)
        }
        SExpr::List(list) => {
            let borrowed = list.borrow();
            if borrowed.s_len() > 0 {
                let car = if borrowed.s_car().unwrap().is_quote().unwrap() {
                    borrowed
                        .s_cdr()
                        .map(|x| x.clone())
                        .collect::<Vec<SExpr>>()
                        .s_car()
                        .unwrap()
                        .clone()
                } else {
                    borrowed.s_car().unwrap().clone()
                };
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
                    let cdr: Vec<SExpr> = list.s_cdr().map(|x| x.clone()).collect();
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
