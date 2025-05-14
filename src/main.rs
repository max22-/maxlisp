use std::env;
use std::fs;

mod lexer;
mod parser;
use parser::Parser;
mod interner;
mod sexp;
use interner::Interner;

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

    let mut interner = Interner::new();
    let mut parser = Parser::new(&source);

    loop {
        match parser.next_form(&mut interner) {
            Ok(o) => match o {
                Some(s) => println!("{}", s.to_string(&interner)),
                None => break,
            },
            Err(e) => {
                println!("{}", e.to_string(file_path, &source));
                break;
            }
        }
    }
}
