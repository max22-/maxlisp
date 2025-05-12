use std::env;
use std::fs;
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

    let lexer = lexer::Lexer::new(&source);

    for tok in lexer.into_iter() {
        match tok {
            Ok(t) => println!("{:?}", String::from_utf8(t.val.to_vec())),
            Err(e) => println!("{:?}", e)
        }
    }
    
}
