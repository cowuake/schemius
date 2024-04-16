use super::{
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    SExpr,
};

macro_rules! fn_is {
    ($($fn:ident, $source_fn:ident, $name:literal)*) => {
        $(
            pub fn $fn(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
                if args.len() != 1 {
                    return Err(format!("Exception in {}: expected 1 argument, found {}", $name, args.len()));
                }

                match args[0].$source_fn() {
                    Ok(res) => Ok(SExpr::Boolean(res)),
                    Err(e) => Err(e)
                }
            }
        )*}
}

fn_is! {
    r_is_char, is_char, "char?"
    r_is_string, is_string, "string?"
    r_is_boolean, is_boolean, "boolean?"
    r_is_number, is_number, "number?"
    r_is_exact, is_exact, "exact?"
    r_is_list, is_list, "list?"
    r_is_pair, is_pair, "pair?"
    r_is_vector, is_vector, "vector?"
    r_is_procedure, is_procedure, "procedure?"
    r_is_symbol, is_symbol, "symbol?"
    r_is_null, is_null, "null?"
}
