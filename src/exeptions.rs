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

#[derive(Debug)]
pub struct TypeError {
    pub msg: String
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[macro_export]
macro_rules! type_error {
    ($message: expr, $($field: expr),*) => {
        return Err(TypeError {
            msg: format!($message, $($field),*),
        })
    };

    ($message: expr) => {
        return Err(TypeError {
            msg: $message.to_string(),
        })
    }
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