use instructions::Bytecode;
use value::Value;

pub mod bytecode_compiler;
pub mod instructions;
pub mod value;

pub fn jump(addr: usize, rel: isize) -> usize {
    (addr as isize + rel) as usize
}
pub fn jump_to(addr: usize, other: usize) -> isize {
    other as isize - addr as isize
}

pub fn vm_run(program: Bytecode) {
    let mut stack: Vec<Value> = Vec::with_capacity(1024);
    let mut tstack: Vec<Value> = Vec::with_capacity(1024);
    let mut rstack: Vec<usize> = Vec::with_capacity(1024);
    let mut ip = 0;
    loop {
        if ip >= program.program.len() {break;}
        let mut next_addr = ip + 1;
        match program.program[ip] {
            instructions::Instr::Halt => {
                next_addr = program.program.len();
            }
            instructions::Instr::Const(ref v) => stack.push(v.clone()),
            instructions::Instr::Pop => {
                let _ = stack.pop();
            }
            instructions::Instr::Dup => {
                let a = stack.pop().unwrap();
                stack.push(a.clone());
                stack.push(a);
            }
            instructions::Instr::Swap => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(b);
                stack.push(a);
            }
            instructions::Instr::Over => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a.clone());
                stack.push(b);
                stack.push(a);
            }
            instructions::Instr::Rot => {
                let c = stack.pop().unwrap();
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(b);
                stack.push(c);
                stack.push(a);
            }
            instructions::Instr::Add => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a+b))
            }
            instructions::Instr::Minus => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a-b))
            }
            instructions::Instr::Mul => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a*b))
            }
            instructions::Instr::Div => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a/b))
            }
            instructions::Instr::Mod => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a%b))
            }
            instructions::Instr::Shr => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a>>b))
            }
            instructions::Instr::Shl => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a<<b))
            }
            instructions::Instr::Bitor => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a|b))
            }
            instructions::Instr::Bitand => {
                let b = stack.pop().unwrap().int();
                let a = stack.pop().unwrap().int();
                stack.push(Value::Int64(a&b))
            }
            instructions::Instr::Lor => {
                let b = stack.pop().unwrap().bool();
                let a = stack.pop().unwrap().bool();
                stack.push(Value::Bool(a||b))
            }
            instructions::Instr::Land => {
                let b = stack.pop().unwrap().bool();
                let a = stack.pop().unwrap().bool();
                stack.push(Value::Bool(a&&b))
            }
            instructions::Instr::Lnot => {
                let a = stack.pop().unwrap().bool();
                stack.push(Value::Bool(!a))
            }
            instructions::Instr::Bind(n) => {
                for _ in 0..n {
                    let value = stack.pop().unwrap();
                    tstack.push(value);
                }
                //tstack.extend(stack.split_off((stack.len()-1)-n))
            }
            instructions::Instr::PushBind(n) => {
                stack.push(tstack[(tstack.len()-1) - n].clone())
            }
            instructions::Instr::SetBind(_) => todo!(),
            instructions::Instr::Unbind(n) => {
                tstack.truncate(tstack.len().saturating_sub(n));
            },
            instructions::Instr::Jmp(addr) => {
                next_addr = jump(ip, addr);
            }
            instructions::Instr::JmpIf(addr) => {
                let a = stack.pop().unwrap().bool();
                if !a {
                    next_addr = jump(ip, addr)
                }
            }
            instructions::Instr::Eq => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a==b))
            }
            instructions::Instr::Neq => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a!=b))
            }
            instructions::Instr::Gt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a>b))
            }
            instructions::Instr::Lt => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a<b))
            }
            instructions::Instr::Gte => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a>=b))
            }
            instructions::Instr::Lte => {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(Value::Bool(a<=b))
            }
            instructions::Instr::Nop => {break;}
            instructions::Instr::GlobalStore(addr) => {
                let value = stack.pop().unwrap();
                if addr >= tstack.len() {
                    tstack.push(value);
                } else {
                    tstack[addr] = value
                }
            }
            instructions::Instr::GlobalLoad(addr) => {
                stack.push(tstack[addr].clone());
            }
            instructions::Instr::CallFn(addr) => {
                rstack.push(next_addr);
                next_addr = addr;
            }
            instructions::Instr::RetFn => {
                next_addr = rstack.pop().unwrap();
            }
            instructions::Instr::Debug => {
                println!(
                    "Debug:\nData Stack: {:?}\nTemp Stack: {:?}",
                    stack, tstack
                );
            }
            instructions::Instr::Exit => todo!(),
            instructions::Instr::Print => {
                print!("{}", stack.pop().unwrap());
            }
            instructions::Instr::IdxSet => {
                let new_val = stack.pop().unwrap();
                let idx = stack.pop().unwrap().int();
                let mut arr = stack.pop().unwrap().array();
                arr[idx as usize] = new_val;
                stack.push(Value::Array(arr));
            }
            instructions::Instr::IdxGet => {
                let idx = stack.pop().unwrap().int();
                let mut arr = stack.pop().unwrap().array();
                stack.push(arr.remove(idx as usize))
            }
            instructions::Instr::Len => {
                let value = Value::Int64(stack.pop().unwrap().array().len() as i64);
                stack.push(value)
            }
            instructions::Instr::Concat => {
                let b = stack.pop().unwrap().array();
                let mut a = stack.pop().unwrap().array();
                a.extend(b);
                stack.push(Value::Array(a))
            }
            instructions::Instr::Head => {
                let a = stack.pop().unwrap().array();
                stack.push(a.first().unwrap_or(&Value::Nil).clone())
            }
            instructions::Instr::Tail => {
                let a = stack.pop().unwrap().array()
                    .split_first()
                    .map_or(Value::Nil, |(_, a)| Value::Array(a.to_vec()));
                stack.push(a)
            }
            instructions::Instr::Call => {
                let ptr = stack.pop().unwrap().ptr();
                rstack.push(next_addr);
                next_addr = ptr
            },
            instructions::Instr::MakeList(n) => {
                let  v = stack.split_off(stack.len().saturating_sub(n));
                stack.push(Value::Array(v));
            }
            instructions::Instr::Error(ref err) => {
                panic!("Runtime Error: {err}")
            }
            instructions::Instr::StackSize => {
                stack.push(Value::Int64(stack.len() as i64))
            }
        }
        ip = next_addr
    }
}
