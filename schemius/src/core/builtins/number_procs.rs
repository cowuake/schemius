use super::{
    s_procedure::{ProcedureArgs, ProcedureEnv, ProcedureOutput},
    SExpr, SNumber,
};

macro_rules! fn_compute_sum_prod {
    ($($fn:ident: $op:tt, $neutral:literal)*) => {
    $(
        pub fn $fn(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
            match args.len() {
                0 => Ok(SExpr::Number(SNumber::Int($neutral))),
                _ => {
                    let mut res = SNumber::Int($neutral);

                    for arg in &args[0..] {
                        match arg {
                            SExpr::Number(n) => res = &res $op &n,
                            num => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), num)),
                        }
                    }

                    Ok(SExpr::Number(res))
                }
            }
        }
    )*}
}

macro_rules! fn_compute_diff_quot {
    ($($fn:ident: $op:tt, $neutral:literal)*) => {
    $(
        pub fn $fn(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
            match args.len() {
                0 => Err(String::from("Exception: too few arguments")),
                _ => {
                    let mut res = match &args[0] {
                            SExpr::Number(num) => num.clone(),
                            num => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), num)),
                    };

                    if args.len() > 1 {
                        for arg in &args[1..] {
                            match arg {
                                SExpr::Number(n) => res = &res $op &n,
                                num => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), num)),
                            }
                        }
                    } else {
                        res = SNumber::Int($neutral) $op res;
                    }

                    Ok(SExpr::Number(res))
                }
            }
        }
    )*}
}

fn_compute_sum_prod! {
    r_sum: +, 0
    r_prod: *, 1
}

fn_compute_diff_quot! {
    r_diff: -, 0
    r_quot: /, 1
}

macro_rules! fn_compare {
    ($($fn:ident: $op:tt)*) => {
        $(
            pub fn $fn(args: ProcedureArgs, _: ProcedureEnv) -> ProcedureOutput {
                if args.len() < 2 {
                    return Err(format!("Exception in {}: expected at least 2 arguments, found {}", stringify!($op), args.len()));
                }

                let mut result = true;

                for (lhs, rhs) in args.iter().zip(args.iter().skip(1)) {
                    match lhs {
                        SExpr::Number(left) => {
                            match rhs {
                                SExpr::Number(right) => {
                                    if !(left $op right) {
                                        result = false;
                                        break;
                                    }
                                },
                                other => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), other)),
                            }
                        },
                        other => return Err(format!("Exception in {}: #<{}> is not a number", stringify!($op), other)),
                    }
                }

                Ok(SExpr::Boolean(result))
            }
        )*}
}

fn_compare! {
    r_equal: ==
    r_gt: >
    r_ge: >=
    r_lt: <
    r_le: <=
}
