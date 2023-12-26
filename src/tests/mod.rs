#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{compiler::parser::Parser, runtime::vm::CHSVM};

    use super::*;
    
    #[test]
    fn test_comp() {
        let input = String::from("1 1 +");
        let mut parser = Parser::new(input.into_bytes(), PathBuf::new());
        let ast = parser.parse().unwrap();
        let mut vm = CHSVM::new(ast);
        vm.run();
    }
}
