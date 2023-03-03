use std::fmt::Display;

use super::{input_type, value::Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Type {
    String,
    Number,
}

impl Type {
    pub fn read_from_cli_to_value(&self) -> Result<Value, String> {
        match self {
            Type::String => {
                // Parse is infalliable
                let val = input_type::<String>().unwrap();
                Ok(Value::String(val))
            }
            Type::Number => {
                let val = input_type::<f64>();
                let val = match val {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(format!("Invalid input, Expected {}, {}", Type::Number, err));
                    }
                };
                Ok(Value::Number(val))
            }
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number => write!(f, "Number"),
            Self::String => write!(f, "String"),
        }
    }
}
