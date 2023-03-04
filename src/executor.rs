use crate::{
    analyzer::{
        AnalyzedExpr, AnalyzedFactor, AnalyzedFactorEnum, AnalyzedLiteral, AnalyzedProgram,
        AnalyzedStatement, AnalyzedTerm,
    },
    parser::{ExprOperator, TermOperator},
    runtime::{keyword::Keyword, value::Value},
    symbol_table::SymbolTable,
};

fn evaluate_factor<'a>(variables: &SymbolTable, factor: &'a AnalyzedFactor) -> Value {
    match &factor.factor {
        AnalyzedFactorEnum::Identifier(handle) => variables.get_value(*handle),
        AnalyzedFactorEnum::Literal(literal) => match literal {
            //TODO: remove clone
            AnalyzedLiteral::String(string) => Value::String(String::clone(*string)),
            AnalyzedLiteral::Number(number) => Value::Number(*number),
        },
        AnalyzedFactorEnum::SubExpression(expr) => evaluate_expr(variables, &*expr),
    }
}

fn evaluate_term<'a>(variables: &'a SymbolTable, term: &'a AnalyzedTerm) -> Value {
    let term = &term.term;
    let mut result = evaluate_factor(variables, &term.0);
    for factor in &term.1 {
        let val = evaluate_factor(variables, &factor.1);
        match factor.0 {
            TermOperator::Multiply => result = result * val,
            TermOperator::Divide => result = result / val,
        }
    }
    result
}

fn evaluate_expr<'a>(variables: &'a SymbolTable, expr: &'a AnalyzedExpr) -> Value {
    let mut result = evaluate_term(variables, &expr.expr.0);

    for term in &expr.expr.1 {
        match term.0 {
            ExprOperator::Add => result = result + evaluate_term(variables, &term.1),
            ExprOperator::Subtract => result = result - evaluate_term(variables, &term.1),
        }
    }
    result
}

fn execute_statement<'a>(
    variables: &'a mut SymbolTable,
    statement: &'a AnalyzedStatement,
) -> Result<(), String> {
    match statement {
        AnalyzedStatement::Function(keyword, vec_expr) => match keyword {
            Keyword::Open => {
                //TODO: finish this keyword function
                println!("Opening : {}", evaluate_expr(variables, &vec_expr[0]));
                Ok(())
            }
        },
        AnalyzedStatement::Declaration(_) => Ok(()),
        AnalyzedStatement::Assignment(handle, expr) => {
            let result = evaluate_expr(variables, expr);
            variables.set_value(*handle, result);
            Ok(())
        }
        AnalyzedStatement::InputOperation(handle, type_info) => {
            let value = type_info.read_from_cli_to_value()?;
            variables.set_value(*handle, value);
            Ok(())
        }

        AnalyzedStatement::OutputOperation(expr) => {
            let val = evaluate_expr(variables, expr);
            println!("{}", val);
            Ok(())
        }
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
