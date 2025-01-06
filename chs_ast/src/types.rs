use core::fmt;
use std::collections::{BTreeMap, HashMap};

use chs_util::{chs_error, CHSError, CHSResult};

use crate::nodes::{ConstExpression, Decl, Expression, Module};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primitive {
    Void,
    Int,
    Bool,
    Char,
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Void => write!(f, "void"),
            Primitive::Int => write!(f, "int"),
            Primitive::Bool => write!(f, "bool"),
            Primitive::Char => write!(f, "char"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CHSType {
    Custom(String),
    Primitive(Primitive),
    Pointer(Box<CHSType>),
    Func(Vec<CHSType>, Box<CHSType>),
    Record(BTreeMap<String, CHSType>),
}

impl CHSType {
    pub fn custom(s: String) -> Self {
        Self::Custom(s)
    }
    pub fn void() -> Self {
        Self::Primitive(Primitive::Void)
    }
    pub fn int() -> Self {
        Self::Primitive(Primitive::Int)
    }
    pub fn bool() -> Self {
        Self::Primitive(Primitive::Bool)
    }
    pub fn char() -> Self {
        Self::Primitive(Primitive::Char)
    }
    pub fn ptr(t: CHSType) -> Self {
        Self::Pointer(t.into())
    }

    #[must_use]
    pub fn is_void(&self) -> bool {
        matches!(self, Self::Primitive(Primitive::Void))
    }
}

impl fmt::Display for CHSType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CHSType::Custom(n) => write!(f, "{n}"),
            CHSType::Primitive(n) => write!(f, "{n}"),
            CHSType::Pointer(a) => write!(f, "*{a}"),
            CHSType::Func(args, ret) => {
                write!(f, "fn(")?;
                for (i, item) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", item)?;
                }
                write!(f, ") -> {}", ret)
            }
            CHSType::Record(args) => {
                write!(f, "record (")?;
                for (i, item) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, "\n")?;
                    }
                    write!(f, "{}: {}", item.0, item.1)?;
                }
                write!(f, "\n)",)
            }
        }
    }
}

#[derive(Default)]
pub struct TypeCheckEnv {
    pub type_defs: HashMap<String, CHSType>,
    pub var_defs: HashMap<String, CHSType>,
}

pub fn type_check(m: &Module, env: &mut TypeCheckEnv) -> Result<(), CHSError> {
    for ele in &m.decls {
        match ele {
            Decl::Function(function) => {
                let ts = function.args.iter().map(|(_, t)| t.clone()).collect();
                let v = CHSType::Func(ts, function.ret_type.clone().into());
                env.type_defs.insert(function.name.clone(), v);
            }
            _ => todo!(),
        }
    }

    for ele in &m.decls {
        if let Decl::Function(function) = ele {
            env.var_defs.extend(function.args.clone().into_iter());
            for ele in &function.body {
                let ret_t = infer(env, ele)?;
                if ret_t != function.ret_type {
                    chs_error!("MISMATCH RETURN TYPE")
                }
            }
            env.var_defs.clear();
        }
    }

    Ok(())
}

fn infer(env: &mut TypeCheckEnv, ele: &Expression) -> CHSResult<CHSType> {
    match ele {
        Expression::ConstExpression(const_expression) => match const_expression {
            ConstExpression::Symbol(a) => {
                if let Some(t) = env.var_defs.get(a) {
                    return Ok(t.clone());
                }
                if let Some(t) = env.type_defs.get(a) {
                    return Ok(t.clone());
                }
                chs_error!("UNKNOW VARIABLE")
            }
            ConstExpression::IntegerLiteral(_) => Ok(CHSType::int()),
            ConstExpression::BooleanLiteral(_) => Ok(CHSType::bool()),
            ConstExpression::StringLiteral(_) => Ok(CHSType::ptr(CHSType::char())),
            ConstExpression::Void => Ok(CHSType::void()),
        },
        // Expression::ExpressionList(vec) => todo!(),
        Expression::VarDecl(var_decl) => {
            let k = &var_decl.name;
            let v = infer(env, &var_decl.value)?;
            env.var_defs.insert(k.clone(), v);
            Ok(CHSType::void())
        }
        Expression::Call(call) => {
            if let CHSType::Func(args, ret_t) = infer(env, &call.caller)? {
                if args.len() != call.args.len() {
                    chs_error!("")
                }
                for (expected, actual) in args.iter().zip(call.args.iter()) {
                    if *expected != infer(env, actual)? {
                        chs_error!("")
                    }
                }
                Ok(*ret_t)
            } else {
                chs_error!("")
            }
        }
        Expression::Binop(binop) => infer(env, &binop.left).and_then(|l| {
            infer(env, &binop.right).and_then(|r| {
                if l != r {
                    chs_error!("")
                } else {
                    if binop.op.is_logical() {
                        Ok(CHSType::bool())
                    } else {
                        Ok(CHSType::int())
                    }
                }
            })
        }),
        Expression::Group(expression) => infer(env, expression),
        Expression::Ref(expression) => Ok(CHSType::ptr(infer(env, expression)?)),
        Expression::Deref(expression) => {
            if let CHSType::Pointer(t) = infer(env, &expression)? {
                Ok(*t)
            } else {
                chs_error!("")
            }
        }
    }
}
