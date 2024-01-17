use std::io;

use chs::{lex_file, vm::CHSVM, compiler::parser::Parser};
use clap::{Arg, Command};

fn main() -> io::Result<()>{
    // Basic CLI just for tests.
    let cmd = Command::new("chsvm")
        .about("...")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Marcos V. Andrade Almeida")
        .subcommand(
            Command::new("run")
                .about("...")
                .arg(Arg::new("filename").value_name("FILE").num_args(1)),

        )
        .get_matches();

    match cmd.subcommand() {
        Some(("run", file_matches)) => {
            if file_matches.contains_id("filename") {
                let filename = file_matches
                    .get_one::<String>("filename")
                    .expect("contains_id");
                let bytes = lex_file(filename.into())?;
                let code = match Parser::new(bytes).parse() {
                    Ok(o) => o,
                    Err(e) => {eprintln!("{}", e); return Ok(())}
                };
                let mut vm = CHSVM::new(code);
                vm.run();
                return Ok(());
            }
            println!("File not provided.");
            println!("Usage: chsvm <file.chs>");
            return Ok(());

        }
        _ => unreachable!()
    }
}