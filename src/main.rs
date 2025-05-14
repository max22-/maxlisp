use std::env;
use std::fs;

mod lexer;
mod parser;
use gc_heap::GcHeap;
use parser::Parser;
mod interner;
mod sexp;
use interner::Interner;
mod gc_heap;

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
    let mut heap = GcHeap::new();

    loop {
        match parser.next_form(&mut heap, &mut interner) {
            Ok(o) => match o {
                Some(s) => println!("{}", heap.get_ref(s).to_string(&heap, &interner)),
                None => break,
            },
            Err(e) => {
                println!("{}", e.to_string(file_path, &source));
                break;
            }
        }
    }
}
