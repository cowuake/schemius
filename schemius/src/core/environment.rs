use std::collections::HashMap;

use super::{
    accessor::*,
    builtins::*,
    s_expression::{s_procedure::*, NumericalConstant, SExpr},
};

pub type EnvAccessor<EnvironmentTrait> = ThreadSafeAccessor<EnvironmentTrait>;

pub trait SchemeEnvironment: Clone
where
    Self: Sized,
{
    fn new() -> Self;
    fn new_child(parent: EnvAccessor<Self>) -> EnvAccessor<Self>;
    fn define(&mut self, key: &str, value: &SExpr) -> Result<(), String>;
    fn set(&mut self, key: &str, value: &SExpr) -> Result<(), String>;
    fn get(&self, key: &str) -> Option<SExpr>;
    fn get_bindings(&self) -> Vec<(&String, &SExpr)>;
    fn get_root(env: ProcedureEnv) -> ProcedureEnv;
}

#[derive(Clone, Debug)]
pub struct Environment {
    parent: Option<EnvAccessor<Environment>>,
    table: HashMap<String, SExpr>,
}

impl SchemeEnvironment for Environment {
    fn new() -> Environment {
        Environment { parent: None, table: HashMap::new() }
    }

    fn new_child(parent: EnvAccessor<Self>) -> EnvAccessor<Self> {
        let env = Environment { parent: Some(parent), table: HashMap::new() };
        EnvAccessor::new(env)
    }

    fn define(&mut self, key: &str, value: &SExpr) -> Result<(), String> {
        self.table.insert(key.to_string(), value.clone());

        Ok(())
    }

    fn set(&mut self, key: &str, value: &SExpr) -> Result<(), String> {
        if self.table.contains_key(key) {
            self.table.insert(key.to_string(), value.clone());

            Ok(())
        } else {
            match self.parent {
                Some(ref parent) => parent.access_mut().set(key, value),
                None => Err(format!("Exception: {} is not bound", key)),
            }
        }
    }

    fn get(&self, key: &str) -> Option<SExpr> {
        match self.table.get(key) {
            Some(val) => Some(val.clone()),
            None => match self.parent {
                Some(ref parent) => parent.access().get(key),
                None => None,
            },
        }
    }

    fn get_bindings(&self) -> Vec<(&String, &SExpr)> {
        let symbols: Vec<(&String, &SExpr)> = self.table.iter().map(|e| (e.0, e.1)).collect();

        symbols
    }

    fn get_root(env: ProcedureEnv) -> ProcedureEnv {
        match &env.access().parent {
            Some(frame) => Environment::get_root(frame.clone()),
            None => env.clone(),
        }
    }
}

macro_rules! bind_numerical_constants {
    ($env:expr, { $($name:expr => $value:ident)* }) => {
        $(
            $env.define($name, &SExpr::Number(NumericalConstant::$value)).unwrap();
        )*
    };
}

macro_rules! bind_primitives {
    ($env:expr, { $($name:expr => $value:ident)* }) => {
        $(
            $env.define($name, &SExpr::Procedure(Procedure::Primitive(Primitive::$value))).unwrap();
        )*
    };
}

macro_rules! bind_special_forms  {
    ($env:expr, { $($name:expr => $value:ident)* }) => {
        $(
            $env.define($name, &SExpr::Procedure(Procedure::SpecialForm(SpecialForm::$value))).unwrap();
        )*
    };
}

impl Default for Environment {
    fn default() -> Self {
        let mut new_env = Environment::new();
        bind_numerical_constants!(new_env, {
            "π" => PI
            "pi" => PI
            "avogadro" => AVOGADRO
            "boltzmann" => BOLTZMANN
            "e" => EULER
            "euler" => EULER
            "golden-ratio" => GOLDEN_RATIO
            "gravitational-constant" => GRAVITATIONAL_CONSTANT
            "h" => PLANCK
            "planck" => PLANCK
        });
        bind_primitives!(new_env, {
            "+" => SUM
            "-" => DIFF
            "*" => PROD
            "/" => QUOT
            "=" => EQUAL
            ">" => GT
            ">=" => GE
            "<" => LT
            "<=" => LE
            "exit" => EXIT
            "eval" => EVAL
            "apply" => APPLY
            "car" => CAR
            "cdr" => CDR
            "complex?" => IS_COMPLEX
            "cons" => CONS
            "list" => LIST
            "set-car!" => SET_CAR
            "display" => DISPLAY
            "char?" => IS_CHAR
            "symbol?" => IS_SYMBOL
            "string?" => IS_STRING
            "boolean?" => IS_BOOLEAN
            "nan?" => IS_NAN
            "number?" => IS_NUMBER
            "exact?" => IS_EXACT
            "infinite?" => IS_INFINITE
            "integer?" => IS_INTEGER
            "rational?" => IS_RATIONAL
            "real?" => IS_REAL
            "list?" => IS_LIST
            "pair?" => IS_PAIR
            "vector?" => IS_VECTOR
            "procedure?" => IS_PROCEDURE
            "null?" => IS_NULL
            "environment-bindings" => ENVIRONMENT_BINDINGS
            "append" => APPEND
            "length" => LENGTH
            "list-ref" => LIST_REF
            "list-tail" => LIST_TAIL
            "reverse" => REVERSE
            "string" => STRING
            "make-string" => MAKE_STRING
            "string-append" => STRING_APPEND
            "string-length" => STRING_LENGTH
            "string-ref" => STRING_REF
            "string-set!" => STRING_SET
            "string-upcase" => STRING_UPCASE
            "string-downcase" => STRING_DOWNCASE
            // "flatten" => FLATTEN
            // "unflatten" => UNFLATTEN
            "zero?" => IS_ZERO
        });
        bind_special_forms!(new_env,
        {
            "'" => QUOTE
            "quote" => QUOTE
            "`" => QUASIQUOTE
            "quasiquote" => QUASIQUOTE
            "and" => AND
            "or" => OR
            "λ" => LAMBDA
            "lambda" => LAMBDA
            "let" => LET
            "let*" => LET_STAR
            "define" => DEFINE
            "set!" => SET
            "if" => IF
            "time" => TIME
            "begin" => BEGIN
            "not" => NOT
            "cond" => COND
        });

        new_env
    }
}
