use super::{
    eval,
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    Accessor, SExpr, SchemeEnvironment, SchemeList,
};

pub fn r_apply(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
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

                Ok(SExpr::List(SchemeList::new(args.clone())))
            }
            _ => Err(String::from("Exception in apply: must provide a quoted list of arguments")),
        },
        Err(e) => Err(e),
    }
}

pub fn r_eval(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if args.len() != 1 {
        return Err(format!("Exception in eval: expected 1 argument, found {}", args.len()));
    }

    eval(&args[0], env.clone())
}

pub fn r_display(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
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

pub fn r_exit(_: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    std::process::exit(0)
}

pub fn r_environment_bindings(args: ProcedureArgs, env: ProcedureEnv) -> ProcedureOutput {
    if !args.is_empty() {
        return Err(format!(
            "Exception in environment-bindings: expected 0 arguments, found {}",
            args.len()
        ));
    }

    let env_guard = env.borrow();
    let mut bindings = env_guard.get_bindings().clone();
    bindings.sort_by(|a, b| a.0.cmp(b.0));

    let mut output: String = "".to_owned();
    bindings.iter().for_each(|b| output.push_str(format!("({}, {})\n", b.0, b.1).as_str()));
    output.remove(output.len() - 1);

    Ok(SExpr::Symbol(output))
}
