#![allow(unused)]

use std::{env, process::exit};

fn main() {
    let mut argv = env::args();
    let program = argv.next().expect("Program always provided.");
    let cmd = argv.next().unwrap_or_else(|| usage_err(&program, "Expect Command"));
    match cmd.as_str() {
        "help" => usage(&program),
        "com"|"compile" => {}
        c => usage_err(&program, format!("Invalid Command \"{c}\""))
    }
}

fn usage(program: &str) {
    println!("USAGE: {program} <COMMAND> [OPTIONS]");
    println!("COMMANDS:");
    println!("    help - Show this message.");
    println!("    com|compile - Compile a program: chs com <file.chs>");
}

fn usage_err(program: &str, err: impl ToString) -> ! {
    eprintln!("ERROR: {}", err.to_string());
    usage(&program);
    exit(1);
}
