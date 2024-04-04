use schemius::core::interpreter::Interpreter;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern {
//     pub fn alert(s: &str);
// }

#[wasm_bindgen]
pub fn evaluate(expression: &str) -> String {
    Interpreter::default().eval_expression_and_format(expression.to_string())
}
