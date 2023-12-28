#![allow(unused)]

use std::{path::Path, ffi::OsStr, fs::File, io::Read};
use bincode;

use compiler::make_ast;
use exepitons::GenericError;
use instructions::Instr;
use runtime::vm::CHSVM;
pub mod runtime;
pub mod compiler;
pub mod instructions;
pub mod exepitons;
pub mod value;
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
        Err(_) => return generic_error!("..."),
    };

    let mut buffer = String::new();

    match f.read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(_) => return generic_error!("...")
    }
    Ok((buffer, name))

}
// 2. compile the file.chs to file.chsb
pub fn compile_chs_file(source: (String, &String)) -> Result<String, GenericError> {

    let ast = match make_ast(source.0) {
        Ok(s) => s.stmt,
        Err(e) => {
            return Err(e);
        }
    };

    // Encode to something implementing `Write`
    let mut f = File::create(format!("./{}.chsb", source.1)).unwrap();
    bincode::serialize_into(&mut f, &ast).unwrap();
    
    return Ok(format!("./{}.chsb", source.1));
}

// 3. run the file.chsb
pub fn run_file(name: String) -> Result<(), GenericError> {
    let mut f = match File::open(name) {
        Ok(ok) => ok,
        Err(_) => return generic_error!("..."),
    };

    let mut buffer = Vec::new();

    match f.read_to_end(&mut buffer) {
        Ok(_) => {}
        Err(_) => return generic_error!("...")
    }
    let a = match bincode::deserialize::<Vec<Instr>>(&buffer[..]) {
        Ok(ok) => ok,
        Err(_) => generic_error!("...")
    };

    let mut vm = CHSVM::new(a);

    vm.run();

    Ok(())
}
