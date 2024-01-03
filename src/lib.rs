#![allow(unused)]

use std::{path::Path, ffi::OsStr, fs::File, io::Read};
use bincode;

use bytecode::{instructions::Instr, ByteCode};
use compiler::make_ast;
use exepitons::GenericError;

use runtime::vm::CHSVM;
pub mod runtime;
pub mod compiler;
pub mod bytecode;
pub mod exepitons;
mod tests;
pub mod cli;



// 1. load the file.chs
pub fn load_file(name: &String) -> Result<(String, &String), GenericError> {
    let file_path = Path::new(name);
    if !file_path.exists() {
        return generic_error!("File do not exists.");
    }
    match file_path.extension() {
        Some(v) => {
            if !v.eq_ignore_ascii_case("chs") {
                return generic_error!("File extension is not .chs.");
            }
        }
        None => {return generic_error!("File extension is not .chs."); }
    }
    // Now is .chs :)

    let mut f = match File::open(file_path) {
        Ok(ok) => ok,
        Err(_) => return generic_error!("Cannot open the file"),
    };

    let mut buffer = String::new();

    match f.read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(_) => return generic_error!("Cannot read the file")
    }
    Ok((buffer, name))

}

pub fn run_chs_file(source: (String, &String)) -> Result<(), GenericError> {

    let ast = match make_ast(source.0) {
        Ok(s) => ByteCode { code: s.stmt },
        Err(e) => {
            return Err(e);
        }
    };

    let mut vm = CHSVM::new(ast);

    vm.run();

    Ok(())
}
