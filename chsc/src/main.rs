use std::{env, fs, process};

use chs_ast::parser::parse;
use chs_vm::bytecode_compiler::IrParser;
use chs_vm::vm_run;

fn main() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("");
    if let Some(ref file) = args.next() {
        let data = fs::read(file).map_err(|err| eprintln!("{err}"))?;
        let ast = match parse(file.clone(), data) {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("{}",e.0);
                return Ok(());
            }
        };
        let bytecode = match IrParser::new(ast.program).parse() {
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
