pub mod lexer;
pub mod parser;
pub mod ast;
pub mod ir;
pub mod codegen;
pub mod regalloc;

use std::fs;

pub fn compile_file(input: &str, output: &str) -> Result<(), String> {
    let source = fs::read_to_string(input)
        .map_err(|e| format!("cannot read {}: {}", input, e))?;

    let tokens = lexer::lex(&source)?;
    let ast = parser::parse(&tokens)?;
    let ir = ir::lower_ast(&ast)?;
    let asm = codegen::generate(&ir)?;

    crate::assembler::bin::write_osl_bin(output, &asm, 0x1000, 0x2000)?;

    Ok(())
}
