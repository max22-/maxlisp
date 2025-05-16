use std::env;
use std::fs;

mod lexer;
mod parser;
use evaluator::EvalItem;
use evaluator::Evaluator;
use evaluator::builtins;
use parser::Parser;
mod context;
mod evaluator;
mod sexp;
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
                    evaluator.push_back(EvalItem::Operand(s));
                    evaluator.push_back(EvalItem::Operator(builtins::eval, "eval"));
                    match evaluator.run(&mut ctx) {
                        Ok(()) => (),
                        Err(e) => println!("{:?}", e),
                    };
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
