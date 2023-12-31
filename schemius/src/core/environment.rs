use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::{
    builtins::*,
    s_expression::{s_procedure::*, NumericalConstant, SExpr},
};

#[derive(Debug)]
pub struct Environment {
    parent: Option<Arc<Mutex<Environment>>>,
    table: HashMap<String, SExpr>,
}

impl Environment {
    pub fn new_child(parent: Arc<Mutex<Environment>>) -> Arc<Mutex<Environment>> {
        let env = Environment { parent: Some(parent), table: HashMap::new() };
        Arc::new(Mutex::new(env))
    }

    pub fn define(&mut self, key: String, value: SExpr) -> Result<(), ()> {
        self.table.insert(key, value);

        Ok(())
    }

    pub fn set(&mut self, key: String, value: SExpr) -> Result<(), String> {
        if self.table.contains_key(&key) {
            self.table.insert(key, value);

            Ok(())
        } else {
            match self.parent {
                Some(ref parent) => parent.lock().unwrap().set(key, value),
                None => Err(format!("Exception: {} is not bound", key)),
            }
        }
    }

    pub fn get(&self, key: &String) -> Option<SExpr> {
        match self.table.get(key) {
            Some(val) => Some(val.clone()),
            None => match self.parent {
                Some(ref parent) => parent.lock().unwrap().get(key),
                None => None,
            },
        }
    }

    pub fn get_bindings(&self) -> Vec<(&String, &SExpr)> {
        let symbols: Vec<(&String, &SExpr)> = self.table.iter().map(|e| (e.0, e.1)).collect();

        symbols
    }

    pub fn get_root(env: ProcedureEnv) -> ProcedureEnv {
        match env.lock().unwrap().parent {
            Some(ref frame) => Environment::get_root(frame.clone()),
            None => env.clone(),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        let table: HashMap<String, SExpr> = HashMap::from([
            (String::from("π"), SExpr::Number(NumericalConstant::PI)),
            (String::from("pi"), SExpr::Number(NumericalConstant::PI)),
            (String::from("avogadro"), SExpr::Number(NumericalConstant::AVOGADRO)),
            (String::from("boltzmann"), SExpr::Number(NumericalConstant::BOLTZMANN)),
            (String::from("e"), SExpr::Number(NumericalConstant::EULER)),
            (String::from("euler"), SExpr::Number(NumericalConstant::EULER)),
            (String::from("golden-ratio"), SExpr::Number(NumericalConstant::GOLDEN_RATIO)),
            (String::from("gravitational-constant"), SExpr::Number(NumericalConstant::GRAVITATIONAL_CONSTANT)),
            (String::from("h"), SExpr::Number(NumericalConstant::PLANCK)),
            (String::from("planck"), SExpr::Number(NumericalConstant::PLANCK)),
            (String::from("exit"), SExpr::Procedure(Procedure::Primitive(Primitive::EXIT))),
            (String::from("+"), SExpr::Procedure(Procedure::Primitive(Primitive::SUM))),
            (String::from("-"), SExpr::Procedure(Procedure::Primitive(Primitive::DIFF))),
            (String::from("*"), SExpr::Procedure(Procedure::Primitive(Primitive::PROD))),
            (String::from("/"), SExpr::Procedure(Procedure::Primitive(Primitive::QUOT))),
            (String::from("="), SExpr::Procedure(Procedure::Primitive(Primitive::EQUAL))),
            (String::from(">"), SExpr::Procedure(Procedure::Primitive(Primitive::GT))),
            (String::from(">="), SExpr::Procedure(Procedure::Primitive(Primitive::GE))),
            (String::from("<"), SExpr::Procedure(Procedure::Primitive(Primitive::LT))),
            (String::from("<="), SExpr::Procedure(Procedure::Primitive(Primitive::LE))),
            (String::from("'"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::QUOTE))),
            (String::from("quote"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::QUOTE))),
            (String::from("`"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::QUASIQUOTE))),
            (String::from("quasiquote"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::QUASIQUOTE))),
            (String::from("λ"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::LAMBDA))),
            (String::from("lambda"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::LAMBDA))),
            (String::from("let"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::LET))),
            (String::from("let*"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::LET_STAR))),
            (String::from("eval"), SExpr::Procedure(Procedure::Primitive(Primitive::EVAL))),
            (String::from("apply"), SExpr::Procedure(Procedure::Primitive(Primitive::APPLY))),
            (String::from("car"), SExpr::Procedure(Procedure::Primitive(Primitive::CAR))),
            (String::from("cdr"), SExpr::Procedure(Procedure::Primitive(Primitive::CDR))),
            (String::from("define"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::DEFINE))),
            (String::from("set!"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::SET))),
            (String::from("cons"), SExpr::Procedure(Procedure::Primitive(Primitive::CONS))),
            (String::from("list"), SExpr::Procedure(Procedure::Primitive(Primitive::LIST))),
            (String::from("set-car!"), SExpr::Procedure(Procedure::Primitive(Primitive::SET_CAR))),
            (String::from("begin"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::BEGIN))),
            (String::from("if"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::IF))),
            (String::from("not"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::NOT))),
            (String::from("cond"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::COND))),
            (String::from("display"), SExpr::Procedure(Procedure::Primitive(Primitive::DISPLAY))),
            (String::from("char?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_CHAR))),
            (String::from("string?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_STRING))),
            (String::from("boolean?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_BOOLEAN))),
            (String::from("number?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_NUMBER))),
            (String::from("exact?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_EXACT))),
            (String::from("list?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_LIST))),
            (String::from("pair?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_PAIR))),
            (String::from("vector?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_VECTOR))),
            (String::from("procedure?"), SExpr::Procedure(Procedure::Primitive(Primitive::IS_PROCEDURE))),
            (String::from("time"), SExpr::Procedure(Procedure::SpecialForm(SpecialForm::TIME))),
            (String::from("environment-bindings"), SExpr::Procedure(Procedure::Primitive(Primitive::ENVIRONMENT_BINDINGS))),
            (String::from("string-set!"), SExpr::Procedure(Procedure::Primitive(Primitive::STRING_SET))),
            (String::from("flatten"), SExpr::Procedure(Procedure::Primitive(Primitive::FLATTEN))),
            (String::from("unflatten"), SExpr::Procedure(Procedure::Primitive(Primitive::UNFLATTEN))),
        ]);

        Self { parent: None, table }
    }
}
