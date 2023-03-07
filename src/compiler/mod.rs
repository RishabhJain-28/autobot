mod compiler_to_rust;
pub use compiler_to_rust::*;

use crate::{analyzer::AnalyzedProgram, symbol_table::SymbolTable};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct CompiledAB<'a> {
    #[serde(borrow)]
    code: AnalyzedProgram<'a>,

    variables: SymbolTable,
}

//just serialize for now
pub fn compile_program(
    variables: SymbolTable,
    analyzed_program: AnalyzedProgram,
) -> Result<(), String> {
    let res = serde_json::to_string_pretty(&CompiledAB {
        code: analyzed_program,
        variables: variables,
    })
    .unwrap();
    println!("json : {}", res);

    // match std::fs::write(SHORTCUT_DB, res) {
    //     Err(err) => {
    //         return Err(format!("Err occured while saving : {}", err));
    //     }
    //     Ok(_) => (),
    // }

    unimplemented!()
}
