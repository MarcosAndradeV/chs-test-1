#[derive(Debug, PartialEq, Eq)]
pub enum Trap {
    StackOverflow,
    StackUnderflow,
    DivByZero,
    OperandNotProvided,
    AddersOutOfBounds,
    ProgramEndWithoutHalt,
}