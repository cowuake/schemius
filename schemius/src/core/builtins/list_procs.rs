use super::{
    s_list::SList,
    s_number::{NativeInt, SNumber},
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    Accessor, ListImplementation, SExpr, SchemeList, SchemePair,
};

pub fn r_set_car(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.s_len() != 2 {
        return Err(format!("Exception in set-car!: expected 2 arguments, found {}", args.s_len()));
    }

    match args.s_car().unwrap() {
        SExpr::List(list) => {
            list.borrow_mut().set_car(args.s_cadr().unwrap().clone());
            Ok(SExpr::Unspecified)
        }
        SExpr::Pair(pair) => {
            let old_cdr = pair.borrow().1.clone();
            pair.replace((Box::new(args.s_cadr().unwrap().clone()), old_cdr));

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

    match args.s_cadr().unwrap() {
        SExpr::List(list) => {
            let mut new_list = vec![];
            new_list.push(car);
            list.borrow_mut().iter().for_each(|x| new_list.push(x.clone()));

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

    args.s_car().unwrap().flatten()
}

pub fn r_unflatten(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in unflatten: expected 1 argument, found {}", length));
    }

    args.s_car().unwrap().unflatten()
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
                    borrowed.s_cdr().unwrap().s_car().unwrap().clone()
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

    match args.s_car().unwrap() {
        SExpr::Pair(pair) => {
            let cdr = pair.borrow().1.clone();
            Ok(*cdr)
        }
        SExpr::List(list) => {
            let list = list.borrow();

            match list.s_len() {
                1.. => {
                    let cdr: Vec<SExpr> = list.s_cdr().unwrap();
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

pub fn r_append(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length < 1 {
        return Err(format!(
            "Exception in #<append>: expected at least 1 arguments, found {}",
            length
        ));
    }

    if length == 1 {
        return Ok(args.s_car().unwrap().clone());
    }

    for arg in args.iter() {
        if !arg.is_list().unwrap() {
            return Err(format!("Exception in #<append>: expected a list, found {}", arg));
        }
    }

    let new_list = ListImplementation::s_append(
        args.iter().map(|x| x.as_list().unwrap()).collect::<Vec<_>>().as_slice(),
    );
    Ok(SExpr::List(SchemeList::new(new_list)))
}

pub fn r_list_ref(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 2 {
        return Err(format!("Exception in #<list-ref>: expected 2 arguments, found {}", length));
    }

    match args.s_car().unwrap() {
        SExpr::List(list) => {
            let index = args.s_cadr().unwrap().as_int().unwrap() as usize;
            let borrowed = list.borrow();
            let len = borrowed.s_len();

            if index >= len {
                return Err(format!(
                    "Exception in #<list-ref>: index {} out of bounds for list of length {}",
                    index, len
                ));
            }

            Ok(borrowed.s_ref(index as usize).unwrap().clone())
        }
        _ => Err(String::from("Exception in #<list-ref>: expected a list")),
    }
}

pub fn r_list_tail(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 2 {
        return Err(format!("Exception in #<list-tail>: expected 2 arguments, found {}", length));
    }

    match args.s_car().unwrap() {
        SExpr::List(list) => {
            let index = args.s_cadr().unwrap().as_int()? as usize;
            let borrowed = list.borrow();
            let len = borrowed.s_len();

            if index >= len {
                return Err(format!(
                    "Exception in #<list-tail>: index {} out of bounds for list of length {}",
                    index, len
                ));
            }

            Ok(SExpr::List(SchemeList::new(borrowed.s_tail(index as usize))))
        }
        _ => Err(String::from("Exception in #<list-tail>: expected a list")),
    }
}

pub fn r_reverse(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in #<reverse>: expected 1 argument, found {}", length));
    }

    match args.s_car().unwrap() {
        SExpr::List(list) => {
            let reversed = list.borrow().s_reverse();
            Ok(SExpr::List(SchemeList::new(reversed)))
        }
        _ => Err(String::from("Exception in #<reverse>: expected a list")),
    }
}

pub fn r_length(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    let length = args.s_len();
    if length != 1 {
        return Err(format!("Exception in #<length>: expected 1 argument, found {}", length));
    }

    match args.s_car().unwrap() {
        SExpr::List(list) => {
            let len = list.borrow().s_len();
            Ok(SExpr::Number(SNumber::Int(NativeInt::from(len as NativeInt))))
        }
        _ => Err(String::from("Exception in #<length>: expected a list")),
    }
}
