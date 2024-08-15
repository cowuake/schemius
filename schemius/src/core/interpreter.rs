use std::fs::read_to_string;
use std::io::{self, Write};

use crate::core::reader;
use crate::scheme::prelude::PRELUDE;

use super::constants::tokens;
use super::environment::Environment;
use super::evaluator::EvalOutput;
use super::{evaluator::Evaluator, s_expression::SExpr};

pub struct Interpreter {
    current_expression: String,
    evaluator: Evaluator,
    line_idx: usize,
    lines: Vec<String>,
}

impl Interpreter {
    pub fn new(environment: Option<Environment>) -> Self {
        let evaluator = match environment {
            Some(env) => Evaluator::new(Some(env)),
            None => Evaluator::default(),
        };
        let source = PRELUDE.lines().fold("".to_string(), |current, next| current + next);
        let mut prelude = source.to_string();
        let expression = match reader::read(&mut prelude) {
            Ok(expr) => expr,
            Err(_) => SExpr::Unspecified,
        };

        match evaluator.eval(&expression) {
            Ok(_) => {
                Self { current_expression: String::new(), evaluator, line_idx: 0, lines: vec![] }
            }
            Err(_) => Self {
                current_expression: String::new(),
                evaluator: Evaluator::default(),
                line_idx: 0,
                lines: vec![],
            },
        }
    }

    fn read(&mut self, next_line: &dyn Fn(&mut Interpreter) -> Result<String, String>) {
        loop {
            // TODO: Handle the case of parentheses inside strings and all other similar cases
            //      -> Only parenthes used as syntax elements should be considered here
            let lparens =
                self.current_expression.chars().filter(|c| c == &'(' || c == &'[').count();
            let rparens =
                self.current_expression.chars().filter(|c| c == &')' || c == &']').count();

            if lparens == rparens {
                break;
            }

            let next = next_line(self).unwrap_or("".to_string());

            if next.is_empty() || next.starts_with(tokens::PREFIX_COMMENT) {
                continue;
            }

            self.current_expression.push_str(&next)
        }
    }

    fn eval(&self, expression: &SExpr) -> EvalOutput {
        self.evaluator.eval(expression)
    }

    fn print(&self, expression: &SExpr) {
        match expression {
            SExpr::Unspecified => print!("{}", SExpr::Unspecified),
            printable => println!("{}", printable),
        }
    }

    fn print_error(&self, error: &str) {
        println!("{}", error);
    }

    fn main_loop(
        &mut self, next_line: &dyn Fn(&mut Interpreter) -> Result<String, String>,
    ) -> Result<(), String> {
        let mut expression: Result<SExpr, String>;

        Ok(loop {
            self.current_expression = next_line(self)?;
            self.read(next_line);

            if self.current_expression.is_empty()
                || self.current_expression.starts_with(tokens::PREFIX_COMMENT)
            {
                continue;
            }

            if self.current_expression == tokens::EOF {
                break;
            }

            expression = reader::read(&mut self.current_expression);
            match self.eval(&expression?) {
                Ok(expr) => self.print(&expr),
                Err(e) => self.print_error(&e),
            }
        })
    }

    pub fn run_repl(&mut self) -> Result<(), String> {
        self.main_loop(&read_line_from_repl)
    }

    pub fn execute_file(&mut self, path: String) -> Result<(), String> {
        match read_to_string(path.as_str()) {
            Ok(file) => {
                for line in file.lines() {
                    self.lines.push(line.trim().to_string());
                }
                self.main_loop(&read_line_from_file)
            }
            Err(_) => Err(format!("Could not read file {}", path)),
        }
    }

    fn is_preliminarily_validated(&self, expression_string: &str) -> bool {
        // TODO: Judge the validity of the approach and extend the function if it's worth
        !((expression_string.trim().starts_with(tokens::OPEN_PAREN)
            && !expression_string.trim_end().ends_with(tokens::CLOSED_PAREN))
            || (expression_string.trim().starts_with(tokens::OPEN_BRACKET)
                && !expression_string.trim_end().ends_with(tokens::CLOSED_BRACKET)))
    }

    pub fn eval_expression(&mut self, expression_string: String) -> EvalOutput {
        if !self.is_preliminarily_validated(&expression_string) {
            return Err("Exception: Invalid syntax.".to_string());
        }

        let mut line = expression_string.clone();
        let expression: SExpr = reader::read(&mut line)?;

        self.eval(&expression)
    }

    pub fn eval_expression_no_print(&mut self, expression_string: String) -> Result<(), String> {
        match self.eval_expression(expression_string) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn eval_expression_and_format(
        &mut self, expression_string: String,
    ) -> Result<String, String> {
        match self.eval_expression(expression_string) {
            Ok(expr) => Ok(format!("{}", expr)),
            Err(e) => Err(e),
        }
    }

    pub fn eval_expression_and_print(&mut self, expression_string: String) -> Result<(), String> {
        let result = self.eval_expression(expression_string)?;
        self.print(&result);
        Ok(())
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new(None)
    }
}

fn read_line_from_file(interpreter: &mut Interpreter) -> Result<String, String> {
    if interpreter.lines.len() > interpreter.line_idx {
        let line = interpreter.lines[interpreter.line_idx].clone();
        interpreter.line_idx += 1;

        Ok(line)
    } else {
        Ok(String::from(tokens::EOF))
    }
}

fn read_line_from_repl(_: &mut Interpreter) -> Result<String, String> {
    let mut line = String::new();

    io::stdout().write_all(b"> ").unwrap();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut line).unwrap();

    line = line.strip_suffix('\n').or(line.strip_suffix('\r')).unwrap().to_string();
    line.extend([' ']);

    Ok(line.trim().to_string())
}
