//CSG : context sensitive grammer
use crate::{
    parser::{
        ExprOperator, ParsedExpr, ParsedFactor, ParsedProgram, ParsedRightSideValue,
        ParsedStatement, ParsedTerm, TermOperator,
    },
    symbol_table::SymbolTable,
    types::{StringLiteral, Types},
};

pub type AnalyzedProgram<'a> = Vec<AnalyzedStatement<'a>>;
#[derive(Debug)]
pub enum AnalyzedStatement<'a> {
    Declaration(usize),
    InputOperation(usize),
    OutputOperation(AnalysedRightSideValue<'a>),
    Assignment(usize, AnalysedRightSideValue<'a>),
}

#[derive(Debug)]
pub enum AnalysedRightSideValue<'a> {
    Expression(AnalyzedExpr),
    String(&'a StringLiteral),
}
pub type AnalyzedExpr = (AnalyzedTerm, Vec<(ExprOperator, AnalyzedTerm)>);
pub type AnalyzedTerm = (AnalyzedFactor, Vec<(TermOperator, AnalyzedFactor)>);

#[derive(Debug)]
pub enum AnalyzedFactor {
    Literal(f64),
    Identifier(usize),
    SubExpression(Box<AnalyzedExpr>),
}
pub fn analyze_program<'a>(
    variables: &mut SymbolTable,
    parsed_program: &'a ParsedProgram,
) -> Result<AnalyzedProgram<'a>, String> {
    let mut analyzed_program = AnalyzedProgram::new();

    for statement in parsed_program {
        analyzed_program.push(analyze_statement(variables, statement)?);
    }
    Ok(analyzed_program)
}

fn analyze_statement<'a>(
    variables: &mut SymbolTable,
    parsed_statement: &'a ParsedStatement,
) -> Result<AnalyzedStatement<'a>, String> {
    match parsed_statement {
        ParsedStatement::Assignment(identifier, right_hand_value) => {
            let (handle, id_type) = variables.find_symbol(identifier)?;

            match right_hand_value {
                ParsedRightSideValue::Expression(expr) => match id_type {
                    Types::Number => {
                        let analyzed_expr = analyze_expr(variables, expr)?;
                        Ok(AnalyzedStatement::Assignment(
                            handle,
                            AnalysedRightSideValue::Expression(analyzed_expr),
                        ))
                    }
                    Types::String => Result::Err("Expected expression found string".to_string()),
                },
                ParsedRightSideValue::String(str) => match id_type {
                    Types::Number => Result::Err("Expected string found expression".to_string()),
                    Types::String => {
                        // let analyzed_string = analyze_string(variables, expr)?;
                        // let analyzed_expr = analyze_expr(variables, expr)?;
                        Ok(AnalyzedStatement::Assignment(
                            handle,
                            AnalysedRightSideValue::String(str),
                        ))
                    }
                },
            }
        }
        ParsedStatement::Declaration(identifier, val_type) => {
            let handle = variables.insert_symbol(identifier, *val_type)?;
            Ok(AnalyzedStatement::Declaration(handle))
        }
        ParsedStatement::InputOperation(identifier) => {
            let (handle, _) = variables.find_symbol(identifier)?;
            Ok(AnalyzedStatement::InputOperation(handle))
        }
        ParsedStatement::OutputOperation(right_hand_value) => match right_hand_value {
            ParsedRightSideValue::Expression(expr) => {
                let analyzed_expr = analyze_expr(variables, expr)?;
                Ok(AnalyzedStatement::OutputOperation(
                    AnalysedRightSideValue::Expression(analyzed_expr),
                ))
            }
            ParsedRightSideValue::String(str) => Ok(AnalyzedStatement::OutputOperation(
                AnalysedRightSideValue::String(str),
            )),
        },
    }
}
fn analyze_expr(
    variables: &mut SymbolTable,
    parsed_expr: &ParsedExpr,
) -> Result<AnalyzedExpr, String> {
    let first_term = analyze_term(variables, &parsed_expr.0)?;
    let mut other_terms = Vec::<(ExprOperator, AnalyzedTerm)>::new();
    for term in &parsed_expr.1 {
        other_terms.push((term.0, analyze_term(variables, &term.1)?));
    }
    Ok((first_term, other_terms))
}
fn analyze_term(
    variables: &mut SymbolTable,
    parsed_term: &ParsedTerm,
) -> Result<AnalyzedTerm, String> {
    let first_factor = analyze_factor(variables, &parsed_term.0)?;
    let mut other_factors = Vec::<(TermOperator, AnalyzedFactor)>::new();
    for factor in &parsed_term.1 {
        other_factors.push((factor.0, analyze_factor(variables, &factor.1)?));
    }
    Ok((first_factor, other_factors))
}
fn analyze_factor(
    variables: &mut SymbolTable,
    parsed_factor: &ParsedFactor,
) -> Result<AnalyzedFactor, String> {
    match parsed_factor {
        ParsedFactor::Literal(value) => Ok(AnalyzedFactor::Literal(*value)),
        ParsedFactor::Identifier(name) => {
            Ok(AnalyzedFactor::Identifier(variables.find_symbol(name)?.0))
        }
        ParsedFactor::SubExpression(expr) => Ok(AnalyzedFactor::SubExpression(
            Box::<AnalyzedExpr>::new(analyze_expr(variables, expr)?),
        )),
    }
}
