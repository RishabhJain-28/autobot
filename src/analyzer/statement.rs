use crate::{parser::ParsedStatement, runtime::types::Type, symbol_table::SymbolTable};

use super::expr::{analyze_expr, AnalyzedExpr};

#[derive(Debug)]
pub enum AnalyzedStatement<'a> {
    Declaration(usize),
    InputOperation(usize, Type),
    OutputOperation(AnalyzedExpr<'a>),
    Assignment(usize, AnalyzedExpr<'a>),
}

pub fn analyze_statement<'a>(
    variables: &mut SymbolTable,
    parsed_statement: &'a ParsedStatement,
) -> Result<AnalyzedStatement<'a>, String> {
    match parsed_statement {
        ParsedStatement::Assignment(identifier, parsed_expr) => {
            let (handle, identifier_type_info) = variables.find_symbol(identifier)?;
            let AnalyzedExpr {
                expr: analyzed_expr,
                type_info: expected_statement_type,
            } = analyze_expr(variables, parsed_expr)?;

            if identifier_type_info != expected_statement_type {
                return Err(format!(
                    "[illegal statement] : {:?} \n Mismatched types=> Expected {:?}, found {:?}",
                    parsed_statement, expected_statement_type, identifier_type_info
                ));
            };

            Ok(AnalyzedStatement::Assignment(
                handle,
                AnalyzedExpr {
                    expr: analyzed_expr,
                    type_info: identifier_type_info,
                },
            ))
        }
        ParsedStatement::Declaration(identifier, id_type) => {
            let handle = variables.insert_symbol(identifier, *id_type)?;
            Ok(AnalyzedStatement::Declaration(handle))
        }
        ParsedStatement::InputOperation(identifier) => {
            let (handle, type_info) = variables.find_symbol(identifier)?;
            Ok(AnalyzedStatement::InputOperation(handle, type_info))
        }
        ParsedStatement::OutputOperation(expr) => {
            let analyzed_expr = analyze_expr(variables, expr)?;
            Ok(AnalyzedStatement::OutputOperation(analyzed_expr))
        }
    }
}
