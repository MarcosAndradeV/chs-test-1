use std::{io, process};

use chs::{chs_frontend::{ast::Program, lexer::read_file_to_bytes, mir::MirParser, parser::Parser}, chs_vm::{bytecode_compiler::IrParser, instructions::Bytecode, vm::CHSVM}};
use clap::{Arg, Command, ArgAction};

fn main() -> io::Result<()>{
    // Basic CLI just for tests.
    let cmd = Command::new("chsc")
        .about("CHS compiler and runner.")
        .version("0.0.1")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("Marcos V. Andrade Almeida")
        .subcommand(
            Command::new("run")
                .about("Runs an CHS file.")
                .short_flag('R')
                .arg(Arg::new("filename").value_name("FILE").num_args(1))
                .arg(
                    Arg::new("debug")
                        .long("debug")
                        .short('d')
                        .action(ArgAction::SetTrue)
                        .help("Runs with debug mode"))
        )
        .subcommand(
            Command::new("build")
                .about("Builds an CHS file.")
                .short_flag('B')
                .arg(Arg::new("filename").value_name("FILE").num_args(1))
        )
        .get_matches();

    match cmd.subcommand() {
        Some(("run", file_matches)) => {
            if file_matches.contains_id("filename") {
                let filename = file_matches
                    .get_one::<String>("filename")
                    .expect("contains_id");
                let bytes = read_file_to_bytes(filename.into())?;
                let mut fist_parser = Parser::new(bytes);
                let program: Program = match fist_parser.parse_to_ast() {
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
                let mut vm = CHSVM::new(bytecode);
                vm.run(file_matches.get_flag("debug"));
                return Ok(());
            }
            println!("File not provided.");
            println!("Usage: chsc run <file.chs>");
            return Ok(());

        }
        Some(("build", file_matches)) => {
            if file_matches.contains_id("filename") {
                let filename = file_matches
                    .get_one::<String>("filename")
                    .expect("contains_id");
                let bytes = read_file_to_bytes(filename.into())?;
                let mir_parser = MirParser::new(bytes);
                let mir = match mir_parser.parse_to_mir() {
                    Ok(code) => code,
                    Err(e) => {
                        eprintln!("{e}");
                        process::exit(1);
                    },
                };
                println!("{:#?}", mir);
                return Ok(());
            }
            println!("File not provided.");
            println!("Usage: chsc build <file.chs>");
            return Ok(());
        }
        e => {
            eprintln!("{:?}", e);
            unreachable!()
        }
    }
}