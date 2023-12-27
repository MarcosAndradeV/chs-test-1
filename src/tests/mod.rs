#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::{compiler::{parser::Parser, make_ast}, runtime::vm::CHSVM};

    use super::*;
    
    #[test]
    fn test_comp() {
        let source = String::from("proc main { 1 1 + print }");
        let ast = match make_ast(source) {
            Ok(s) => s.stmt,
            Err(e) => return
        };
        let mut vm = CHSVM::new(ast);
        vm.run();

    }
}
