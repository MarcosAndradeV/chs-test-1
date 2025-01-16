use std::{collections::HashMap, fs, path::PathBuf};

use chs_lexer::Lexer;
use chs_types::CHSType;
use chs_util::{chs_error, CHSResult};
use nodes::{Expression, Module};
use parser::Parser;

mod nodes;
mod parser;

pub fn parse_file(file_path: String) -> CHSResult<Module> {
    match fs::read(&file_path) {
        Ok(input) => Parser::new(Lexer::new(PathBuf::from(file_path), input)).parse(),
        Err(err) => chs_error!("ERROR: {}", err),
    }
}

struct TypeDB {
    globals: HashMap<String, CHSType>,
    locals: Vec<HashMap<String, CHSType>>,
}
impl TypeDB {
    fn get(&self, s: &str) -> Option<&CHSType> {
        if self.locals.is_empty() {
            return self.globals.get(s);
        } else {
            return self
                .locals
                .last()
                .unwrap()
                .get(s)
                .or_else(|| self.globals.get(s));
        }
    }
}

pub fn check_module(m: &mut Module) -> CHSResult<()> {
    use nodes::Expression::*;
    let mut db = TypeDB {
        globals: m.type_decls.clone(),
        locals: vec![],
    };
    for ele in m.funcs.iter_mut() {
        db.locals.push(HashMap::default());
        let blen = ele.body.len().saturating_sub(1);
        for (i, expr) in ele.body.iter_mut().enumerate() {
            match expr {
                VarDecl(ref mut var_decl) => {
                    if let Some(ref ttype) = var_decl.ttype {
                        check_expr(&mut db, &var_decl.value, ttype)?;
                    } else {
                        let ttype = infer_type(&mut db, &var_decl.value)?;
                        if let Some(scope) = db.locals.last_mut() {
                            scope.insert(var_decl.name.clone(), ttype.clone());
                        }
                        var_decl.ttype = Some(ttype)
                    }
                }
                Assign(ref mut assign) => {
                    todo!()
                }
                _ => {
                    if i == blen {
                        check_expr(&mut db, expr, &ele.ret_type)?;
                    } else {
                        let _ = infer_type(&mut db, expr)?;
                    }
                }
            }
        }
        db.locals.pop();
    }
    Ok(())
}

fn check_expr(db: &mut TypeDB, ele: &Expression, expect_type: &CHSType) -> CHSResult<()> {
    let actual_type = infer_type(db, ele)?;
    if actual_type != *expect_type {
        chs_error!("TODO check_expr")
    }
    Ok(())
}

fn infer_type(db: &mut TypeDB, ele: &Expression) -> CHSResult<CHSType> {
    use nodes::ConstExpression::*;
    use nodes::Expression::*;
    let ttype: CHSType = match ele {
        ConstExpression(Symbol(s)) => match db.get(s) {
            Some(t) => t.clone(),
            None => chs_error!("TODO"),
        },
        ConstExpression(IntegerLiteral(_)) => CHSType::int(),
        ConstExpression(BooleanLiteral(_)) => CHSType::bool(),
        ConstExpression(StringLiteral(_)) => CHSType::char(),
        ConstExpression(Void) => CHSType::void(),
        Call(call) => {
            let func = infer_type(db, &call.caller)?;
            match func {
                CHSType::Func(args, ret) => {
                    if call.args.len() != args.len() {
                        chs_error!("Mismatch Number of arguments")
                    }
                    for (actual, expect_type) in call.args.iter().zip(args.iter()) {
                        check_expr(db, actual, expect_type)?;
                    }
                    return Ok(*ret);
                }
                _ => chs_error!("TODO"),
            }
        }
        Unop(unop) => todo!(),
        Group(expression) => todo!(),
        Binop(binop) => todo!(),
        ExpressionList(vec) => todo!(),
        _ => todo!(),
    };
    Ok(ttype)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_1() {
        let ast = Parser::new(Lexer::new(
            file!().into(),
            r#"
                fn main()
                end
            "#
            .as_bytes()
            .to_vec(),
        ))
        .parse();
        assert!(ast.is_ok());
        assert!(ast.as_ref().unwrap().funcs.len() == 1);
        assert!(ast
            .unwrap()
            .type_decls
            .get("main")
            .is_some_and(|t| *t == CHSType::Func(vec![], CHSType::void().into())));
    }

    #[test]
    fn test_parser_2() {
        let ast = Parser::new(Lexer::new(
            file!().into(),
            r#"
                fasm fn exit(code: int)
                    "mov rax, 60"
                    ";; mov rdi, {code}"
                    "syscall"
                end
            "#
            .as_bytes()
            .to_vec(),
        ))
        .parse();
        assert!(ast.is_ok());
        assert!(ast.as_ref().unwrap().fasm_funcs.len() == 1);
        assert!(ast
            .unwrap()
            .type_decls
            .get("exit")
            .is_some_and(|t| *t == CHSType::Func(vec![CHSType::int()], CHSType::void().into())));
    }

    #[test]
    fn test_type_check_1() {
        let ast = Parser::new(Lexer::new(
            file!().into(),
            r#"
                fn main()
                end
            "#
            .as_bytes()
            .to_vec(),
        ))
        .parse();
        assert!(ast.is_ok());
        let mut ast = ast.unwrap();
        assert!(ast.as_ref().funcs.len() == 1);
        assert!(ast
            .as_ref()
            .type_decls
            .get("main")
            .is_some_and(|t| *t == CHSType::Func(vec![], CHSType::void().into())));
        assert!(check_module(&mut ast).is_ok());
    }

    #[test]
    fn test_type_check_2() {
        let ast = Parser::new(Lexer::new(
            file!().into(),
            r#"
                fn main() -> void
                    1
                end
            "#
            .as_bytes()
            .to_vec(),
        ))
        .parse();
        assert!(ast.is_ok());
        let mut ast = ast.unwrap();
        assert!(ast.as_ref().funcs.len() == 1);
        assert!(ast
            .as_ref()
            .type_decls
            .get("main")
            .is_some_and(|t| *t == CHSType::Func(vec![], CHSType::void().into())));
        assert!(check_module(&mut ast).is_err());
    }
}
