//CSG : context sensitive grammer
mod error;
mod expr;
mod factor;
mod shortcut;
mod statement;
mod term;

pub use expr::AnalyzedExpr;
pub use factor::AnalyzedFactor;
pub use factor::AnalyzedFactorEnum;
pub use factor::AnalyzedLiteral;
pub use statement::AnalyzedStatement;
pub use term::AnalyzedTerm;

use crate::{parser::ParsedProgram, symbol_table::SymbolTable};

use self::statement::analyze_statement;

pub type AnalyzedProgram<'a> = Vec<AnalyzedStatement<'a>>;

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
