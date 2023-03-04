use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use super::types::Type;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
}

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a + b),
            (Value::String(a), Value::String(b)) => {
                let res = a + &b;
                Value::String(res)
            }
            (_, _) => panic!("Invalid Addition"),
        }
    }
}
impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a - b),
            (_, _) => panic!("Invalid Substraction"),
        }
    }
}
impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a * b),
            (_, _) => panic!("Invalid Multiplication"),
        }
    }
}
impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Value::Number(a / b),
            (_, _) => panic!("Invalid Division"),
        }
    }
}
impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(val) => write!(f, "{}", val),
            Self::String(val) => write!(f, "{}", val),
        }
    }
}
impl<'a> Into<Type> for Value {
    fn into(self) -> Type {
        match self {
            Self::Number(_) => Type::Number,
            Self::String(_) => Type::String,
        }
    }
}
