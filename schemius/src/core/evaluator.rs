use std::sync::{Arc, Mutex};

use super::{
    builtins::{Primitive, SpecialForm},
    environment::Environment,
    s_expression::*,
};

pub type EvalOutput = Result<SExpr, String>;

pub struct Evaluator {
    root_environment: Arc<Mutex<Environment>>,
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

impl Evaluator {
    pub fn new() -> Self {
        Self { root_environment: Arc::new(Mutex::new(Environment::default())) }
    }

    pub fn eval(&self, expression: &SExpr) -> EvalOutput {
        eval(expression, self.root_environment.clone())
    }
}

pub fn eval(arg: &SExpr, env: ProcedureEnv) -> EvalOutput {
    let mut to_be_evaluated = arg.clone();
    let mut environment = env.clone();

    loop {
        match to_be_evaluated {
            SExpr::Symbol(ref val) => match environment.lock().unwrap().get(val) {
                Some(v) => return Ok(v),
                None => return Err(format!("Exception: in eval: could not find a value bound to <{}>", val)),
            },
            SExpr::List(list) => {
                if list.borrow().len() > 0 {
                    match eval(&list.borrow()[0], environment.clone()) {
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
                                            let result = special_form(args.to_vec(), environment.clone());
                                            match result {
                                                Ok(expression) => return Ok(expression),
                                                Err(e) => return Err(e),
                                            }
                                        } else {
                                            let result = special_form(args.to_vec(), environment.clone());
                                            match result {
                                                Ok(expression) => {
                                                    to_be_evaluated = expression;
                                                    continue;
                                                }
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
                                            match primitive(args.to_vec(), environment.clone()) {
                                                Ok(res) => return Ok(res),
                                                Err(e) => return Err(e),
                                            }
                                        } else if primitive == Primitive::APPLY {
                                            let result = Primitive::APPLY(args.to_vec(), environment.clone());
                                            match result {
                                                Ok(expr) => {
                                                    to_be_evaluated = expr;
                                                    continue;
                                                }
                                                Err(e) => return Err(e),
                                            }
                                        } else {
                                            let mut expanded_args = vec![];

                                            for arg in args.iter() {
                                                match eval(arg, environment.clone()) {
                                                    Ok(res) => expanded_args.push(res),
                                                    Err(e) => return Err(e),
                                                }
                                            }

                                            return primitive(expanded_args, environment.clone());
                                        }
                                    }
                                    Procedure::Compound(ref arg_names, ref body, ref closure_env) => {
                                        if arg_names.len() != args.len() {
                                            return Err(String::from("Exception: found different lengths for arguments and their names"));
                                        }

                                        let mut expanded_args = vec![];

                                        for arg in args.iter() {
                                            match eval(arg, environment.clone()) {
                                                Ok(res) => expanded_args.push(res),
                                                Err(e) => return Err(e),
                                            }
                                        }

                                        let lambda_env = Environment::new_child(closure_env.clone());

                                        for (name, arg) in arg_names.iter().zip(expanded_args.iter()) {
                                            match eval(arg, environment.clone()) {
                                                Ok(val) => {
                                                    if lambda_env.lock().unwrap().define(name.clone(), val).is_err() {
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

                                        to_be_evaluated = SExpr::List(SList::new(new));
                                        environment = eval_env.clone();
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
