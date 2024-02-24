use std::{collections::HashMap, vec::IntoIter};

use crate::{chs_frontend::ast::{Expr, FnExpr, IfExpr, ListExpr, Operation, PeekExpr, Program, VarExpr, WhileExpr}, exeptions::GenericError, generic_error};

use super::{instructions::{Bytecode, Instr, Opcode}, value::Value};


#[derive(Debug, PartialEq, Eq)]
enum NamesDef {
    Fn,
    Var,
    None,
}

pub struct IrParser {
    program: IntoIter<Expr>,
    instrs: Vec<Instr>,
    consts: Vec<Value>,
    var_def: HashMap<String, usize>,
    var_count: usize,
    peek_def: Vec<String>,
    fn_def: HashMap<String, usize>,
}

impl IrParser {
    pub fn new(program: Program) -> Self {
        Self {
            program: program.into_iter(),
            instrs: Vec::new(),
            consts: Vec::new(),
            var_def: HashMap::new(),
            var_count: 0,
            peek_def: Vec::new(),
            fn_def: HashMap::new(),
        }
    }

    pub fn parse(&mut self) -> Result<Bytecode, GenericError> {
        while let Some(expr) = self.program.next() {
            self.expr(expr)?;
        }

        if let Some((_, entry)) = self.fn_def.iter().find(|(nm, _)| *nm == "main") {            
            Ok(Bytecode {
                program: self.instrs.clone(),
                consts: self.consts.clone(),
                entry: *entry
            })
        } else {
          generic_error!("main entry point not provided!")  
        }

    }

    fn expr(&mut self, expr: Expr) -> Result<(), GenericError> {
        Ok(match expr {
            Expr::If(v) => self.if_expr(*v)?,
            Expr::Whlie(v) => self.while_expr(*v)?,
            Expr::Var(v) => self.var_expr(*v)?,
            Expr::Peek(v) => self.peek_expr(*v)?,
            Expr::Fn(v) => self.fn_expr(*v)?,
            Expr::ListExpr(v) => self.list_expr(*v)?,
            _ => self.simple_expr(expr)?,
        })
    }

    fn list_expr(&mut self, expr: ListExpr) -> Result<(), GenericError> {
        let list_init = self.instrs.len();
        for e in expr.itens.into_iter() {
            self.expr(e)?
        }
        self.instrs.push(Instr::new(Opcode::MakeList, Some(self.instrs.len().saturating_sub(list_init))));
        Ok(())
    }

    fn checks_def(&self, name: &str) -> NamesDef {
        if self.var_def.get(name).is_some() {
            return NamesDef::Var;
        } else if self.fn_def.get(name).is_some() {
            return NamesDef::Fn;
        }
        NamesDef::None
    }

    fn fn_expr(&mut self, expr: FnExpr) -> Result<(), GenericError> {
        match self.checks_def(&expr.name) {
            NamesDef::Fn   => generic_error!("{} is already Function name.", expr.name),
            NamesDef::Var  => generic_error!("{} is already Variable name.", expr.name),
            NamesDef::None => {}
        };
        let curr_len = self.instrs.len();
        let ismain = expr.name.as_str() == "main";
        self.fn_def.insert(expr.name, curr_len);
        for e in expr.body.into_iter() {
            self.expr(e)?
        }
        if ismain {
            return Ok(())
        }
        self.instrs.push(Instr::new(Opcode::RetFn, None));
        Ok(())
    }

    fn peek_expr(&mut self, expr: PeekExpr) -> Result<(), GenericError> {
        let names_len = expr.names.len();
        self.instrs.push(Instr::new(Opcode::Bind, Some(names_len)));
        for e in expr.names.iter().rev() {
            match self.checks_def(e) {
                NamesDef::Fn   => generic_error!("{} is already Function name.", e),
                NamesDef::Var  => generic_error!("{} is already Variable name.", e),
                NamesDef::None => {}
            };
            self.peek_def.push(e.to_string())
        }
        for e in expr.body.into_iter() {
            self.expr(e)?
        }
        self.instrs
            .push(Instr::new(Opcode::Unbind, Some(names_len)));
        for _ in expr.names.iter() {
            self.peek_def.pop();
        }
        Ok(())
    }

