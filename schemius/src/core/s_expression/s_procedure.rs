use crate::core::environment::{EnvAccessor, Environment};

use super::{ListImplementation, SExpr};

pub type ProcedureArgs = ListImplementation;
pub type ProcedureEnv = EnvAccessor<Environment>;
pub type ProcedureOutput = Result<SExpr, String>;
pub type ProcedureSignature = fn(ProcedureArgs, ProcedureEnv) -> ProcedureOutput;

pub type SpecialFormOutput = Result<SExpr, String>;
pub type SpecialFormSignature = fn(ProcedureArgs, ProcedureEnv) -> SpecialFormOutput;

#[derive(Clone, Debug)]
pub enum Procedure {
    SpecialForm(SpecialFormSignature),
    Primitive(ProcedureSignature),
    Compound(Vec<String>, Vec<SExpr>, ProcedureEnv),
}
