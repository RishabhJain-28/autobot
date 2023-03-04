use super::{types::Type, value::Value};
use std::{io::Write, str::FromStr};

#[allow(dead_code)]
pub fn input_type<T: FromStr>(prompt: Option<&str>) -> Result<T, <T as FromStr>::Err> {
    let mut text = String::new();
    eprint!("{}", prompt.unwrap_or("$ "));
    std::io::stderr().flush().unwrap();
    std::io::stdin()
        .read_line(&mut text)
        .expect("Cannot read line.");
    let input = text.trim().parse::<T>();

    input
}

#[allow(dead_code)]
pub fn input(input_type: Type, prompt: Option<&str>) -> Result<Value, String> {
    let mut text = String::new();
    eprint!("{}", prompt.unwrap_or("$ "));
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

#[allow(dead_code)]
pub fn input_type_untill<T: FromStr>(prompt: Option<&str>, error_propmt: Option<&str>) -> T {
    let mut text = String::new();

    loop {
        eprint!("{}", prompt.unwrap_or("$ "));
        std::io::stderr().flush().unwrap();
        std::io::stdin()
            .read_line(&mut text)
            .expect("Cannot read line.");
        let input = text.trim().parse::<T>();
        match input {
            Ok(val) => {
                return val;
            }
            _ => eprintln!("{}", error_propmt.unwrap_or("Unexpected input type")),
        }
    }
}
