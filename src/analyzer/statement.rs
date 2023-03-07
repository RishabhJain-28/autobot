use crate::{
    parser::ParsedStatement,
    runtime::{keyword::Keywords, types::Type},
    symbol_table::SymbolTable,
};

use super::{
    error::AnalyzerError,
    expr::{analyze_expr, AnalyzedExpr},
};

#[derive(Debug)]
pub enum AnalyzedStatement<'a> {
    Declaration(usize),
    InputOperation(usize, Type),
    OutputOperation(AnalyzedExpr<'a>),
    Assignment(usize, AnalyzedExpr<'a>),
    Function(Keywords, Vec<AnalyzedExpr<'a>>),
}

//TODO: return AnalyserError instead of strings
pub fn analyze_statement<'a>(
    variables: &mut SymbolTable,
    parsed_statement: &'a ParsedStatement,
) -> Result<AnalyzedStatement<'a>, String> {
    match parsed_statement {
        ParsedStatement::Shortcut(val) => {
            println!("parsed shortcut: {:?}", val);
            Err(format!("unimplemnted"))
        }
        ParsedStatement::Function(keyword, vec_expr) => {
            let analyzed_vec_expr: Vec<AnalyzedExpr<'a>> = match *keyword {
                // TODO: remove hardcoded number of args, keyword should contain this info
                Keywords::Open(_) => {
                    if vec_expr.len() > 1 {
                        return Err(format!(
                            "[illegal statement]:{:?}\n{}",
                            parsed_statement,
                            AnalyzerError::unexpected_number_of_args(1, vec_expr.len() as i32)
                        ));
                    }
                    let expr = analyze_expr(variables, &vec_expr[0])?;
                    if expr.type_info != Type::String {
                        return Err(format!(
                            "[illegal statement]:{:?}\n{}",
                            parsed_statement,
                            AnalyzerError::mismatched_type(Type::String, expr.type_info)
                        ));
                    }
                    vec![expr]
                }
            };
            Ok(AnalyzedStatement::Function(*keyword, analyzed_vec_expr))
        }
        ParsedStatement::Assignment(identifier, parsed_expr) => {
            let (handle, identifier_type_info) = variables.find_symbol(identifier)?;
            let AnalyzedExpr {
                expr: analyzed_expr,
                type_info: expected_statement_type,
            } = analyze_expr(variables, parsed_expr)?;

            if identifier_type_info != expected_statement_type {
                return Err(format!(
                    "[illegal statement]:{:?}\n{}",
                    parsed_statement,
                    AnalyzerError::mismatched_type(expected_statement_type, identifier_type_info)
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
