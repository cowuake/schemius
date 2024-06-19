use super::{
    accessor::Accessor,
    builtins::{Primitive, SpecialForm},
    environment::{EnvAccessor, Environment, SchemeEnvironment},
    s_expression::*,
};

pub type EvalOutput = Result<SExpr, String>;

pub struct Evaluator {
    root_environment: EnvAccessor<Environment>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Evaluator {
    pub fn new(environment: Option<Environment>) -> Self {
        let env = match environment {
            Some(env) => EnvAccessor::new(env),
            None => EnvAccessor::new(Environment::default()),
        };
        Self { root_environment: env }
    }

    pub fn eval(&self, expression: &SExpr) -> EvalOutput {
        eval(expression, self.root_environment.clone())
    }
}

fn expand_args(args: &Vec<SExpr>, env: ProcedureEnv) -> EvalOutput {
    let mut expanded_args = vec![];

    for arg in args.iter() {
        match eval(arg, env.clone()) {
            Ok(val) => expanded_args.push(val),
            Err(e) => return Err(e),
        }
    }

    Ok(SExpr::List(SchemeList::new(expanded_args)))
}

pub fn eval(expression: &SExpr, env: ProcedureEnv) -> EvalOutput {
    let mut current_expression = expression.clone();
    let mut current_env = env.clone();

    loop {
        match current_expression {
            SExpr::Symbol(ref val) => {
                return match current_env.borrow().get(val) {
                    Some(v) => Ok(v),
                    None => Err(format!(
                        "Exception: in eval: could not find a value bound to <{}>",
                        val
                    )),
                }
            }
            SExpr::List(list) => {
                if list.borrow().s_len() > 0 {
                    let first = eval(&list.borrow()[0], current_env.clone());
                    match first {
                        Ok(res) => match res {
                            SExpr::Procedure(proc) => {
                                let args = &list.borrow()[1..].to_vec();

                                match proc {
                                    Procedure::SpecialForm(special_form) => {
                                        if special_form == SpecialForm::QUOTE
                                            || special_form == SpecialForm::QUASIQUOTE
                                            || special_form == SpecialForm::DEFINE
                                            || special_form == SpecialForm::SET
                                            || special_form == SpecialForm::LET
                                            || special_form == SpecialForm::LET_STAR
                                            || special_form == SpecialForm::TIME
                                        {
                                            let result =
                                                special_form(args.to_vec(), current_env.clone());
                                            return match result {
                                                Ok(expression) => Ok(expression),
                                                Err(e) => Err(e),
                                            };
                                        } else {
                                            let result =
                                                special_form(args.to_vec(), current_env.clone());
                                            match result {
                                                Ok(expression) => current_expression = expression,
                                                Err(e) => return Err(e),
                                            }
                                        }
                                    }
                                    Procedure::Primitive(primitive) => {
                                        match expand_args(&args, current_env.clone()) {
                                            Ok(result) => match result {
                                                SExpr::List(list) => {
                                                    if primitive != Primitive::EVAL
                                                        && primitive != Primitive::APPLY
                                                    {
                                                        return primitive(
                                                            list.borrow().to_vec(),
                                                            current_env.clone(),
                                                        );
                                                    }
                                                    current_expression = primitive(
                                                        list.borrow().to_vec(),
                                                        current_env.clone(),
                                                    )?;
                                                }
                                                _ => {
                                                    return Err(String::from(
                                                        "Exception: could not expand arguments",
                                                    ))
                                                }
                                            },
                                            Err(e) => return Err(e),
                                        }
                                    }
                                    Procedure::Compound(
                                        ref arg_names,
                                        ref body,
                                        ref closure_env,
                                    ) => {
                                        if arg_names.s_len() != args.s_len() {
                                            return Err(String::from("Exception: found different lengths for arguments and their names"));
                                        }

                                        let mut expanded_args = vec![];

                                        for arg in args.iter() {
                                            match arg {
                                                SExpr::List(list)
                                                    if list
                                                        .borrow()
                                                        .s_car()
                                                        .unwrap()
                                                        .is_quote()
                                                        .unwrap() =>
                                                {
                                                    expanded_args.push(arg.clone())
                                                }
                                                _ => match eval(arg, current_env.clone()) {
                                                    Ok(res) => expanded_args.push(res),
                                                    Err(e) => return Err(e),
                                                },
                                            }
                                        }

                                        let lambda_env =
                                            Environment::new_child(closure_env.clone());

                                        for (name, arg) in
                                            arg_names.iter().zip(expanded_args.iter())
                                        {
                                            match eval(arg, current_env.clone()) {
                                                Ok(val) => {
                                                    if lambda_env
                                                        .borrow_mut()
                                                        .define(&name, &val)
                                                        .is_err()
                                                    {
                                                        return Err(String::from("Exception: could not bind value to the procedure frame"));
                                                    }
                                                }
                                                Err(e) => return Err(e),
                                            }
                                        }

                                        let eval_env = Environment::new_child(lambda_env);

                                        let mut new = vec![];
                                        new.push(SExpr::Procedure(Procedure::SpecialForm(
                                            SpecialForm::BEGIN,
                                        )));
                                        body.iter().for_each(|x| new.push(x.clone()));

                                        current_expression = SExpr::List(SchemeList::new(new));
                                        current_env = eval_env.clone();
                                        continue;
                                    }
                                };
                            }
                            non_proc => {
                                return Err(format!(
                                    "Exception in eval: #<{}> is not a procedure",
                                    non_proc
                                ))
                            }
                        },
                        Err(e) => return Err(e),
                    };
                } else {
                    return Err(format!("Exception: wrong syntax {}", SExpr::List(list)));
                }
            }
            other => return Ok(other),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::s_expression::{s_number::SNumber, SExpr};

    use super::Evaluator;

    #[test]
    fn evaluator_ok_int() {
        let evaluator = Evaluator::default();
        let expression = SExpr::Number(SNumber::Int(0));
        let res = evaluator.eval(&expression);

        assert!(res.is_ok())
    }
}
