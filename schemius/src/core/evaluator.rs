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
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self { root_environment: EnvAccessor::new(Environment::default()) }
    }

    pub fn eval(&self, expression: &SExpr) -> EvalOutput {
        eval(expression, self.root_environment.clone())
    }
}

pub fn eval(expression: &SExpr, env: ProcedureEnv) -> EvalOutput {
    let mut current_expression = expression.clone();
    let mut current_env = env.clone();

    loop {
        match current_expression {
            SExpr::Symbol(ref val) => return match current_env.borrow().get(val) {
                Some(v) => Ok(v),
                None => Err(format!("Exception: in eval: could not find a value bound to <{}>", val)),
            },
            SExpr::List(list) => {
                if list.borrow().len() > 0 {
                    match eval(&list.borrow()[0], current_env.clone()) {
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
                                            let result = special_form(args.to_vec(), current_env.clone());
                                            return match result {
                                                Ok(expression) => Ok(expression),
                                                Err(e) => Err(e),
                                            }
                                        } else {
                                            let result = special_form(args.to_vec(), current_env.clone());
                                            match result {
                                                Ok(expression) => current_expression = expression,
                                                Err(e) => return Err(e),
                                            }
                                        }
                                    }
                                    Procedure::Primitive(primitive) => {
                                        if primitive == Primitive::CONS
                                            || primitive == Primitive::DISPLAY
                                            || primitive == Primitive::CAR
                                            || primitive == Primitive::CDR
                                            || primitive == Primitive::SET_CAR
                                            || primitive == Primitive::FLATTEN
                                        {
                                            return match primitive(args.to_vec(), current_env.clone()) {
                                                Ok(res) => Ok(res),
                                                Err(e) => Err(e),
                                            }
                                        } else if primitive == Primitive::APPLY {
                                            let result = Primitive::APPLY(args.to_vec(), current_env.clone());
                                            match result {
                                                Ok(expr) => {
                                                    current_expression = expr;
                                                    continue;
                                                }
                                                Err(e) => return Err(e),
                                            }
                                        } else {
                                            let mut expanded_args = vec![];

                                            for arg in args.iter() {
                                                match eval(arg, current_env.clone()) {
                                                    Ok(res) => expanded_args.push(res),
                                                    Err(e) => return Err(e),
                                                }
                                            }

                                            return primitive(expanded_args, current_env.clone());
                                        }
                                    }
                                    Procedure::Compound(ref arg_names, ref body, ref closure_env) => {
                                        if arg_names.len() != args.len() {
                                            return Err(String::from("Exception: found different lengths for arguments and their names"));
                                        }

                                        let mut expanded_args = vec![];

                                        for arg in args.iter() {
                                            match eval(arg, current_env.clone()) {
                                                Ok(res) => expanded_args.push(res),
                                                Err(e) => return Err(e),
                                            }
                                        }

                                        let lambda_env = Environment::new_child(closure_env.clone());

                                        for (name, arg) in arg_names.iter().zip(expanded_args.iter()) {
                                            match eval(arg, current_env.clone()) {
                                                Ok(val) => {
                                                    if lambda_env.borrow_mut().define(&name, &val).is_err() {
                                                        return Err(String::from("Exception: could not bind value to the procedure frame"));
                                                    }
                                                }
                                                Err(e) => return Err(e),
                                            }
                                        }

                                        let eval_env = Environment::new_child(lambda_env);

                                        let mut new = vec![];
                                        new.push(SExpr::Procedure(Procedure::SpecialForm(SpecialForm::BEGIN)));
                                        body.iter().for_each(|x| new.push(x.clone()));

                                        current_expression = SExpr::List(SList::new(new));
                                        current_env = eval_env.clone();
                                        continue;
                                    }
                                };
                            }
                            non_proc => return Err(format!("Exception in eval: #<{}> is not a procedure", non_proc)),
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
        let evaluator = Evaluator::new();
        let expression = SExpr::Number(SNumber::Int(0));
        let res = evaluator.eval(&expression);

        assert!(res.is_ok())
    }
}
