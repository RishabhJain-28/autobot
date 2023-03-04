use crate::{
    analyzer::{
        AnalyzedExpr, AnalyzedFactor, AnalyzedFactorEnum, AnalyzedProgram, AnalyzedStatement,
        AnalyzedTerm,
    },
    parser::{ExprOperator, TermOperator},
    runtime::types::Type,
    symbol_table::SymbolTable,
};
fn translate_to_rust_factor(variables: &SymbolTable, analyzed_factor: &AnalyzedFactor) -> String {
    match &analyzed_factor.factor {
        AnalyzedFactorEnum::Literal(value) => value.to_string(),
        AnalyzedFactorEnum::Identifier(handle) => "_".to_string() + &variables.get_name(*handle),
        AnalyzedFactorEnum::SubExpression(expr) => {
            let evaluated_expr = translate_to_rust_expr(variables, &*expr);
            match expr.type_info {
                Type::String => evaluated_expr,
                _ => "(".to_string() + &evaluated_expr + ")",
            }
        }
    }
}

fn translate_to_rust_term(variables: &SymbolTable, analyzed_term: &AnalyzedTerm) -> String {
    let mut result = translate_to_rust_factor(variables, &analyzed_term.term.0);
    for factor in &analyzed_term.term.1 {
        match factor.0 {
            TermOperator::Multiply => {
                result += " * ";
                result += &translate_to_rust_factor(variables, &factor.1);
            }
            TermOperator::Divide => {
                result += " / ";
                result += &translate_to_rust_factor(variables, &factor.1);
            }
        }
    }
    result
}

fn translate_to_rust_expr(variables: &SymbolTable, analyzed_expr: &AnalyzedExpr) -> String {
    let mut result = translate_to_rust_term(variables, &analyzed_expr.expr.0);
    for term in &analyzed_expr.expr.1 {
        match term.0 {
            ExprOperator::Add => {
                //TODO : change for strings
                result += " + ";
                result += &translate_to_rust_term(variables, &term.1);
            }
            ExprOperator::Subtract => {
                result += " - ";
                result += &translate_to_rust_term(variables, &term.1);
            }
        }
    }
    result
}
fn translate_to_rust_statement(
    variables: &SymbolTable,
    analyzed_statement: &AnalyzedStatement,
) -> String {
    match analyzed_statement {
        AnalyzedStatement::Function(keyword, vec_expr) => {
            unimplemented!()
        }
        AnalyzedStatement::Assignment(handle, expr) => format!(
            "_{} = {}",
            variables.get_name(*handle),
            translate_to_rust_expr(&variables, expr)
        ),
        AnalyzedStatement::Declaration(handle) => {
            let var_name = variables.get_name(*handle);
            let var_type = variables.get_type(*handle);
            match var_type {
                Type::Number => {
                    format!("let mut _{} : {} = 0.0", var_name, "f64")
                }
                Type::String => {
                    format!("let mut _{} : {} = String::from(\"\")", var_name, "String")
                }
            }
        }
        AnalyzedStatement::InputOperation(handle, type_info) => match type_info {
            Type::Number => {
                format!("_{} = input_number()", variables.get_name(*handle))
            }
            Type::String => {
                format!("_{} = input_string()", variables.get_name(*handle))
            }
        },
        AnalyzedStatement::OutputOperation(expr) => format!(
            "println!(\"{}\", {})",
            "{}",
            translate_to_rust_expr(&variables, expr)
        ),
    }
}
pub fn translate_to_rust_program(
    variables: &SymbolTable,
    analyzed_program: &AnalyzedProgram,
) -> String {
    let mut rust_program = String::new();

    for statement in analyzed_program {
        rust_program += "\t";

        rust_program += &translate_to_rust_statement(&variables, &statement);
        rust_program += ";\n";
    }

    let out_dir = env!("OUT_DIR");

    const MAIN: &str = "fn main(){ 
            match run() { 
                Ok(_) => (), 
                Err(err) => println!(\"Error:{}\",err)
            }
        }";
    format!(
        "include!(r\"{}\\runtime\\mod.rs\");
         \n\n{}\n\nfn run() -> Result<(), String> {{\n{}\nOk(())\n}}",
        out_dir, MAIN, rust_program
    )
}
