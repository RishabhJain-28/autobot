use std::ops::Add;

use crate::{
    analyzer::{
        AnalyzedExpr, AnalyzedFactor, AnalyzedFactorEnum, AnalyzedLiteral, AnalyzedProgram,
        AnalyzedStatement, AnalyzedTerm,
    },
    parser::{ExprOperator, TermOperator},
    runtime,
    symbol_table::SymbolTable,
    types::Types,
    value::{Value, self},
};

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
    match &factor.factor {
        //CAHNGE
        AnalyzedFactorEnum::Identifier(handle) => *variables.get_value(*handle),
        // AnalyzedFactorEnum::Identifier(handle) => Value::Number(0.0),
        // AnalyzedFactorEnum::Identifier(handle) => match variables.get_value(handle) {
        //     SymbolValue::Number(val) => Value::Number(*val),
        //     //CHANGE: remove to string
        //     SymbolValue::String(str) => Value::String(str.to_string()),
        // },
        AnalyzedFactorEnum::Literal(literal) => match literal {
            //CHANGE: remove clone
            AnalyzedLiteral::String(string) => Value::String((**string).clone()),
            AnalyzedLiteral::Number(number) => Value::Number(*number),
        },
        AnalyzedFactorEnum::SubExpression(expr) => {
            evaluate_expr(variables, &*expr)
            // let evaluated_expr = evaluate_expr(variables, &*expr);
            // match evaluated_expr. {

            // }
            // change
            // Value::Number(0.0)
        }
    }
}

fn evaluate_term(variables: &SymbolTable, term: &AnalyzedTerm) -> Value {
    let type_info = &term.type_info;
    let term = &term.term;
    let mut result = evaluate_factor(variables, &term.0);
    // let mut result = match type_info {

    // }

    // get_number(result);

    for factor in &term.1 {
        let val = evaluate_factor(variables, &factor.1);
        // let val = get_number(val);

        // let value = match get_number(val) {
        //     Ok(v) => v,
        //     Err(err) => panic!("{}", err),
        // };

        match factor.0 {
            TermOperator::Multiply => result = result * val,
            TermOperator::Divide => result = result / val,
        }
    }
    result
}

fn evaluate_expr(variables: &SymbolTable, expr: &AnalyzedExpr) -> Value {
    let mut result = evaluate_term(variables, &expr.expr.0);

    for term in &expr.expr.1 {
        match term.0 {
            ExprOperator::Add => result = result + evaluate_term(variables, &term.1),
            ExprOperator::Subtract => result = result - evaluate_term(variables, &term.1),
        }
    }
    result
}

fn execute_statement(
    variables: &mut SymbolTable,
    statement: &AnalyzedStatement,
) -> Result<(), String> {
    eprintln!("in execute_statement rhv: {:?}", statement);

    match statement {
        AnalyzedStatement::Declaration(_) => Ok(()),
        AnalyzedStatement::Assignment(handle, expr) => {
            let result = evaluate_expr(variables, expr);

            variables.set_value(*handle, result);
            Ok(())

        }
        AnalyzedStatement::InputOperation(handle, type_info) => {
            match type_info {
                Types::String => {
                    let val = runtime::input_type::<String>();
                    
                    let val = match val {
Ok(val)=>val, 
Err(err)=>{
   return Err(format!(
            "Invalid input, Expected {}, {}",
            Types::String,
         err
        ));
}                        
                    };

                    variables.set_value(*handle, Value::String(val));
                }
                Types::Number => {
                    let val = runtime::input_type::<f64>();
                    let val = match val {
                        Ok(val)=>val, 
                        Err(err)=>{
                           return Err(format!(
                                    "Invalid input, Expected {}, {}",
                                    Types::Number,
                                 err
                                ));
                        }                  
                        };                  
                    variables.set_value(*handle, Value::Number(val));
                }
            };

            Ok(())
        }

        AnalyzedStatement::OutputOperation(expr) => {
            let val= evaluate_expr(variables, expr);
            println!("{}",val );
Ok(())
        }
        // AnalyzedStatement::OutputOperation(expr) => match expr.expr {
            
        //     AnalysedRightSideValue::Expression(expr) => {
        //         eprintln!("expr in out {:?}", expr);
        //         println!("{}", evaluate_expr(variables, expr))
        //         Ok(())

        //     }
        //     AnalysedRightSideValue::String(str) => {
        //         eprintln!("string in out{:?}", str);

        //         println!("{}", str)
        //         Ok(())

        //     }
        // },
        // AnalyzedStatement::OutputOperation(expr) => match expr.expr {
            
        //     AnalysedRightSideValue::Expression(expr) => {
        //         eprintln!("expr in out {:?}", expr);
        //         println!("{}", evaluate_expr(variables, expr))
        //         Ok(())

        //     }
        //     AnalysedRightSideValue::String(str) => {
        //         eprintln!("string in out{:?}", str);

        //         println!("{}", str)
        //         Ok(())

        //     }
        // },
    }
}

pub fn execute_program(variables: &mut SymbolTable, analyzed_program: &AnalyzedProgram)->Result<(),String> {
    for statement in analyzed_program {
        execute_statement(variables, statement)?
    }
    Ok(())
}
