use std::{env, process};
use std::fs::File;
use std::io::{Read, self};
use std::process::exit;

use chsvm;


use crate::chsvm::compiler::make_ast;
use crate::chsvm::runtime::vm::CHSVM;


fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {println!("Usage: chsvm <file.chs>"); process::exit(0)}
        2 => {},
        _ => {println!("Usage: chsvm <file.chs>"); process::exit(0)}
    }

    let name = match args.get(1) {
        Some(name) => name,
        None => {println!("Usage: chsvm <file.chs>"); process::exit(0)}
    };
    
    let mut f = File::open(name)?;
    let mut buffer = String::new();

    f.read_to_string(&mut buffer)?;
    
    let ast = match make_ast(buffer) {
        Ok(s) => s.stmt,
        Err(e) => {
            println!("{:?}", e);
            exit(1);
        }
    };
    let mut vm = CHSVM::new(ast);
    vm.run();
    Ok(())
}