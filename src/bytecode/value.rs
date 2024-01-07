
#[derive(Debug, PartialOrd, Clone)]
pub enum CHSValue {
    Null,
    I(i64),
}

impl PartialEq for CHSValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CHSValue::Null => {
                match other {
                    CHSValue::Null => true,
                    _ => false
                }
            },
            CHSValue::I(v) => {
                match other {
                    CHSValue::I(o) => v == o,
                    _ => false
                }
            },
        }
    }
}