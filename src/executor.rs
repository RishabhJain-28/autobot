use std::{fmt::format, ops::Add};

use crate::{
    analyzer::{
        AnalysedRightSideValue, AnalyzedExpr, AnalyzedFactor, AnalyzedProgram, AnalyzedStatement,
        AnalyzedTerm,
    },
    parser::{ExprOperator, TermOperator},
    symbol_table::{SymbolTable, SymbolValue},
    types::Types,
};

enum Value {
    String(String),
    Number(f64),
}
// impl Add for Value{
//     fn add(self, rhs: Self) -> f64 {

//     }
// }

fn get_number(input: Value) -> Result<f64, String> {
    match input {
        Value::String(_) => Err(format!("Expectd number found string")),
        Value::Number(num) => Ok(num),
    }
}
fn get_string(input: Value) -> Result<String, String> {
    match input {
        Value::String(str) => Ok(str),
        Value::Number(num) => Err(format!("Expectd string found number")),
    }
}

fn evaluate_factor(variables: &SymbolTable, factor: &AnalyzedFactor) -> Result<Value, String> {
    match factor {
        AnalyzedFactor::Identifier(handle) => match variables.get_value(*handle) {
            SymbolValue::Number(val) => Ok(Value::Number(*val)),
            SymbolValue::String(str) => Ok(Value::String(str.to_string())),
        },
        AnalyzedFactor::Literal(val) => Ok(Value::Number(*val)),
        //check if can unwrap
        // should subexpression be checked for type?
        AnalyzedFactor::SubExpression(expr) => evaluate_expr(variables, expr),
    }
}

fn evaluate_term(variables: &SymbolTable, term: &AnalyzedTerm) -> Result<Value, String> {
    let result = evaluate_factor(variables, &term.0)?;

    let mut result = match result {
        Value::Number(num) => num,
        Value::String(_) => {
            if !&term.1.is_empty() {
                return Err(format!(
                    "[illegal term op: math ops not allowed on strings]"
                ));
            } else {
                return Ok(result);
            }
        }
    };

    // let mut result = match  result{

    //     Value::Number(num)=>num,
    //     Value::String(_)=>String::new(),

    // }

    for factor in &term.1 {
        let val = evaluate_factor(variables, &factor.1)?;
        let mut val = match val {
            Value::Number(num) => num,
            Value::String(_) => {
                // if !&term.1.is_empty() {
                return Err(format!(
                    "[illegal term op: math ops not allowed on strings]"
                ));
                // } else {
                //     return Ok(result);
                // }
            }
        };
        // let val = get_number(val);
        // let value = match get_number(val) {
        //     Ok(v) => v,
        //     Err(err) => panic!("{}", err),
        // };

        // let (res, val) = match

        match factor.0 {
            TermOperator::Multiply => result *= val,
            TermOperator::Divide => result /= val,
        }
    }
    Ok(Value::Number(result))
}

fn evaluate_expr(variables: &SymbolTable, expr: &AnalyzedExpr) -> Result<Value, String> {
    let mut result = evaluate_term(variables, &expr.0)?;

    let mut result = match result {
        Value::Number(num) => num,
        Value::String(_) => {
            if !&expr.1.is_empty() {
                return Err(format!(
                    "[illegal expr op: math ops not allowed on strings]"
                ));
            } else {
                return Ok(result);
            }
        }
    };
    for term in &expr.1 {
        let val = evaluate_term(variables, &term.1)?;
        let mut val = match val {
            Value::Number(num) => num,
            Value::String(_) => {
                return Err(format!(
                    "[illegal expr op: math ops not allowed on strings]"
                ));
            }
        };
        match term.0 {
            ExprOperator::Add => result += val,
            ExprOperator::Subtract => result -= val,
        }
    }
    Ok(Value::Number(result))
}

fn execute_statement(
    variables: &mut SymbolTable,
    statement: &AnalyzedStatement,
) -> Result<(), String> {
    eprintln!("in execute_statement rhv: {:?}", statement);

    match statement {
        AnalyzedStatement::Declaration(_) => Ok(()),
        AnalyzedStatement::Assignment(handle, right_hand_value) => {
            match right_hand_value {
                AnalysedRightSideValue::Expression(expr) => {
                    eprintln!("in execute_statement rhv: {:?}", expr);

                    let val = match evaluate_expr(variables, expr)? {
                        Value::Number(num) => SymbolValue::Number(num),
                        Value::String(string) => SymbolValue::String(string),
                    };
                    variables.set_value(*handle, val)
                }
                AnalysedRightSideValue::String(str) => {
                    eprintln!("in string analysed rhv: {}", str);
                    variables.set_value(*handle, SymbolValue::String((**str).clone()))
                }
            }
            Ok(())
            // variables.set_value(*handle, evaluate_expr(variables, expr))
        }
        AnalyzedStatement::InputOperation(handle) => {
            let mut text = String::new();
            eprint!("? ");
            std::io::stdin()
                .read_line(&mut text)
                .expect("Cannot read line.");

            let id_type = variables.get_type(*handle);
            let value;
            match id_type {
                Types::Number => {
                    value = match text.trim().parse::<f64>() {
                        Ok(val) => SymbolValue::Number(val),
                        Err(_) => {
                            panic!("expected input as number")
                        }
                    };
                }
                Types::String => {
                    value = match text.trim().parse::<String>() {
                        Ok(str) => SymbolValue::String(str),
                        Err(_) => {
                            panic!("expected input as String")
                        }
                    };
                }
            }

            variables.set_value(*handle, value);
            Ok(())
        }
        AnalyzedStatement::OutputOperation(right_hand_value) => match right_hand_value {
            AnalysedRightSideValue::Expression(expr) => {
                eprintln!("expr in out {:?}", expr);
                let res = evaluate_expr(variables, expr)?;
                match res {
                    Value::Number(num) => {
                        println!("{}", num);
                    }
                    Value::String(string) => {
                        println!("{}", string);
                    }
                }
                // println!("{}", get_number(evaluate_expr(variables, expr)?)?);
                Ok(())
            }
            AnalysedRightSideValue::String(str) => {
                eprintln!("string in out{:?}", str);

                println!("{}", str);
                Ok(())
            }
        },
    }
}

pub fn execute_program(
    variables: &mut SymbolTable,
    analyzed_program: &AnalyzedProgram,
) -> Result<(), String> {
    for statement in analyzed_program {
        execute_statement(variables, statement)?
    }
    Ok(())
}