    fn var_expr(&mut self, expr: VarExpr) -> Result<(), GenericError> {
        let var_ptr = self.var_count;
        self.var_count += 1;
        match self.checks_def(&expr.name) {
            NamesDef::Fn   => generic_error!("{} is already Function name.", expr.name),
            NamesDef::Var  => {}
            NamesDef::None => {}
        };
        self.var_def.insert(expr.name.clone(), var_ptr);
        for e in expr.value.into_iter() {
            self.expr(e)?;
        }
        self.instrs
            .push(Instr::new(Opcode::GlobalStore, Some(var_ptr)));
        Ok(())
    }

    fn while_expr(&mut self, expr: WhileExpr) -> Result<(), GenericError> {
        let whileaddrs = self.instrs.len();
        for e in expr.cond.into_iter() {
            self.expr(e)?
        }
        let ifaddrs = self.instrs.len();
        self.instrs.push(Instr::new(Opcode::JmpIf, None));
        for e in expr.while_block.into_iter() {
            self.expr(e)?
        }
        self.instrs.push(Instr::new(Opcode::Jmp, Some(whileaddrs)));
        let curr_len = self.instrs.len();
        let elem = unsafe { self.instrs.get_unchecked_mut(ifaddrs) };
        *elem = Instr::new(Opcode::JmpIf, Some(curr_len));
        Ok(())
    }

    fn if_expr(&mut self, expr: IfExpr) -> Result<(), GenericError> {
        for e in expr.cond.into_iter() {
            self.expr(e)?
        }
        let offset = self.instrs.len();
        self.instrs.push(Instr::new(Opcode::JmpIf, None));
        for e in expr.if_branch.into_iter() {
            self.expr(e)?
        }
        if let Some(vec) = expr.else_branch {
            let offset2 = self.instrs.len();
            self.instrs.push(Instr::new(Opcode::Jmp, None));
            let elem = unsafe { self.instrs.get_unchecked_mut(offset) };
            *elem = Instr::new(Opcode::JmpIf, Some(offset2 + 1));
            for e in vec.into_iter() {
                self.expr(e)?
            }
            let curr_len = self.instrs.len();
            let elem = unsafe { self.instrs.get_unchecked_mut(offset2) };
            *elem = Instr::new(Opcode::Jmp, Some(curr_len));
        } else {
            let curr_len = self.instrs.len();
            let elem = unsafe { self.instrs.get_unchecked_mut(offset) };
            *elem = Instr::new(Opcode::JmpIf, Some(curr_len));
        }
        Ok(())
    }

