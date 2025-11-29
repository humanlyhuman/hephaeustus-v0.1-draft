mod lexer;
mod parser;
mod opcodes;
mod emitter;
mod bin;

use std::env;
use std::fs;

fn assemble_file(path: &str, out: &str) -> Result<(), String> {
    let input = fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {}", path, e))?;

    let tokens = lexer::lex(&input)?;
    let ast = parser::parse(&tokens)?;
    let encoded = emitter::emit(&ast)?;

    bin::write_osl_bin(out, &encoded, 0x1000, 0x200000)?;

    println!("Assembled {} instructions", encoded.len());
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <input.asm> <output.oslbin>", args[0]);
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    match assemble_file(input, output) {
        Ok(_) => println!("Successfully assembled {} -> {}", input, output),
        Err(e) => {
            eprintln!("Assembly failed: {}", e);
            std::process::exit(1);
        }
    }
}
