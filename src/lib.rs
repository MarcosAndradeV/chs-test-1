use std::{path::PathBuf, fs::File, io::{self, Read}};

pub mod config;
pub mod exeptions;
pub mod instructions;
pub mod vm;
pub mod compiler;

pub fn lex_file(filepath: PathBuf) -> io::Result<Vec<u8>> {
    let mut file = File::open(filepath)?;
    let mut data = vec![];
    file.read_to_end(&mut data)?;
    Ok(data)
}
