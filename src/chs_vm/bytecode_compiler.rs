use std::{collections::HashMap, vec::IntoIter};

use crate::{
    chs_error,
    chs_frontend::ast::{
        Expr, FnExpr, IfExpr, LambdaExpr, ListExpr, Operation, PeekExpr, Program, SExpr, WhileExpr,
    },
    utils::CHSError,
};

use super::{
    instructions::{Bytecode, Instr},
    value::Value,
};

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

    pub fn parse(&mut self) -> Result<Bytecode, CHSError> {
        while let Some(expr) = self.program.next() {
            self.expr(expr)?;
        }

        Ok(Bytecode {
            program: self.instrs.clone(),
            consts: self.consts.clone(),
        })
    }

    fn expr(&mut self, expr: Expr) -> Result<(), CHSError> {
        Ok(match expr {
            Expr::If(v) => self.if_expr(*v)?,
            Expr::Whlie(v) => self.while_expr(*v)?,
            Expr::Peek(v) => self.peek_expr(*v)?,
            Expr::Fn(v) => self.fn_expr(*v)?,
            Expr::ListExpr(v) => self.list_expr(*v)?,
            Expr::SExpr(v) => self.s_expr(*v)?,
            Expr::LambdaExpr(v) => self.lambda_expr(*v)?,
            _ => self.simple_expr(expr)?,
        })
    }

    fn lambda_expr(&mut self, expr: LambdaExpr) -> Result<(), CHSError> {
        let ifaddrs = self.instrs.len();
        self.instrs.push(Instr::Jmp(0));
        let init_len = self.instrs.len();
        for e in expr.body {
            self.expr(e)?;
        }
        self.instrs.push(Instr::RetFn);
        let curr_len = self.instrs.len();
        let elem = unsafe { self.instrs.get_unchecked_mut(ifaddrs) };
        *elem = Instr::Jmp((curr_len - ifaddrs) as isize);
        self.instrs.push(Instr::Const(Value::Ptr(init_len)));
        Ok(())
    }

    fn s_expr(&mut self, expr: SExpr) -> Result<(), CHSError> {
        for e in expr.args.into_iter() {
            self.expr(e)?
        }
        self.expr(expr.func)?;
        Ok(())
    }

    fn list_expr(&mut self, expr: ListExpr) -> Result<(), CHSError> {
        let mut list_init = 0;
        for e in expr.itens.into_iter() {
            match e {
                Expr::Op(ref o) => {
                    match **o {
                        Operation::Dup | Operation::Swap => list_init += 2,
                        Operation::Over | Operation::Rot => list_init += 3,
                        Operation::Nop | Operation::Print | Operation::Debug | Operation::Exit => {}
                        _ => list_init += 1,
                    }
                    self.expr(e)?
                }
                Expr::IntExpr(_)
                | Expr::StrExpr(_)
                | Expr::BoolExpr(_)
                | Expr::ListExpr(_)
                | Expr::NilExpr
                | Expr::IdentExpr(_) => {
                    list_init += 1;
                    self.expr(e)?
                }
                _ => chs_error!("Compiler Error: {e} is Not allowed inside list expresions"),
            }
        }
        self.instrs.push(Instr::MakeList(list_init));
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

    fn fn_expr(&mut self, expr: FnExpr) -> Result<(), CHSError> {
        match self.checks_def(&expr.name) {
            NamesDef::Fn => chs_error!("Compiler Error: {} is already Function name.", expr.name),
            NamesDef::Var => chs_error!("Compiler Error: {} is already Variable name.", expr.name),
            NamesDef::None => {}
        };
        let ifaddrs = self.instrs.len();
        self.instrs.push(Instr::Jmp(0));
        let curr_len = self.instrs.len();
        self.fn_def.insert(expr.name, curr_len);
        for e in expr.body.into_iter() {
            self.expr(e)?
        }
        self.instrs.push(Instr::RetFn);
        let curr_len = self.instrs.len();
        let elem = unsafe { self.instrs.get_unchecked_mut(ifaddrs) };
        *elem = Instr::Jmp((curr_len - ifaddrs) as isize);
        Ok(())
    }

    fn peek_expr(&mut self, expr: PeekExpr) -> Result<(), CHSError> {
        let names_len = expr.names.len();
        self.instrs.push(Instr::Bind(names_len));
        for e in expr.names.iter() {
            match self.checks_def(e) {
                NamesDef::Fn => chs_error!("Compiler Error: {} is already Function name.", e),
                NamesDef::Var => chs_error!("Compiler Error: {} is already Variable name.", e),
                NamesDef::None => {}
            };
            self.peek_def.push(e.to_string())
        }
        for e in expr.body.into_iter() {
            self.expr(e)?
        }
        self.instrs.push(Instr::Unbind(names_len));
        for _ in expr.names.iter() {
            self.peek_def.pop();
        }
        Ok(())
    }

    fn while_expr(&mut self, expr: WhileExpr) -> Result<(), CHSError> {
        let whileaddrs = self.instrs.len();
        for e in expr.cond.into_iter() {
            self.expr(e)?
        }
        let ifaddrs = self.instrs.len();
        self.instrs.push(Instr::JmpIf(0));
        for e in expr.while_block.into_iter() {
            self.expr(e)?
        }
        let curr_len = self.instrs.len();
        self.instrs
            .push(Instr::Jmp(-((curr_len - whileaddrs) as isize)));
        let curr_len = self.instrs.len();
        let elem = unsafe { self.instrs.get_unchecked_mut(ifaddrs) };
        *elem = Instr::JmpIf((curr_len - ifaddrs) as isize);
        Ok(())
    }

    fn if_expr(&mut self, expr: IfExpr) -> Result<(), CHSError> {
        let offset = self.instrs.len();
        self.instrs.push(Instr::JmpIf(0));
        for e in expr.if_branch.into_iter() {
            self.expr(e)?
        }
        if let Some(vec) = expr.else_branch {
            let offset2 = self.instrs.len();
            self.instrs.push(Instr::Jmp(0));
            let elem = unsafe { self.instrs.get_unchecked_mut(offset) };
            *elem = Instr::JmpIf((offset2 - (offset) + 1) as isize);
            for e in vec.into_iter() {
                self.expr(e)?
            }
            let curr_len = self.instrs.len();
            let elem = unsafe { self.instrs.get_unchecked_mut(offset2) };
            *elem = Instr::Jmp((curr_len - offset2) as isize);
        } else {
            let curr_len = self.instrs.len();
            let elem = unsafe { self.instrs.get_unchecked_mut(offset) };
            *elem = Instr::JmpIf((curr_len - offset) as isize);
        }
        Ok(())
    }

    fn simple_expr(&mut self, expr: Expr) -> Result<(), CHSError> {
        match expr {
            Expr::IntExpr(v) => {
                let v = match v.parse() {
                    Ok(ok) => ok,
                    Err(e) => chs_error!("{v} {}", e),
                };
                self.instrs.push(Instr::Const(Value::Int64(v)));
            }
            Expr::StrExpr(v) => {
                self.instrs
                    .push(Instr::Const(Value::Str(v.chars().collect())));
            }
            Expr::BoolExpr(v) => {
                if v.as_str() == "true" {
                    self.instrs.push(Instr::Const(Value::Bool(true)));
                } else {
                    self.instrs.push(Instr::Const(Value::Bool(false)));
                }
            }
            Expr::NilExpr => {
                self.instrs.push(Instr::Const(Value::Nil));
            }
            Expr::ErrorExpr(s) => self.instrs.push(Instr::Error(s)),
            Expr::Op(v) => match *v {
                Operation::Pop => self.instrs.push(Instr::Pop),
                Operation::Dup => self.instrs.push(Instr::Dup),
                Operation::Swap => self.instrs.push(Instr::Swap),
                Operation::Over => self.instrs.push(Instr::Over),
                Operation::Add => self.instrs.push(Instr::Add),
                Operation::Minus => self.instrs.push(Instr::Minus),
                Operation::Mul => self.instrs.push(Instr::Mul),
                Operation::Div => self.instrs.push(Instr::Div),
                Operation::Mod => self.instrs.push(Instr::Mod),
                Operation::Eq => self.instrs.push(Instr::Eq),
                Operation::Neq => self.instrs.push(Instr::Neq),
                Operation::Gt => self.instrs.push(Instr::Gt),
                Operation::Gte => self.instrs.push(Instr::Gte),
                Operation::Lte => self.instrs.push(Instr::Lte),
                Operation::Lt => self.instrs.push(Instr::Lt),
                Operation::Land => self.instrs.push(Instr::Land),
                Operation::Lor => self.instrs.push(Instr::Lor),
                Operation::Lnot => self.instrs.push(Instr::Lnot),
                Operation::Shl => self.instrs.push(Instr::Shl),
                Operation::Shr => self.instrs.push(Instr::Shr),
                Operation::Bitand => self.instrs.push(Instr::Bitand),
                Operation::Bitor => self.instrs.push(Instr::Bitor),
                Operation::Debug => self.instrs.push(Instr::Debug),
                Operation::Exit => self.instrs.push(Instr::Exit),
                Operation::Print => self.instrs.push(Instr::Print),
                Operation::IdxSet => self.instrs.push(Instr::IdxSet),
                Operation::IdxGet => self.instrs.push(Instr::IdxGet),
                Operation::Len => self.instrs.push(Instr::Len),
                Operation::Concat => self.instrs.push(Instr::Concat),
                Operation::Tail => self.instrs.push(Instr::Tail),
                Operation::Head => self.instrs.push(Instr::Head),
                Operation::Call => self.instrs.push(Instr::Call),
                Operation::Rot => self.instrs.push(Instr::Rot),
                Operation::Nop => self.instrs.push(Instr::Nop),
                Operation::StackSize => self.instrs.push(Instr::StackSize),
            },
            Expr::IdentExpr(val) => {
                if let Some((v, _)) = self
                    .peek_def
                    .iter()
                    .enumerate()
                    .rev()
                    .find(|(_, s)| s.as_str() == val.as_str())
                {
                    self.instrs.push(Instr::PushBind(v));
                } else if let Some(addrs) = self.fn_def.get(val.as_ref()) {
                    self.instrs.push(Instr::CallFn(*addrs));
                } else if let Some(v) = self.var_def.get(val.as_ref()) {
                    self.instrs.push(Instr::GlobalLoad(*v));
                } else {
                    chs_error!("Compiler Error: {} is not defined", val.to_string())
                }
            }
            Expr::Assigin(val) => {
                if let Some(v) = self.var_def.get(val.as_ref()) {
                    self.instrs.push(Instr::GlobalStore(*v));
                } else {
                    let var_ptr = self.var_count;
                    self.var_count += 1;
                    self.var_def.insert(*val.clone(), var_ptr);
                    self.instrs.push(Instr::GlobalStore(var_ptr));
                }
            }
            e => chs_error!("Compiler Error: {} is not simple expression", e),
        }
        Ok(())
    }
}
