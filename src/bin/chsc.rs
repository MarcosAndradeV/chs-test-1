use std::{io, process};

use chs::{compiler::{ir::{IrParser, Program}, lexer::lex_file, parser::Parser, type_checker::type_check_program}, instructions::Bytecode, vm::CHSVM};
use clap::{Arg, Command, ArgAction};

fn main() -> io::Result<()>{
    // Basic CLI just for tests.
    let cmd = Command::new("chsc")
        .about("...")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Marcos V. Andrade Almeida")
        .subcommand(
            Command::new("run")
                .about("...")
                .short_flag('R')
                .arg(Arg::new("filename").value_name("FILE").num_args(1))
                .arg(
                    Arg::new("debug")
                        .long("debug")
                        .short('d')
                        .action(ArgAction::SetTrue)
                        .help("Runs with debug mode"))
                .arg(
                    Arg::new("check")
                        .long("check")
                        .short('c')
                        .action(ArgAction::SetTrue)
                        .help("Runs with type check mode")),
                

        )
        .get_matches();

    match cmd.subcommand() {
        Some(("run", file_matches)) => {
            if file_matches.contains_id("filename") {
                let filename = file_matches
                    .get_one::<String>("filename")
                    .expect("contains_id");
                let bytes = lex_file(filename.into())?;
                let mut fist_parser = Parser::new(bytes);
                let program: Program = match fist_parser.parse_to_ir() {
                    Ok(prog) => prog,
                    Err(e) => {
                        eprintln!("{e}");
                        process::exit(1);
                    },
                };
                let mut second_parser = IrParser::new(program);
                let bytecode: Bytecode = match second_parser.parse() {
                    Ok(code) => code,
                    Err(e) => {
                        eprintln!("{e}");
                        process::exit(1);
                    },
                };
                if file_matches.get_flag("check") {
                    match type_check_program(&bytecode) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{}", e);
                            process::exit(1);
                        }
                    }
                }
                let mut vm = CHSVM::new(bytecode);
                vm.run(file_matches.get_flag("debug"));
                return Ok(());
            }
            println!("File not provided.");
            println!("Usage: chsvm <file.chs>");
            return Ok(());

        }
        _ => unreachable!()
    }
}