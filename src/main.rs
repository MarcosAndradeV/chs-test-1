use std::fs::File;
use std::io::{Read, self};
use std::process::exit;

use chsvm;


use crate::chsvm::compiler::make_ast;
use crate::chsvm::runtime::vm::CHSVM;


fn main() -> io::Result<()> {
    
    let mut f = File::open("main.chs")?;
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