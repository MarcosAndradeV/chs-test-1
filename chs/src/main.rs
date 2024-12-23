use chs_ast::{types::{infer, InferEnv}, Parser};
use chs_lexer::Lexer;
use std::{
    env::{self, Args},
    fs,
    path::PathBuf,
    process::exit,
};

fn main() {
    let mut args = env::args();
    let program_path = args.next().expect("program_path");

    if let Some(cmd) = args.next() {
        match cmd.as_str() {
            "lex" => {
                let (fpath, bytes) = get_file(&mut args);
                let mut lex = Lexer::new(fpath, bytes);
                loop {
                    let token = lex.next_token();
                    if token.kind.is_eof() {
                        break;
                    }
                    println!("{token}")
                }
            }
            "parse" => {
                let (fpath, bytes) = get_file(&mut args);
                let lex = Lexer::new(fpath, bytes);
                let parser = Parser::new(lex);
                match parser.parse() {
                    Ok(ok) => {
                        println!("{:?}", ok);
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        exit(1)
                    }
                }
            }
            "eval" => {
                let (fpath, bytes) = get_file(&mut args);
                let lex = Lexer::new(fpath, bytes);
                let parser = Parser::new(lex);
                match parser.parse() {
                    Ok(ok) => {
                        let mut env = InferEnv::default();
                        for expr in &ok.top_level {
                            match infer(&mut env, expr, 0) {
                                Err(err) => {
                                    eprintln!("{err}");
                                    exit(1)
                                }
                                _ => ()
                            }
                        }
                    }
                    Err(err) => {
                        eprintln!("{err}");
                        exit(1)
                    }
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
    println!("  parse                  Dump AST from a file.");
    println!("  eval                   Evaluate a file.");
    println!("  help                   Show this message.");
}

fn get_file(args: &mut Args) -> (PathBuf, Vec<u8>) {
    if let Some(filepath) = args.next() {
        let fpath = PathBuf::from(filepath);
        if !fpath.exists() {
            println!("File not found.");
            exit(1)
        }
        match fs::read(fpath.as_path()) {
            Ok(ok) => (fpath, ok),
            Err(err) => {
                eprintln!("Cannot read file {err}");
                exit(1)
            }
        }
    } else {
        println!("No file provided.");
        exit(1)
    }
}
