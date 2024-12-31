use std::{env, fs};

use chs_ast::Parser;
use chs_lexer::Lexer;

fn main() {
    let mut args = env::args();
    let _program = args.next().expect("Expect program");
    let file = args.next().expect("Expect file");
    let src = fs::read(&file)
        .expect("Failed to read file");
    let lex = Lexer::new(file.into(), src);
    let module = Parser::new(lex).parse().expect("Parsing falied");
    println!("{}", module)
}
