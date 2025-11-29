pub mod lexer;
pub mod parser;
pub mod opcodes;
pub mod emitter;
pub mod bin;

use lexer::*;
use parser::*;
use emitter::*;
use bin::*;

use std::fs;

pub fn assemble_file(path: &str, out: &str) -> Result<(), String> {
    let input = fs::read_to_string(path)
        .map_err(|e| format!("cannot read {}: {}", path, e))?;

    let tokens = lex(&input)?;
    let ast = parse(&tokens)?;
    let encoded = emit(&ast)?;

    write_osl_bin(out, &encoded, 0x1000, 0x2000)?;

    println!("Assembled {} instructions", encoded.len());
    Ok(())
}
