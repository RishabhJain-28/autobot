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

fn get_number(input: Value) -> f64 {
    match input {
        Value::String(_) => panic!("Expectd number found string"),
        Value::Number(num) => num,
    }
}
fn get_string(input: Value) -> String {
    match input {
        Value::String(str) => str,
        Value::Number(num) => panic!("Expectd string found number"),
    }
}

fn evaluate_factor(variables: &SymbolTable, factor: &AnalyzedFactor) -> Value {
    match factor {
        AnalyzedFactor::Identifier(handle) => match variables.get_value(*handle) {
            SymbolValue::Number(val) => Value::Number(*val),
            SymbolValue::String(str) => Value::String(str.to_string()),
        },
        AnalyzedFactor::Literal(val) => Value::Number(*val),
        AnalyzedFactor::SubExpression(expr) => Value::Number(evaluate_expr(variables, expr)),
    }
}

fn evaluate_term(variables: &SymbolTable, term: &AnalyzedTerm) -> f64 {
    let mut result = evaluate_factor(variables, &term.0);
    let mut result = get_number(result);

    for factor in &term.1 {
        let val = evaluate_factor(variables, &factor.1);
        let val = get_number(val);
        // let value = match get_number(val) {
        //     Ok(v) => v,
        //     Err(err) => panic!("{}", err),
        // };

        match factor.0 {
            TermOperator::Multiply => result *= val,
            TermOperator::Divide => result /= val,
        }
    }
    result
}

fn evaluate_expr(variables: &SymbolTable, expr: &AnalyzedExpr) -> f64 {
    let mut result = evaluate_term(variables, &expr.0);

    for term in &expr.1 {
        match term.0 {
            ExprOperator::Add => result += evaluate_term(variables, &term.1),
            ExprOperator::Subtract => result -= evaluate_term(variables, &term.1),
        }
    }
    result
}

fn execute_statement(variables: &mut SymbolTable, statement: &AnalyzedStatement) {
    eprintln!("in execute_statement rhv: {:?}", statement);

    match statement {
        AnalyzedStatement::Declaration(_) => {}
        AnalyzedStatement::Assignment(handle, right_hand_value) => {
            match right_hand_value {
                AnalysedRightSideValue::Expression(expr) => {
                    eprintln!("in execute_statement rhv: {:?}", expr);

                    let val = evaluate_expr(variables, expr);
                    variables.set_value(*handle, SymbolValue::Number(val))
                }
                AnalysedRightSideValue::String(str) => {
                    eprintln!("in string analysed rhv: {}", str);
                    variables.set_value(*handle, SymbolValue::String((**str).clone()))
                }
            }

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
        }
        AnalyzedStatement::OutputOperation(right_hand_value) => match right_hand_value {
            AnalysedRightSideValue::Expression(expr) => {
                eprintln!("expr in out {:?}", expr);
                println!("{}", evaluate_expr(variables, expr))
            }
            AnalysedRightSideValue::String(str) => {
                eprintln!("string in out{:?}", str);

                println!("{}", str)
            }
        },
    }
}

pub fn execute_program(variables: &mut SymbolTable, analyzed_program: &AnalyzedProgram) {
    for statement in analyzed_program {
        execute_statement(variables, statement)
    }
}
