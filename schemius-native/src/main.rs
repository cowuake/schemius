use clap::Parser;
use schemius::core::interpreter::Interpreter;

/// Simple Scheme interpreter born as a personal learning project
#[derive(Parser, Debug)]
#[command(author="Riccardo Mura", version, about, long_about = None)]
struct Args {
    /// Eval expression (without printing return values)
    #[arg(short, long, value_name = "EXPRESSION")]
    eval: Option<String>,

    /// Eval expression and print expression outcome
    #[arg(short, long, value_name = "EXPRESSION")]
    print: Option<String>,

    /// Interpret Scheme source file
    #[arg(short, long, value_name = "FILE PATH")]
    source: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut interpreter = Interpreter::default();

    match args.eval {
        Some(expr) => {
            interpreter.eval_expression_no_print(expr);
            return;
        }
        _ => (),
    }

    match args.print {
        Some(expr) => return interpreter.eval_expression_and_print(expr),
        _ => (),
    }

    match args.source {
        Some(path) => return interpreter.execute_file(path),
        _ => (),
    }

    println!(
        "
    ███████╗ ██████╗██╗  ██╗███████╗███╗   ███╗██╗██╗   ██╗███████╗
    ██╔════╝██╔════╝██║  ██║██╔════╝████╗ ████║██║██║   ██║██╔════╝
    ███████╗██║     ███████║█████╗  ██╔████╔██║██║██║   ██║███████╗
    ╚════██║██║     ██╔══██║██╔══╝  ██║╚██╔╝██║██║██║   ██║╚════██║
    ███████║╚██████╗██║  ██║███████╗██║ ╚═╝ ██║██║╚██████╔╝███████║
    ╚══════╝ ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚═╝ ╚═════╝ ╚══════╝
    "
    );
    println!("Welcome to Schemius!");
    println!("\t(environment-bindings) -> Show bindings in current env");
    println!("\t(exit)                 -> Exit program");
    println!();
    interpreter.run_repl()
}
