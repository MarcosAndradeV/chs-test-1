use std::{env, io, process};

use chs::{
    chs_frontend::{ast::Program, lexer::read_file_to_bytes, parser::Parser},
    chs_vm::{bytecode_compiler::IrParser, instructions::Bytecode, vm::CHSVM},
};

fn main() -> io::Result<()> {
    let mut args = env::args();
    let chsc = args.next().expect("Program");
    if let Some(filename) = args.next() {
        let bytes = read_file_to_bytes(filename.into())?;
        let mut fist_parser = Parser::new(bytes);
        let program: Program = match fist_parser.parse_to_ast() {
            Ok(prog) => prog,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        let mut second_parser = IrParser::new(program);
        let bytecode: Bytecode = match second_parser.parse() {
            Ok(code) => code,
            Err(e) => {
                eprintln!("{e}");
                process::exit(1);
            }
        };
        let mut vm = CHSVM::new(bytecode);
        vm.run(false);
        return Ok(());
    }else{
        println!("File not provided.");
        println!("Usage: {chsc} run <file.chs>");
        return Ok(());
    }
}
