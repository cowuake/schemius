use schemius::Interpreter;
use std::sync::{LazyLock, Mutex};
use wasm_bindgen::prelude::*;

static INTERPRETER: LazyLock<Mutex<Interpreter>> =
    LazyLock::new(|| Mutex::new(Interpreter::default()));

#[wasm_bindgen]
pub fn evaluate(expression: &str) -> String {
    match INTERPRETER.try_lock().unwrap().eval_expression_and_format(expression.to_string()) {
        Ok(result) => result,
        Err(err) => err,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        assert_eq!(evaluate("(display 'hello)"), "hello");
    }
}
