use super::super::{types::Type, value::Value};

use super::{Operator, BINARY_OP};

#[derive(Debug, Clone, Copy, PartialEq)]
// pub enum ExprOperator {
//     Add,
//     Subtract,
// }
pub enum ExprOperator {
    Add(AddOp),
    Subtract(SubOp),
}

impl ExprOperator {
    pub fn add() -> Self {
        ExprOperator::Add(AddOp)
    }
    pub fn subtract() -> Self {
        ExprOperator::Subtract(SubOp)
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct AddOp;
impl Operator<{ BINARY_OP }> for AddOp {
    fn execute_op(&self, args: [Value; BINARY_OP]) -> Value {
        let [a, b] = args;
        a + b
    }
    fn compile_op(&self, args: [(&String, Type); BINARY_OP]) -> String {
        let [a, b] = args;
        let mut res = String::from(a.0);
        match (a.1, b.1) {
            (Type::Number, Type::Number) => {
                res += " + ";
                res += b.0;
            }
            (Type::String, Type::String) => {
                res += " + &";
                res += b.0;
            }
            (_, _) => panic!("Invalid Addition"),
        }

        res
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]

pub struct SubOp;
impl Operator<{ BINARY_OP }> for SubOp {
    fn execute_op(&self, args: [Value; BINARY_OP]) -> Value {
        let [a, b] = args;
        a - b
    }
    fn compile_op(&self, args: [(&String, Type); BINARY_OP]) -> String {
        let [a, b] = args;
        let mut res = String::from(a.0);
        match (a.1, b.1) {
            (Type::Number, Type::Number) => {
                res += " + ";
                res += b.0;
            }
            (_, _) => panic!("Invalid Substraction"),
        }
        res
    }
}
