#![allow(unused)]

use std::{env, process::exit};

fn main() {
    let mut argv = env::args();
    let program = argv.next().expect("Program always provided.");
    let cmd = argv.next().unwrap_or_else(|| usage_err(&program, "Expect Command"));
    match cmd.as_str() {
        "help" => usage(&program),
        "com"|"compile" => {}
        "parse" => {
            let file_path = argv.next().unwrap_or_else(|| msg_err("ERROR: File not provided."));
            let ast = chs_ast::parse_file(file_path).unwrap_or_else(|err| msg_err(err));
            println!("{ast}");
        }
        c => usage_err(&program, format!("Invalid Command \"{c}\""))
    }
}

fn usage(program: &str) {
    println!("USAGE: {program} <COMMAND> [OPTIONS]");
    println!("COMMANDS:");
    println!("    help - Show this message.");
    println!("    com|compile - Compile a program: chs com <file.chs>");
    println!("    parse - Parse a program and print its AST: chs parse <file.chs>");
}

fn usage_err(program: &str, err: impl ToString) -> ! {
    eprintln!("ERROR: {}", err.to_string());
    usage(&program);
    exit(1);
}

fn msg_err(err: impl ToString) -> ! {
    eprintln!("{}", err.to_string());
    exit(1);
}
