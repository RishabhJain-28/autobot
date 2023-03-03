use std::{io::Write, str::FromStr};

pub mod types;
pub mod value;
use self::{types::Type, value::Value};

//TODO : add prompt option to input
#[allow(dead_code)]
pub fn input_type<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut text = String::new();
    eprint!("$ ");
    std::io::stderr().flush().unwrap();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    let input = text.trim().parse::<T>();

    input
}
//TODO : add prompt option to input
#[allow(dead_code)]
pub fn input(input_type: Type) -> Result<Value, String> {
    let mut text = String::new();
    eprint!("$ ");
    std::io::stderr().flush().unwrap();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");

    let result = match input_type {
        Type::Number => {
            let input = text.trim().parse::<f64>();
            if input.is_err() {
                return Err(format!(
                    "Invalid input, Expected {}, {}",
                    Type::Number,
                    input.unwrap_err(),
                ));
            }
            Ok(Value::Number(input.unwrap()))
        }
        Type::String => {
            let input = text.trim().parse::<String>();
            Ok(Value::String(input.unwrap()))
        }
    };
    result
}
