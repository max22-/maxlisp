use std::env;
use std::fs;

use lexer::LexerErrorType;
mod lexer;

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

    let mut lexer = lexer::Lexer::new(&source);

    loop {
        match lexer.next_token() {
            Ok(t) => println!("{:?}", t),
            Err(e) => match e.r#type {
                LexerErrorType::Eof => break,
                _ => {
                    println!("{:?}", e);
                    break;
                }
            }
        }

    }
    
}
