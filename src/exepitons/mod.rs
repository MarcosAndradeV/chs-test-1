use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Trap {
    StackOverflow,
    StackUnderflow,
    DivByZero,
    OperandNotProvided,
    OperandTypeNotCorrect,
    AddersOutOfBounds,
    ProgramEndWithoutHalt,
}

#[derive(Debug)]
pub struct GenericError {
    pub msg: String
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[macro_export]
macro_rules! generic_error {
    ($message: expr, $($field: expr),*) => {
        return Err(GenericError {
            msg: format!($message, $($field),*),
        })
    };

    ($message: expr) => {
        return Err(GenericError {
            msg: $message.to_string(),
        })
    }
}