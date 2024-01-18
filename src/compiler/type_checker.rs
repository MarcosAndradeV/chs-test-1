use crate::{type_error, instructions::{Bytecode, Opcode}, exeptions::TypeError, value::Value};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Types {
    Int,
    Str,
    Ptr,
    Bool,
    List,
}

pub fn type_check_program(code: &Bytecode) -> Result<(), TypeError> {
    let mut type_stack: Vec<Types> = Vec::new();
    let mut ip: usize = 0;
    while ip < code.program.len() {
        let instr = code.program[ip];

        match instr.kind {
            Opcode::PushPtr => {
                match instr.operands {
                    Some(_) => {},
                    None => type_error!("OPERAND_NOT_PROVIDED"),
                }
                type_stack.push(Types::Ptr);
            }
            Opcode::Const => {
                let addrs = match instr.operands {
                    Some(v) => v,
                    None => type_error!("OPERAND_NOT_PROVIDED"),
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
            Opcode::Add | Opcode::Minus | Opcode::Mul | Opcode::Div => {
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
            Opcode::Eq | Opcode::Neq => {
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
            Opcode::Println | Opcode::Print => {
                if type_stack.len() < 1 {
                    type_error!("Not enugth operands for {:?}.", instr.kind)
                }
                type_stack.pop();
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
            _ => type_error!("Unimplemented! {:?}", instr.kind)
        }

        ip+=1;
    }
    if type_stack.len() != 0 {
        println!("NOTE: {} data left on the stack. {:?}", type_stack.len(), type_stack)
    }
    Ok(())
}