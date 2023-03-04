use crate::{
    parser::{ParsedTerm, TermOperator},
    runtime::types::Type,
    symbol_table::SymbolTable,
};

use super::factor::{analyze_factor, AnalyzedFactor};

#[derive(Debug)]
pub struct AnalyzedTerm<'a> {
    pub term: (AnalyzedFactor<'a>, Vec<(TermOperator, AnalyzedFactor<'a>)>),
    pub type_info: Type,
}

pub fn analyze_term<'a>(
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
