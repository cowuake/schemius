use super::{
    s_list::SList,
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

                match args.s_car().unwrap().$source_fn() {
                    Ok(res) => Ok(SExpr::Boolean(res)),
                    Err(e) => Err(e)
                }
            }
        )*}
}

fn_is! {
    r_is_boolean, is_boolean, "boolean?"
    r_is_char, is_char, "char?"
    r_is_complex, is_complex, "complex?"
    r_is_exact, is_exact, "exact?"
    r_is_infinite, is_infinite, "infinite?"
    r_is_integer, is_integer, "integer?"
    r_is_list, is_list, "list?"
    r_is_nan, is_nan, "nan?"
    r_is_rational, is_rational, "rational?"
    r_is_real, is_real, "real?"
    r_is_null, is_null, "null?"
    r_is_number, is_number, "number?"
    r_is_pair, is_pair, "pair?"
    r_is_procedure, is_procedure, "procedure?"
    r_is_string, is_string, "string?"
    r_is_symbol, is_symbol, "symbol?"
    r_is_vector, is_vector, "vector?"
    r_is_zero, is_zero, "zero?"
}
