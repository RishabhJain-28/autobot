use std::process::id;

//CSG : context sensitive grammer
use crate::{
    parser::{
        ExprOperator, ParsedExpr, ParsedFactor, ParsedLiteral, ParsedProgram, ParsedStatement,
        ParsedTerm, TermOperator,
    },
    symbol_table::SymbolTable,
    types::Types,
};

pub type AnalyzedProgram<'a> = Vec<AnalyzedStatement<'a>>;
#[derive(Debug)]
pub enum AnalyzedStatement<'a> {
    Declaration(usize),
    InputOperation(usize, Types),
    OutputOperation(AnalyzedExpr<'a>),
    Assignment(usize, AnalyzedExpr<'a>),
}

pub type AnalyzedExpr<'a> = (
    (AnalyzedTerm<'a>, Vec<(ExprOperator, AnalyzedTerm<'a>)>),
    Types,
);
pub type AnalyzedTerm<'a> = (
    (AnalyzedFactor<'a>, Vec<(TermOperator, AnalyzedFactor<'a>)>),
    Types,
);

pub type AnalyzedFactor<'a> = (AnalyzedFactorUntyped<'a>, Types);

#[derive(Debug)]
pub enum AnalyzedFactorUntyped<'a> {
    Literal(AnalyzedLiteral<'a>),
    Identifier(usize),
    SubExpression(Box<AnalyzedExpr<'a>>),
}

#[derive(Debug)]
pub enum AnalyzedLiteral<'a> {
    String(&'a String),
    Number(f64),
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
    eprintln!("{:?}", parsed_statement);
    match parsed_statement {
        ParsedStatement::Assignment(identifier, expr) => {
            let (handle, id_type) = variables.find_symbol(identifier)?;
            let (analyzed_expr, type_info) = analyze_expr(variables, expr)?;
            eprintln!("id_type: {:?}", id_type);
            eprintln!("type_info: {:?}", type_info);

            if type_info != id_type {
                return Err(format!("Expected type {:?} found {:?}", id_type, type_info));
            };

            Ok(AnalyzedStatement::Assignment(
                handle,
                (analyzed_expr, type_info),
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
fn analyze_expr<'a>(
    variables: &mut SymbolTable,
    parsed_expr: &'a ParsedExpr<'a>,
) -> Result<AnalyzedExpr<'a>, String> {
    let first_term = analyze_term(variables, &parsed_expr.0)?;

    let mut other_terms = Vec::<(ExprOperator, AnalyzedTerm)>::new();
    for term in &parsed_expr.1 {
        other_terms.push((term.0, analyze_term(variables, &term.1)?));
    }

    Ok(((first_term, other_terms), Types::Number))
}

fn analyze_term<'a>(
    variables: &mut SymbolTable,
    parsed_term: &'a ParsedTerm,
) -> Result<AnalyzedTerm<'a>, String> {
    let first_factor = analyze_factor(variables, &parsed_term.0)?;

    let mut other_factors = Vec::<(TermOperator, AnalyzedFactor)>::new();
    for factor in &parsed_term.1 {
        other_factors.push((factor.0, analyze_factor(variables, &factor.1)?));
    }
    Ok(((first_factor, other_factors), Types::Number))
}

fn analyze_factor<'a>(
    variables: &mut SymbolTable,
    parsed_factor: &'a ParsedFactor,
) -> Result<AnalyzedFactor<'a>, String> {
    match parsed_factor {
        ParsedFactor::Literal(literal) => match literal {
            ParsedLiteral::Number(val) => Ok((
                AnalyzedFactorUntyped::Literal(AnalyzedLiteral::Number(*val)),
                Types::Number,
            )),
            ParsedLiteral::String(string) => Ok((
                AnalyzedFactorUntyped::Literal(AnalyzedLiteral::String(string)),
                Types::String,
            )),
        },
        ParsedFactor::Identifier(name) => {
            let (handle, type_info) = variables.find_symbol(name)?;
            Ok((AnalyzedFactorUntyped::Identifier(handle), type_info))
        }
        ParsedFactor::SubExpression(expr) => {
            let expr = analyze_expr(variables, expr)?;
            Ok((
                AnalyzedFactorUntyped::SubExpression(Box::<AnalyzedExpr>::new(expr)),
                expr.1,
            ))
        }
    }
}
