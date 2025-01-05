
#[derive(Debug, Default, Clone, Copy)]
pub enum Instruction {
    #[default]
    Halt,
    PushConst(i64),
    PushLocal(usize),
    SetLocal(usize),
    Drop(usize),
    Add,
    Sub,
    Mult,
    Div,
    Eq,
    NotEq,
    Lt,
    Jmp(usize),
    JmpIf(usize),
    Call(usize),
    Ret,
    Print,
}
