
use chs_ast::lexer::Lexer;
use chs_ast::parser::Parser;
use std::{env, fs, path::Path};

fn main() {
    let mut args = env::args();
    let program_path = args.next().expect("program_path");

    if let Some(cmd) = args.next() {
        match cmd.as_str() {
            "lex" => {
                if let Some(filepath) = args.next() {
                    let fpath = Path::new(&filepath);
                    if !fpath.exists() {
                        println!("File not found.");
                    }
                    match fs::read(fpath) {
                        Ok(ok) => {
                            let mut lex = Lexer::new(filepath, ok);
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
            "parse" => {
                if let Some(filepath) = args.next() {
                    let fpath = Path::new(&filepath);
                    if !fpath.exists() {
                        println!("File not found.");
                    }
                    match fs::read(fpath) {
                        Ok(ok) => {
                            let lex = Lexer::new(filepath, ok);
                            let mut parser = Parser::new(lex);
                            let ast = parser.parse();
                            match ast {
                                Ok(ast) => {
                                    println!("{ast}");
                                }
                                Err(err) => println!("{err}")
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
            "ast" => {
                if let Some(filepath) = args.next() {
                    let fpath = Path::new(&filepath);
                    if !fpath.exists() {
                        println!("File not found.");
                    }
                    match fs::read(fpath) {
                        Ok(ok) => {
                            let lex = Lexer::new(filepath, ok);
                            let mut parser = Parser::new(lex);
                            let ast = parser.parse();
                            match ast {
                                Ok(ast) => {
                                    println!("{ast:#?}");
                                }
                                Err(err) => println!("{err}")
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
    println!("  ast                    Dump ast from a file.");
    println!("  parse                  Parse and print ast from a file.");
    println!("  help                   Show this message.");
}
