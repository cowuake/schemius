use lazy_static::lazy_static;
use schemius::Interpreter;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

lazy_static! {
    static ref INTERPRETER: Mutex<Interpreter> = Mutex::new(Interpreter::default());
}

#[wasm_bindgen]
pub fn evaluate(expression: &str) -> String {
    INTERPRETER.try_lock().unwrap().eval_expression_and_format(expression.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate("(display 'hello)"), "hello");
    }
}
