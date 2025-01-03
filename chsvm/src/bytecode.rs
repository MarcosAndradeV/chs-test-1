
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
    Call(usize),
    Ret,
}
