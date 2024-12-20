
use chs_lexer::Lexer;
use std::{env, fs, path::PathBuf};

fn main() {
    let mut args = env::args();
    let program_path = args.next().expect("program_path");

    if let Some(cmd) = args.next() {
        match cmd.as_str() {
            "lex" => {
                if let Some(filepath) = args.next() {
                    let fpath = PathBuf::from(filepath);
                    if !fpath.exists() {
                        println!("File not found.");
                    }
                    match fs::read(fpath.as_path()) {
                        Ok(ok) => {
                            let mut lex = Lexer::new(fpath, ok);
                            loop {
                                let token = lex.next_token();
                                if token.kind.is_eof() {
                                    break;
                                }
                                println!("{token}")
                            }
                        }
                        Err(err) => {
                            eprintln!("Cannot read file {err}")
                        }
                    }
                } else {
                    println!("No file provided.");
                }
            }
            "version" => {
                println!("Version: 0.0.1");
            }
            "help" => {
                usage(&program_path);
            }
            _ => {
                println!("Unknow command.");
                usage(&program_path);
            }
        }
    } else {
        println!("No command provided.");
        usage(&program_path);
    }
}

fn usage(program_path: &String) {
    println!("Usage {program_path} <command> [options] file...");
    println!("Command:");
    println!("  version                Display compiler version information.");
    println!("  lex                    Dump tokens from a file.");
    println!("  help                   Show this message.");
}
