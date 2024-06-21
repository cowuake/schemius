use super::{
    s_list::SList,
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    Accessor, SExpr, SchemeEnvironment, SchemeList,
};

pub fn r_apply(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.s_len() != 2 {
        return Err(format!("Exception in apply: expected 2 arguments, found {}", args.s_len()));
    }

    let proc = &args[0];
    let args = &args[1];
    let mut to_be_evaluated = vec![];
    to_be_evaluated.push(proc.clone());

    match args {
        SExpr::List(list) => list.borrow().iter().for_each(|arg| to_be_evaluated.push(arg.clone())),
        other => return Err(format!("Exception in #<apply>: {} is not a list", other)),
    }

    Ok(SExpr::List(SchemeList::new(to_be_evaluated)))
}

pub fn r_eval(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.s_len() != 1 {
        return Err(format!("Exception in eval: expected 1 argument, found {}", args.s_len()));
    }

    Ok(args[0].clone())
}

pub fn r_display(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
    if args.s_len() != 1 {
        return Err(format!("Exception in display: expected 1 argument, found {}", args.s_len()));
    }

    match &args[0] {
        SExpr::String(string) => Ok(SExpr::Symbol(string.borrow().to_string())), // Avoids double quotes
        expr => Ok(SExpr::Symbol(format!("{}", expr))),
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
