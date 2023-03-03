use std::{io::Write, str::FromStr};

use crate::{types::Types, value::Value};

//TODO : add prompt
#[allow(dead_code)]
pub fn input_type<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut text = String::new();
    eprint!("$ ");
    std::io::stderr().flush().unwrap();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    let input = text.trim().parse::<T>();
    // if input.is_err() {
    //     return Err(format!(
    //         "Invalid input, Expected {}, {}",
    //         T::,
    //         input.unwrap_err(),
    //     ));
    // }
    // Ok(Value::Number(input.unwrap()))
    input
}
//TODO : add prompt
#[allow(dead_code)]
pub fn input(input_type: Types) -> Result<Value, String> {
    let mut text = String::new();
    eprint!("$ ");
    std::io::stderr().flush().unwrap();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");

    let result = match input_type {
        Types::Number => {
            let input = text.trim().parse::<f64>();
            if input.is_err() {
                return Err(format!(
                    "Invalid input, Expected {}, {}",
                    Types::Number,
                    input.unwrap_err(),
                ));
            }
            Ok(Value::Number(input.unwrap()))
        }
        Types::String => {
            let input = text.trim().parse::<String>();
            if input.is_err() {
                return Err(format!(
                    "Invalid input, Expected {}, {}",
                    Types::String,
                    input.unwrap_err(),
                ));
            }
            Ok(Value::String(input.unwrap()))
        }
    };
    result
}
