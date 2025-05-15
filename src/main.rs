use std::env;
use std::fs;

mod lexer;
mod parser;
use evaluator::Evaluator;
use parser::Parser;
mod sexp;
mod evaluator;
mod context;
use context::Context;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: {} file.lsp", args[0]);
        return;
    }
    let file_path = &args[1];
    let source = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(e) => {
            println!("failed to open {}: {}", file_path, e);
            return;
        }
    };

    let mut ctx = Context::new();
    let mut parser = Parser::new(&source);
    let mut evaluator = Evaluator::new(&mut ctx);

    loop {
        match parser.next_form(&mut ctx) {
            Ok(o) => match o {
                Some(s) => {
                    println!("{}", ctx.heap.get_ref(s).to_string(&ctx));
                    evaluator.push_back(s);
                    evaluator.eval(&mut ctx);
                }
                None => break,
            },
            Err(e) => {
                println!("{}", e.to_string(file_path, &source));
                break;
            }
        }
    }
}
