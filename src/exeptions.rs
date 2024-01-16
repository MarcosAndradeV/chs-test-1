use core::fmt;

#[derive(Debug)]
pub struct VMError {
    pub msg: String
}

impl fmt::Display for VMError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[macro_export]
macro_rules! vm_error {
    ($message: expr, $($field: expr),*) => {
        return Err(VMError {
            msg: format!($message, $($field),*),
        })
    };

    ($message: expr) => {
        return Err(VMError {
            msg: $message.to_string(),
        })
    }
}

pub const DIV_BY_ZERO: &str = "DivByZero";
pub const NOT_IMPLEMETED: &str ="NotImplemeted";
pub const STACK_OVERFLOW: &str = "StackOverflow";
pub const STACK_UNDERFLOW: &str ="StackUnderflow";
pub const ADDERS_OUT_OF_BOUNDS: &str ="AddersOutOfBounds";
pub const OPERAND_NOT_PROVIDED: &str = "OperandNotProvided";
pub const PROGRAM_END_WITHOUT_HALT: &str ="ProgramEndWithoutHalt";
pub const TYPE_INCORRECT: &str = "TypeIncorrect";



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