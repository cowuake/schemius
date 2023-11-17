use std::fs::read_to_string;
use std::io::{self, Write};

use crate::core::reader;
use crate::scheme::prelude::PRELUDE;

use super::evaluator::EvalOutput;
use super::{evaluator::Evaluator, s_expression::SExpr};

pub struct Interpreter {
    current_expression: String,
    evaluator: Evaluator,
    line_idx: usize,
    lines: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Self {
        let evaluator = Evaluator::new();
        let source = PRELUDE.lines().fold("".to_string(), |current, next| current + next);
        let mut prelude = source.to_string();
        let expression = reader::read(&mut prelude);

        match evaluator.eval(&expression) {
            Ok(_) => Self { current_expression: String::new(), evaluator: evaluator, line_idx: 0, lines: vec![] },
            Err(_) => Self { current_expression: String::new(), evaluator: Evaluator::new(), line_idx: 0, lines: vec![] },
        }
    }

    fn read(&mut self, next_line: &dyn Fn(&mut Interpreter) -> String) {
        loop {
            let lparens = self.current_expression.chars().filter(|c| c == &'(' || c == &'[').count();
            let rparens = self.current_expression.chars().filter(|c| c == &')' || c == &']').count();

            if lparens == rparens {
                break;
            }

            let next = next_line(self);

            if next.is_empty() || next.starts_with(';') {
                continue;
            }

            self.current_expression.push_str(&next)
        }
    }

    fn eval(&self, expression: &SExpr) -> EvalOutput {
        self.evaluator.eval(expression)
    }

    fn format(&self, expression: EvalOutput) -> String {
        (match expression {
            Ok(val) => val.to_string(),
            Err(e) => e,
        })
        .to_string()
    }

    fn print(&self, expression: EvalOutput) {
        match expression {
            Ok(expr) => match expr {
                SExpr::Unspecified => print!("{}", SExpr::Unspecified),
                printable => println!("{}", printable),
            },
            Err(e) => println!("{}", e),
        }
    }

    fn main_loop(&mut self, next_line: &dyn Fn(&mut Interpreter) -> String) {
        let mut expression: SExpr;

        loop {
            self.current_expression = next_line(self);
            self.read(next_line);

            if self.current_expression.is_empty() || self.current_expression.starts_with(';') {
                continue;
            }

            if self.current_expression == "EOF" {
                break;
            }

            expression = reader::read(&mut self.current_expression);
            let result = self.eval(&expression);

            self.print(result)
        }
    }

    pub fn run_repl(&mut self) -> () {
        self.main_loop(&read_line_from_repl);
    }

    pub fn execute_file(&mut self, path: String) {
        match read_to_string(path.as_str()) {
            Ok(file) => {
                for line in file.lines() {
                    self.lines.push(line.trim().to_string());
                }
                self.main_loop(&read_line_from_file)
            }
            Err(_) => println!("Could not read file {}", path),
        }
    }

    pub fn eval_expression(&mut self, expression_string: String) -> EvalOutput {
        let mut line = expression_string.clone();
        let expression: SExpr = reader::read(&mut line);

        self.eval(&expression)
    }

    pub fn eval_expression_no_print(&mut self, expression_string: String) {
        match self.eval_expression(expression_string) {
            Ok(_) => {}
            _ => println!("Evaluation failed."),
        };
    }

    pub fn eval_expression_and_format(&mut self, expression_string: String) -> String {
        let result = self.eval_expression(expression_string);

        self.format(result)
    }

    pub fn eval_expression_and_print(&mut self, expression_string: String) {
        let result = self.eval_expression(expression_string);

        self.print(result)
    }
}

fn read_line_from_file(interpreter: &mut Interpreter) -> String {
    if interpreter.lines.len() > interpreter.line_idx {
        let line = interpreter.lines[interpreter.line_idx].clone();
        interpreter.line_idx += 1;

        line
    } else {
        String::from("EOF")
    }
}

fn read_line_from_repl(_: &mut Interpreter) -> String {
    let mut line = String::new();

    io::stdout().write(b"> ").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();

    line = line.strip_suffix('\n').or(line.strip_suffix('\r')).unwrap().to_string();
    line.extend([' ']);

    line.trim().to_string()
}
