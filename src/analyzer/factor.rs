mod factor;
mod literal;
use crate::{
    parser::{ParsedFactor, ParsedLiteral},
    symbol_table::SymbolTable,
};
pub use factor::{AnalyzedFactor, AnalyzedFactorEnum};
pub use literal::AnalyzedLiteral;

use super::expr::analyze_expr;

pub fn analyze_factor<'a>(
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
