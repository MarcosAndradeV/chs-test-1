use core::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum VMError {
    DivByZero,
    NotImplemeted,
    StackOverflow,
    StackUnderflow,
    AddersOutOfBounds,
    OperandNotProvided,
    TypeIncorrect,
    ProgramEndWithoutHalt,
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VMError::DivByZero => write!(f, "DivByZero"),
            VMError::NotImplemeted => write!(f, "NotImplemeted"),
            VMError::StackOverflow => write!(f, "StackOverflow"),
            VMError::StackUnderflow => write!(f, "StackUnderflow"),
            VMError::AddersOutOfBounds => write!(f, "AddersOutOfBounds"),
            VMError::OperandNotProvided => write!(f, "OperandNotProvided"),
            VMError::ProgramEndWithoutHalt => write!(f, "ProgramEndWithoutHalt"),
            VMError::TypeIncorrect => write!(f, "TypeIncorrect"),
        }
    }
}


#[derive(Debug)]
pub struct GenericError {
    pub msg: String
}

impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GenericError({})", self.msg)
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