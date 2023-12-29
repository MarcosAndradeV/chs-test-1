use core::{fmt, ops::{Div, Mul, Sub, Add}};
use std::{str::FromStr, ops::Rem};
use serde::{Deserialize, Serialize};
use crate::exepitons::Trap;

#[derive(Debug, PartialOrd, Clone, Copy)]
#[derive(Serialize, Deserialize)]
pub enum CHSValue {
    I(i64),
    U(u64),
    F(f64),
    P(usize),
    B(u8),
    None,
}

impl CHSValue {
    pub fn is_zero(&self) -> bool {
        match self {
            CHSValue::I(v) => *v == 0,
            CHSValue::U(v) => *v == 0,
            CHSValue::F(v) => *v == 0.0,
            CHSValue::P(v) => *v == 0,
            CHSValue::B(v) => *v == 0,
            CHSValue::None => false,
        }
    }

    pub fn as_i64(self) -> i64 {
        match self {
            CHSValue::P(v) => v as i64,
            CHSValue::I(v) => v as i64,
            CHSValue::U(v) => v as i64,
            CHSValue::F(v) => v as i64,
            CHSValue::B(v) => v  as i64,
            CHSValue::None => 0
        }
    }

    pub fn as_bool(self) -> bool {
        match self {
            CHSValue::P(v) => v >= 1usize,
            CHSValue::I(v) => v >= 1i64,
            CHSValue::U(v) => v >= 1u64,
            CHSValue::F(v) => v >= 1.0,
            CHSValue::B(v) => v >= 1u8,
            CHSValue::None => false
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            CHSValue::P(v) => v as u8,
            CHSValue::I(v) => v as u8,
            CHSValue::U(v) => v as u8,
            CHSValue::F(v) => v as u8,
            CHSValue::B(v) => v as u8,
            CHSValue::None => 0
        }
    }

    pub fn as_usize(self) -> usize {
        match self {
            CHSValue::P(v) => v,
            CHSValue::I(v) => v as usize,
            CHSValue::U(v) => v as usize,
            CHSValue::F(v) => v as usize,
            CHSValue::B(v) => v as usize,
            CHSValue::None => 0
        }
    }

    pub fn none() -> Self {
        Self::None
    }
}

impl PartialEq for CHSValue {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CHSValue::I(v) => {
                match other {
                    CHSValue::I(o) => v == o,
                    _ => false
                }
            },
            CHSValue::U(v) => {
                match other {
                    CHSValue::U(o) => v == o,
                    _ => false
                }
            },
            CHSValue::F(v) => {
                match other {
                    CHSValue::F(o) => v == o,
                    _ => false
                }
            },
            CHSValue::P(v) => {
                match other {
                    CHSValue::P(o) => v == o,
                    _ => false
                }
            },
            CHSValue::B(v) => {
                match other {
                    CHSValue::B(o) => v == o,
                    _ => false
                }
            },
            CHSValue::None => false
        }
    }
}

impl Eq for CHSValue {}

impl Add for CHSValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            CHSValue::I(v) => {
                match rhs {
                    CHSValue::I(o) => CHSValue::I(v + o),
                    _ => unreachable!()
                }
            },
            CHSValue::U(v) => {
                match rhs {
                    CHSValue::U(o) => CHSValue::U(v + o),
                    _ => unreachable!()
                }
            },
            CHSValue::F(v) => {
                match rhs {
                    CHSValue::F(o) => CHSValue::F(v + o),
                    _ => unreachable!()
                }
            },
            CHSValue::P(v) => {
                match rhs {
                    CHSValue::P(o) => CHSValue::P(v + o),
                    _ => unreachable!()
                }
            },
            CHSValue::B(v) => {
                match rhs {
                    CHSValue::B(o) => CHSValue::B(v + o),
                    _ => unreachable!()
                }
            },
            CHSValue::None => CHSValue::None
        }
    }
}

