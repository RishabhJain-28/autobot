use crate::{
    parser::ParsedExpr,
    runtime::{operator::ExprOperator, types::Type},
    symbol_table::SymbolTable,
};

use super::term::{analyze_term, AnalyzedTerm};

#[derive(Debug)]
pub struct AnalyzedExpr<'a> {
    pub expr: (AnalyzedTerm<'a>, Vec<(ExprOperator, AnalyzedTerm<'a>)>),
    pub type_info: Type,
}
pub fn analyze_expr<'a>(
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
