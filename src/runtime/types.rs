use std::fmt::Display;

use super::{
    input_utill::{input_type, input_type_untill},
    value::Value,
};

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
                // let val = input_type::<String>(Some("Enter a string")).unwrap();
                let val = input_type::<String>(None).unwrap();
                Ok(Value::String(val))
            }
            Type::Number => {
                // let val = input_type::<f64>(Some("Enter a number"));
                let val = input_type::<f64>(None);
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
    #[allow(dead_code)]
    pub fn read_from_cli_to_value_untill_correct(&self) -> Value {
        match self {
            Type::String => {
                Value::String(input_type_untill::<String>(Some("Enter a string: "), None))
            }
            Type::Number => Value::Number(input_type_untill::<f64>(Some("Enter a number: "), None)),
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
