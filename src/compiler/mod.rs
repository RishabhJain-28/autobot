mod compiler_to_rust;
pub use compiler_to_rust::*;

use crate::{analyzer::AnalyzedProgram, symbol_table::SymbolTable};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct CompiledAB<'a> {
    #[serde(borrow)]
    pub code: AnalyzedProgram<'a>,

    pub variables: SymbolTable,
}

impl<'a> CompiledAB<'a> {
    pub fn get_code_variables(self) -> (AnalyzedProgram<'a>, SymbolTable) {
        (self.code, self.variables)
    }
}

//just serialize for now
pub fn compile_program(
    variables: SymbolTable,
    analyzed_program: AnalyzedProgram,
    name: &str,
) -> Result<(), String> {
    let res = serde_json::to_string_pretty(&CompiledAB {
        code: analyzed_program,
        variables: variables,
    })
    .unwrap();
    //todo: fix file location
    match std::fs::write(format!("{}.json", String::from(name)), res) {
        Err(err) => {
            return Err(format!("Err occured while saving : {}", err));
        }
        Ok(_) => Ok(()),
    }
}