impl Sub for CHSValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            CHSValue::I(v) => {
                match rhs {
                    CHSValue::I(o) => CHSValue::I(v - o),
                    _ => unreachable!()
                }
            },
            CHSValue::U(v) => {
                match rhs {
                    CHSValue::U(o) => CHSValue::U(v - o),
                    _ => unreachable!()
                }
            },
            CHSValue::F(v) => {
                match rhs {
                    CHSValue::F(o) => CHSValue::F(v - o),
                    _ => unreachable!()
                }
            },
            CHSValue::P(v) => {
                match rhs {
                    CHSValue::P(o) => CHSValue::P(v - o),
                    _ => unreachable!()
                }
            },
            CHSValue::B(v) => {
                match rhs {
                    CHSValue::B(o) => CHSValue::B(v - o),
                    _ => unreachable!()
                }
            },
            CHSValue::None => CHSValue::None
        }
    }
}

impl Mul for CHSValue {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            CHSValue::I(v) => {
                match rhs {
                    CHSValue::I(o) => CHSValue::I(v * o),
                    _ => unreachable!()
                }
            },
            CHSValue::U(v) => {
                match rhs {
                    CHSValue::U(o) => CHSValue::U(v * o),
                    _ => unreachable!()
                }
            },
            CHSValue::F(v) => {
                match rhs {
                    CHSValue::F(o) => CHSValue::F(v * o),
                    _ => unreachable!()
                }
            },
            CHSValue::P(v) => {
                match rhs {
                    CHSValue::P(o) => CHSValue::P(v * o),
                    _ => unreachable!()
                }
            },
            CHSValue::B(v) => {
                match rhs {
                    CHSValue::B(o) => CHSValue::B(v * o),
                    _ => unreachable!()
                }
            },
            CHSValue::None => CHSValue::None
        }
    }
}

impl Div for CHSValue {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            CHSValue::I(v) => {
                match rhs {
                    CHSValue::I(o) => CHSValue::I(v / o),
                    _ => unreachable!()
                }
            },
            CHSValue::U(v) => {
                match rhs {
                    CHSValue::U(o) => CHSValue::U(v / o),
                    _ => unreachable!()
                }
            },
            CHSValue::F(v) => {
                match rhs {
                    CHSValue::F(o) => CHSValue::F(v / o),
                    _ => unreachable!()
                }
            },
            CHSValue::P(v) => {
                match rhs {
                    CHSValue::P(o) => CHSValue::P(v / o),
                    _ => unreachable!()
                }
            },
            CHSValue::B(v) => {
                match rhs {
                    CHSValue::B(o) => CHSValue::B(v / o),
                    _ => unreachable!()
                }
            },
            CHSValue::None => CHSValue::None
        }
    }
}

impl Rem for CHSValue {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            CHSValue::I(v) => {
                match rhs {
                    CHSValue::I(o) => CHSValue::I(v % o),
                    _ => unreachable!()
                }
            },
            CHSValue::U(v) => {
                match rhs {
                    CHSValue::U(o) => CHSValue::U(v % o),
                    _ => unreachable!()
                }
            },
            CHSValue::F(v) => {
                match rhs {
                    CHSValue::F(o) => CHSValue::F(v % o),
                    _ => unreachable!()
                }
            },
            CHSValue::P(v) => {
                match rhs {
                    CHSValue::P(o) => CHSValue::P(v % o),
                    _ => unreachable!()
                }
            },
            CHSValue::B(v) => {
                match rhs {
                    CHSValue::B(o) => CHSValue::B(v % o),
                    _ => unreachable!()
                }
            },
            CHSValue::None => CHSValue::None
        }
    }
}

impl fmt::Display for CHSValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CHSValue::I(v) => write!(f, "{}", v),
            CHSValue::U(v) => write!(f, "{}", v),
            CHSValue::F(v) => write!(f, "{}", v),
            CHSValue::P(v) => write!(f, "{}", v),
            CHSValue::B(v) => write!(f, "{}", v),
            CHSValue::None => write!(f, "{}", 0)
        }
    }
}

impl From<i64> for CHSValue  {
    fn from(value: i64) -> Self {
        Self::I(value)
    }
}

impl From<f64> for CHSValue  {
    fn from(value: f64) -> Self {
        Self::F(value)
    }
}

impl From<usize> for CHSValue  {
    fn from(value: usize) -> Self {
        Self::P(value)
    }
}