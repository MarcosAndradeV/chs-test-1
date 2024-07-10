use std::{env, fs, process};

use chs::{
    chs_frontend::parser::Parser, chs_vm::{bytecode_compiler::IrParser, instructions::Bytecode, vm_run}
};


fn main() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("");
    if let Some(ref file) = args.next() {
        let data = fs::read(file).map_err(|err| eprintln!("{err}"))?;
        let ast = match Parser::new(data).parse_to_ast() {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("{}",e.0);
                return Ok(());
            }
        };
        let bytecode: Bytecode = match IrParser::new(ast).parse() {
            Ok(code) => code,
            Err(e) => {
                eprintln!("{e:?}");
                process::exit(1);
            }
        };
        //let mut vm = CHSVM::new(bytecode);
        //vm.run();
        vm_run(bytecode);
        return Ok(());
    }else{
        println!("File not provided.");
        println!("Usage: {program} <file.chs>");
        return Ok(());
    }
}