    fn simple_expr(&mut self, expr: Expr) -> Result<(), GenericError> {
        match expr {
            Expr::IntExpr(v) => {
                let v = match v.parse() {
                    Ok(ok) => ok,
                    Err(e) => generic_error!("{v} {}", e),
                };
                self.consts.push(Value::Int64(v));
                self.instrs
                    .push(Instr::new(Opcode::Const, Some(self.consts.len() - 1)));
            }
            Expr::StrExpr(v) => {
                self.consts.push(Value::Str(v.chars().collect()));
                self.instrs
                    .push(Instr::new(Opcode::Const, Some(self.consts.len() - 1)));
            }
            Expr::BoolExpr(v) => {
                if v.as_str() == "true" {
                    self.consts.push(Value::Bool(true));
                } else {
                    self.consts.push(Value::Bool(false));
                }
                self.instrs
                    .push(Instr::new(Opcode::Const, Some(self.consts.len() - 1)));
            }
            Expr::NilExpr => {
                self.consts.push(Value::Nil);
                self.instrs
                    .push(Instr::new(Opcode::Const, Some(self.consts.len() - 1)));
            }
            // Expr::ListExpr(v) => {
            //     let mut list = vec![];
            //     for s in *v {
            //         match s.parse::<i64>() {
            //             Ok(o) => list.push(Value::Int64(o)),
            //             Err(e) => generic_error!("{}", e),
            //         }
            //     }
            //     self.consts.push(Value::Array(list));
            //     self.instrs
            //         .push(Instr::new(Opcode::Const, Some(self.consts.len() - 1)));
            // }
            Expr::Op(v) => {
                match *v {
                    Operation::Pop    => self.instrs.push(Instr::new(Opcode::Pop, None)),
                    Operation::Dup    => self.instrs.push(Instr::new(Opcode::Dup, None)),
                    Operation::Swap   => self.instrs.push(Instr::new(Opcode::Swap, None)),
                    Operation::Over   => self.instrs.push(Instr::new(Opcode::Over, None)),
                    Operation::Add    => self.instrs.push(Instr::new(Opcode::Add, None)),
                    Operation::Minus  => self.instrs.push(Instr::new(Opcode::Minus, None)),
                    Operation::Mul    => self.instrs.push(Instr::new(Opcode::Mul, None)),
                    Operation::Div    => self.instrs.push(Instr::new(Opcode::Div, None)),
                    Operation::Mod    => self.instrs.push(Instr::new(Opcode::Mod, None)),
                    Operation::Eq     => self.instrs.push(Instr::new(Opcode::Eq, None)),
                    Operation::Neq    => self.instrs.push(Instr::new(Opcode::Neq, None)),
                    Operation::Gt     => self.instrs.push(Instr::new(Opcode::Gt, None)),
                    Operation::Gte    => self.instrs.push(Instr::new(Opcode::Gte, None)),
                    Operation::Lte    => self.instrs.push(Instr::new(Opcode::Lte, None)),
                    Operation::Lt     => self.instrs.push(Instr::new(Opcode::Lt, None)),
                    Operation::Land   => self.instrs.push(Instr::new(Opcode::Land, None)),
                    Operation::Lor    => self.instrs.push(Instr::new(Opcode::Lor, None)),
                    Operation::Shl    => self.instrs.push(Instr::new(Opcode::Shl, None)),
                    Operation::Shr    => self.instrs.push(Instr::new(Opcode::Shr, None)),
                    Operation::Bitand => self.instrs.push(Instr::new(Opcode::Bitand, None)),
                    Operation::Bitor  => self.instrs.push(Instr::new(Opcode::Bitor, None)),
                    Operation::Debug  => self.instrs.push(Instr::new(Opcode::Debug, None)),
                    Operation::Exit   => self.instrs.push(Instr::new(Opcode::Exit, None)),
                    Operation::Print  => self.instrs.push(Instr::new(Opcode::Print, None)),
                    Operation::IdxSet => self.instrs.push(Instr::new(Opcode::IdxSet, None)),
                    Operation::IdxGet => self.instrs.push(Instr::new(Opcode::IdxGet, None)),
                    Operation::Len    => self.instrs.push(Instr::new(Opcode::Len, None)),
                    Operation::Concat => self.instrs.push(Instr::new(Opcode::Concat, None)),
                    Operation::Tail   => self.instrs.push(Instr::new(Opcode::Tail, None)),
                    Operation::Head   => self.instrs.push(Instr::new(Opcode::Head, None)),
                    Operation::Call   => self.instrs.push(Instr::new(Opcode::Call, None)),
                }
            }
            Expr::IdentExpr(val) => {
                if let Some((v, _)) = self
                    .peek_def
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, s)| s.as_str() == val.as_str())
                {
                    self.instrs.push(Instr::new(Opcode::PushBind, Some(v)));
                } else if let Some(addrs) = self.fn_def.get(val.as_ref())
                {
                    self.instrs.push(Instr::new(Opcode::CallFn, Some(*addrs)));
                } else if let Some(v) = self.var_def.get(val.as_ref()) {
                    self.instrs.push(Instr::new(Opcode::GlobalLoad, Some(*v)));
                } else {
                    generic_error!("{} is not defined", val.to_string())
                }
            }
            Expr::Assigin(val) => {
                if let Some(v) = self.var_def.get(val.as_ref()) {
                    self.instrs.push(Instr::new(Opcode::GlobalStore, Some(*v)));
                } else {
                    generic_error!(
                        "Cannot assigin a variable that's not existes {}",
                        val.as_ref()
                    )
                }
            }
            e => generic_error!("{} is not simple expression", e),
        }
        Ok(())
    }
}
