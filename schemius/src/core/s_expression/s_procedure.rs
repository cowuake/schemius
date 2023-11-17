use std::sync::{Arc, Mutex};

use crate::core::environment::Environment;

use super::SExpr;

pub type ProcedureArgs = Vec<SExpr>;
pub type ProcedureEnv = Arc<Mutex<Environment>>;
pub type ProcedureOutput = Result<SExpr, String>;
pub type ProcedureSignature = fn(ProcedureArgs, ProcedureEnv) -> ProcedureOutput;

pub type SpecialFormOutput = Result<SExpr, String>;
pub type SpecialFormSignature = fn(ProcedureArgs, ProcedureEnv) -> SpecialFormOutput;

#[derive(Clone, Debug)]
pub enum Procedure {
    SpecialForm(SpecialFormSignature),
    Primitive(ProcedureSignature),
    Compound(Vec<String>, Vec<SExpr>, Arc<Mutex<Environment>>),
}
