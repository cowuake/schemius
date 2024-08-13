use super::{accessor::*, environment::*, evaluator::*, s_expression::*};

mod base_procs;
mod boolean_procs;
mod list_procs;
mod number_procs;
mod special_forms;
mod string_procs;

use base_procs::*;
use boolean_procs::*;
use list_procs::*;
use number_procs::*;
use special_forms::*;
use string_procs::*;

pub struct Primitive;
pub struct SpecialForm;

impl Primitive {
    pub const EXIT: ProcedureSignature = r_exit;
    pub const SUM: ProcedureSignature = r_sum;
    pub const DIFF: ProcedureSignature = r_diff;
    pub const PROD: ProcedureSignature = r_prod;
    pub const QUOT: ProcedureSignature = r_quot;
    pub const EQUAL: ProcedureSignature = r_equal;
    pub const GT: ProcedureSignature = r_gt;
    pub const GE: ProcedureSignature = r_ge;
    pub const LT: ProcedureSignature = r_lt;
    pub const LE: ProcedureSignature = r_le;
    pub const EVAL: ProcedureSignature = r_eval;
    pub const APPLY: ProcedureSignature = r_apply;
    pub const CAR: ProcedureSignature = r_car;
    pub const CDR: ProcedureSignature = r_cdr;
    pub const CONS: ProcedureSignature = r_cons;
    pub const LIST: ProcedureSignature = r_list;
    pub const SET_CAR: ProcedureSignature = r_set_car;
    pub const DISPLAY: ProcedureSignature = r_display;
    pub const IS_BOOLEAN: ProcedureSignature = r_is_boolean;
    pub const IS_CHAR: ProcedureSignature = r_is_char;
    pub const IS_COMPLEX: ProcedureSignature = r_is_complex;
    pub const IS_EXACT: ProcedureSignature = r_is_exact;
    pub const IS_INFINITE: ProcedureSignature = r_is_infinite;
    pub const IS_INTEGER: ProcedureSignature = r_is_integer;
    pub const IS_LIST: ProcedureSignature = r_is_list;
    pub const IS_NAN: ProcedureSignature = r_is_nan;
    pub const IS_NULL: ProcedureSignature = r_is_null;
    pub const IS_NUMBER: ProcedureSignature = r_is_number;
    pub const IS_PAIR: ProcedureSignature = r_is_pair;
    pub const IS_PROCEDURE: ProcedureSignature = r_is_procedure;
    pub const IS_RATIONAL: ProcedureSignature = r_is_rational;
    pub const IS_REAL: ProcedureSignature = r_is_real;
    pub const IS_STRING: ProcedureSignature = r_is_string;
    pub const IS_SYMBOL: ProcedureSignature = r_is_symbol;
    pub const IS_VECTOR: ProcedureSignature = r_is_vector;
    pub const IS_ZERO: ProcedureSignature = r_is_zero;
    pub const ENVIRONMENT_BINDINGS: ProcedureSignature = r_environment_bindings;
    pub const APPEND: ProcedureSignature = r_append;
    pub const LENGTH: ProcedureSignature = r_length;
    pub const LIST_REF: ProcedureSignature = r_list_ref;
    pub const LIST_SPLICE: ProcedureSignature = r_list_splice;
    pub const LIST_TAIL: ProcedureSignature = r_list_tail;
    pub const REVERSE: ProcedureSignature = r_reverse;
    pub const MAKE_STRING: ProcedureSignature = r_make_string;
    pub const STRING: ProcedureSignature = r_string;
    pub const STRING_APPEND: ProcedureSignature = r_string_append;
    pub const STRING_DOWNCASE: ProcedureSignature = r_string_downcase;
    pub const STRING_LENGTH: ProcedureSignature = r_string_length;
    pub const STRING_REF: ProcedureSignature = r_string_ref;
    pub const STRING_SET: ProcedureSignature = r_string_set;
    pub const STRING_UPCASE: ProcedureSignature = r_string_upcase;
}

impl SpecialForm {
    pub const BEGIN: SpecialFormSignature = r_begin;
    pub const COND: SpecialFormSignature = r_cond;
    pub const DEFINE: SpecialFormSignature = r_define;
    pub const IF: SpecialFormSignature = r_if;
    pub const LAMBDA: SpecialFormSignature = r_lambda;
    pub const LET: SpecialFormSignature = r_let;
    pub const LET_STAR: SpecialFormSignature = r_let_star;
    pub const NOT: SpecialFormSignature = r_not;
    pub const QUOTE: SpecialFormSignature = r_quote;
    pub const QUASIQUOTE: SpecialFormSignature = r_quasiquote;
    pub const SET: SpecialFormSignature = r_set;
    pub const TIME: SpecialFormSignature = r_time;
    pub const AND: SpecialFormSignature = r_and;
    pub const OR: SpecialFormSignature = r_or;
}
