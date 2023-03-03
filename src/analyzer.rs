//CSG : context sensitive grammer
use crate::{
    parser::{
        ExprOperator, ParsedExpr, ParsedFactor, ParsedLiteral, ParsedProgram, ParsedStatement,
        ParsedTerm, TermOperator,
    },
    runtime::types::Type,
    symbol_table::SymbolTable,
};

pub type AnalyzedProgram<'a> = Vec<AnalyzedStatement<'a>>;
#[derive(Debug)]
pub enum AnalyzedStatement<'a> {
    Declaration(usize),
    InputOperation(usize, Type),
    OutputOperation(AnalyzedExpr<'a>),
    Assignment(usize, AnalyzedExpr<'a>),
}

#[derive(Debug)]
pub struct AnalyzedExpr<'a> {
    pub expr: (AnalyzedTerm<'a>, Vec<(ExprOperator, AnalyzedTerm<'a>)>),
    pub type_info: Type,
}

#[derive(Debug)]

pub struct AnalyzedTerm<'a> {
    pub term: (AnalyzedFactor<'a>, Vec<(TermOperator, AnalyzedFactor<'a>)>),
    pub type_info: Type,
}
#[derive(Debug)]

pub struct AnalyzedFactor<'a> {
    pub factor: AnalyzedFactorEnum<'a>,
    pub type_info: Type,
}
impl<'a> AnalyzedFactor<'a> {
    fn from_literal(literal: AnalyzedLiteral<'a>) -> Self {
        match literal {
            AnalyzedLiteral::Number(_) => Self {
                factor: AnalyzedFactorEnum::Literal(literal),
                type_info: Type::Number,
            },
            AnalyzedLiteral::String(_) => Self {
                factor: AnalyzedFactorEnum::Literal(literal),
                type_info: Type::String,
            },
        }
    }

    fn get_identifier(handle: usize, type_info: Type) -> Self {
        Self {
            factor: AnalyzedFactorEnum::Identifier(handle),
            type_info,
        }
    }

    fn from_analysed_expression(expr: AnalyzedExpr<'a>) -> Self {
        Self {
            type_info: expr.type_info,
            factor: AnalyzedFactorEnum::SubExpression(Box::<AnalyzedExpr<'a>>::new(expr)),
        }
    }
}
#[derive(Debug)]
pub enum AnalyzedFactorEnum<'a> {
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

fn analyze_expr<'a>(
    variables: &mut SymbolTable,
    parsed_expr: &'a ParsedExpr,
) -> Result<AnalyzedExpr<'a>, String> {
    let first_term = analyze_term(variables, &parsed_expr.0)?;

    let expected_expr_type = first_term.type_info;

    let mut other_terms = Vec::<(ExprOperator, AnalyzedTerm)>::new();
    for term in &parsed_expr.1 {
        let analysed_term = analyze_term(variables, &term.1)?;
        if analysed_term.type_info != expected_expr_type {
            return Err(format!(
                "[illegal factor] : {:?} \n Mismatched types=> Expected {:?}, found {:?}",
                parsed_expr, expected_expr_type, analysed_term.type_info
            ));
        };

        other_terms.push((term.0, analysed_term));
    }
    Ok(AnalyzedExpr {
        type_info: first_term.type_info,
        expr: (first_term, other_terms),
    })
}

fn analyze_term<'a>(
    variables: &mut SymbolTable,
    parsed_term: &'a ParsedTerm,
) -> Result<AnalyzedTerm<'a>, String> {
    let first_factor = analyze_factor(variables, &parsed_term.0)?;
    let expected_term_type = first_factor.type_info;

    let mut other_factors = Vec::<(TermOperator, AnalyzedFactor)>::new();
    for factor in &parsed_term.1 {
        let analysed_fac = analyze_factor(variables, &factor.1)?;

        if analysed_fac.type_info != expected_term_type {
            return Err(format!(
                "[illegal term] : {:?} \n Mismatched types=> Expected {:?}, found {:?}",
                parsed_term, expected_term_type, analysed_fac.type_info
            ));
        };
        other_factors.push((factor.0, analysed_fac));
    }
    Ok(AnalyzedTerm {
        type_info: expected_term_type,
        term: (first_factor, other_factors),
    })
}

fn analyze_factor<'a>(
    variables: &mut SymbolTable,
    parsed_factor: &'a ParsedFactor,
) -> Result<AnalyzedFactor<'a>, String> {
    match parsed_factor {
        ParsedFactor::Literal(literal) => match literal {
            ParsedLiteral::Number(val) => {
                Ok(AnalyzedFactor::from_literal(AnalyzedLiteral::Number(*val)))
            }
            ParsedLiteral::String(string) => Ok(AnalyzedFactor::from_literal(
                AnalyzedLiteral::String(string),
            )),
        },
        ParsedFactor::Identifier(name) => {
            let (handle, type_info) = variables.find_symbol(name)?;
            Ok(AnalyzedFactor::get_identifier(handle, type_info))
        }
        ParsedFactor::SubExpression(expr) => {
            let analysed_expr = analyze_expr(variables, &**expr)?;
            Ok(AnalyzedFactor::from_analysed_expression(analysed_expr))
        }
    }
}
