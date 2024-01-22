use std::collections::HashMap;

use crate::{type_error, instructions::{Builtin, Bytecode, Opcode}, exeptions::TypeError, value::Value};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Types {
    Int = 0,
    Str,
    Ptr,
    Bool,
    List,
    Label
}

pub fn type_check_program(code: &Bytecode) -> Result<(), TypeError> {
    let mut type_stack: Vec<Types> = Vec::new();
    let mut sym_table: HashMap<usize, Types> = HashMap::new();
    let mut snapshot_stack: Vec<Vec<Types>> = Vec::new();
    let mut ip: usize = 0;
    while ip < code.program.len() {
        let instr = code.program[ip];

        match instr.kind {
            Opcode::PushPtr => {
                match instr.operands {
                    Some(_) => {},
                    None => type_error!("Operand not provided for {:?}", instr.kind)
                }
                type_stack.push(Types::Ptr);
            }
            Opcode::Const => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => type_error!("Operand not provided for {:?}", instr.kind)
                };
                let val = match code.consts.get(addrs) {
                    Some(v) => v,
                    None => type_error!("Some"),
                };
                match val {
                    Value::Int64(_) => type_stack.push(Types::Int),
                    Value::Str(_) => type_stack.push(Types::Str),
                    Value::List(_) => type_stack.push(Types::List),
                    _ => type_error!("Unimplemented! {}", val)
                }
            }
            Opcode::Add | Opcode::Minus | Opcode::Mul | Opcode::Div | Opcode::Mod => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let a = type_stack.pop().unwrap();
                let b = type_stack.pop().unwrap();
                match (a, b) {
                    (Types::Int, Types::Int) => type_stack.push(Types::Int),
                    (ta, tb) => type_error!("Cannot add {:?} {:?}", ta, tb),
                }    
            }
            Opcode::Shl | Opcode::Shr | Opcode::Bitand | Opcode::Bitor => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let a = type_stack.pop().unwrap();
                let b = type_stack.pop().unwrap();
                match (a, b) {
                    (Types::Int, Types::Int) => type_stack.push(Types::Int),
                    (ta, tb) => type_error!("Cannot add {:?} {:?}", ta, tb),
                }    
            }
            Opcode::Eq | Opcode::Neq | Opcode::Land | Opcode::Lor => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                type_stack.pop().unwrap();
                type_stack.pop().unwrap();
                type_stack.push(Types::Bool); 
            }
            Opcode::Gt | Opcode::Gte | Opcode::Lte | Opcode::Lt => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let a = type_stack.pop().unwrap();
                let b = type_stack.pop().unwrap();
                match (a, b) {
                    (Types::Int, Types::Int) => type_stack.push(Types::Bool),
                    (ta, tb) => type_error!("Cannot compare {:?} {:?} with {:?}", ta, tb, instr.kind),
                }
            }


            Opcode::Pop => {
                if type_stack.len() < 1 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                type_stack.pop();
            }
            Opcode::Dup => {
                if type_stack.len() < 1 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let a = type_stack.pop().unwrap();
                type_stack.push(a);
                type_stack.push(a);
            }
            Opcode::Dup2 => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let b = type_stack.pop().unwrap();
                let a = type_stack.pop().unwrap();
                type_stack.push(a);
                type_stack.push(b);
                type_stack.push(a);
                type_stack.push(b);
            }
            Opcode::Over => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let b = type_stack.pop().unwrap();
                let a = type_stack.pop().unwrap();
                type_stack.push(a);
                type_stack.push(b);
                type_stack.push(a);
            }
            Opcode::Swap => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let b = type_stack.pop().unwrap();
                let a = type_stack.pop().unwrap();
                type_stack.push(b);
                type_stack.push(a);
            }
            Opcode::PushLabel => {
                match instr.operands {
                    Some(o) => {
                        match o {
                            1 => {
                                snapshot_stack.push(type_stack.clone())
                            },
                            2 => {
                                let temp = snapshot_stack.pop().unwrap();
                                snapshot_stack.push(type_stack.clone());
                                type_stack = temp;
                            }
                            3 => {
                                snapshot_stack.push(type_stack.clone())
                            }
                            4 => {}
                            _ => type_error!("PushLabel {} is not implemented", o)
                        }
                    }
                    None => type_error!("Operand not provided for {:?}", instr.kind)
                }
            }
            Opcode::DropLabel => {
                match instr.operands {
                    Some(o) => {
                        match o {
                            1 => {
                                if !(snapshot_stack.pop().unwrap() == type_stack) {
                                    type_error!("elseless-if is not allowed to mutate the stack.")
                                }
                            }
                            2 => {
                                if type_stack != snapshot_stack.pop().unwrap() {
                                    type_error!("if-else branches must produce same type signature.")
                                }
                            }
                            3 => {
                                if !(snapshot_stack.pop().unwrap() == type_stack) {
                                    type_error!("while is not allowed to mutate the stack.")
                                }
                            }
                            4 => {}
                            _ => type_error!("DropLabel"),
                        }
                    }
                    None => type_error!("DropLabel"),
                }
            }
            Opcode::JmpIf => {
                if type_stack.len() < 1 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let a = type_stack.pop().unwrap();
                if a != Types::Bool {
                    type_error!("JmpIf");
                }
            }
            Opcode::GlobalStore => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let b = type_stack.pop().unwrap();
                let pos = match instr.operands {
                    Some(v) => v,
                    None => type_error!("Operand not provided for {:?}", instr.kind)
                };
                sym_table.insert(pos, b);
                let a = type_stack.pop().unwrap();
                if a != Types::Ptr {
                    type_error!("Store");
                }
            }
            Opcode::InlineStore => {
                if type_stack.len() < 2 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let a = type_stack.pop().unwrap();
                if a != Types::Ptr {
                    type_error!("Store");
                }
                let b = type_stack.pop().unwrap();
                let pos = match instr.operands {
                    Some(v) => v,
                    None => type_error!("Operand not provided for {:?}", instr.kind)
                };
                sym_table.insert(pos, b);
            }
            Opcode::GlobalLoad => {
                if type_stack.len() < 1 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                let a = type_stack.pop().unwrap();
                if a != Types::Ptr {
                    type_error!("Load");
                }
                let pos = match instr.operands {
                    Some(v) => v,
                    None => type_error!("Operand not provided for {:?}", instr.kind)
                };
                let value = match sym_table.get(&pos) {
                    Some(ok) => *ok,
                    None => type_error!("nothig in sym table at {}", pos),
                };
                type_stack.push(value);

            }


            Opcode::Call => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => type_error!("{:?} operand is not provided.", instr.kind)
                };
                if addrs > code.program.len() {
                    type_error!("Address out of bounds.")
                }
            }
            Opcode::Buildin => {
                let typ: usize = match instr.operands {
                    Some(v) => v,
                    None => type_error!("{:?} operand is not provided.", instr.kind)
                };
                let buildin = Builtin::from(typ);
                if buildin.is_invalid() { type_error!("") }
                match buildin {
                    Builtin::IdxGet => {
                        if type_stack.len() < 2 {
                            type_error!("Not enugth operands for {:?}.", instr.kind)
                        }
                        let b = type_stack.pop().unwrap();
                        let a = type_stack.pop().unwrap();
                        if b != Types::Int {
                            type_error!("Index needs to be int type found {:?}", b);
                        }
                        match a {
                            Types::List => type_stack.push(Types::Int),
                            Types::Str => type_stack.push(Types::Str),
                            _ => type_error!("Type {:?} cannot be indexed by {:?}", a, b)
                        }
        
                    }
                    Builtin::IdxSet => {
                        if type_stack.len() < 3 {
                            type_error!("Not enugth operands for {:?}.", instr.kind)
                        }
                        let c = type_stack.pop().unwrap();
                        if c != Types::Int {
                            type_error!("Lists only suport int found {:?}", c);
                        }
                        let b = type_stack.pop().unwrap();
                        if b != Types::Int {
                            type_error!("Index needs to be int type found {:?}", b);
                        }
                        let a = type_stack.pop().unwrap();
                        if a != Types::Ptr {
                            type_error!("IdxSet");
                        }
                        type_stack.push(Types::Ptr);
                        type_stack.push(Types::Int);
        
                    }
                    Builtin::Len => {
                        if type_stack.len() < 1 {
                            type_error!("Not enugth operands for {:?}.", instr.kind)
                        }
                        let a = type_stack.pop().unwrap();
                        if a != Types::List && a != Types::Str {
                            type_error!("Cannot get the length of {:?}", a);
                        }
                        type_stack.push(Types::Int);
                    }
                    Builtin::Println | Builtin::Print => {
                        if type_stack.len() < 1 {
                            type_error!("Not enugth operands for {:?}.", instr.kind)
                        }
                        type_stack.pop();
                    }
                    Builtin::Debug => {}
                    Builtin::Length => todo!(),
                    Builtin::Builtins => todo!(),
                    Builtin::TimeUnix => todo!(),
                    Builtin::Args => todo!(),
                    Builtin::Exit => todo!(),
                    Builtin::TypeOf => todo!(),
                    Builtin::CallFunc => todo!(),
                    Builtin::FStat => todo!(),
                    Builtin::FWrite => todo!(),
                    Builtin::FAppend => todo!(),
                    Builtin::FRead => todo!(),
                    Builtin::ReadLine => todo!(),
                    Builtin::SWrite => todo!(),
                    Builtin::SRead => todo!(),
                    Builtin::GetSyscalls => todo!(),
                    Builtin::Syscall => todo!(),
                    Builtin::Invalid => todo!(),
                }
            }
            Opcode::Ret | Opcode::Jmp | Opcode::Halt => {}
            _ => type_error!("Unimplemented! {:?}", instr.kind)
        }

        ip+=1;
    }
    if type_stack.len() != 0 {
        println!("NOTE: {} data left on the stack. {:?}", type_stack.len(), type_stack)
    }
    Ok(())
}