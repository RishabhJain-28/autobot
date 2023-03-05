mod expr_op;
pub use expr_op::ExprOperator;

use super::value::Value;
pub trait Operator<const COUNT: usize> {
    fn execute_op(&self, _: [Value; COUNT]) -> Value {
        unimplemented!()
    }
    fn compile_op(&self, _: [&String; COUNT]) -> String {
        unimplemented!()
    }
}
const BINARY_OP: usize = 2;
