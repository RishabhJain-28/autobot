use serde::{Deserialize, Serialize};

use crate::{parser::ParsedShortcut, runtime::keyboard::KeyModes, symbol_table};

use super::{analyze_program, AnalyzedProgram};

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzedShortcut<'a> {
    pub mode: Vec<KeyModes>,
    pub key: char,
    pub name: &'a str,
    pub body: Box<AnalyzedProgram<'a>>,
    pub flag: bool,
    pub symbol_table: symbol_table::SymbolTable,
}

impl<'a> AnalyzedShortcut<'a> {
    pub fn from_parsed_shortcut(value: &'a ParsedShortcut) -> Result<AnalyzedShortcut<'a>, String> {
        let mut variables = symbol_table::SymbolTable::new();
        let analysed_body = analyze_program(&mut variables, &*value.1 .0)?;
        Ok(Self {
            body: Box::new(analysed_body),
            flag: value.0 .2 .0,         //TODO : convert to NO_REPEAT flag
            mode: value.0 .0 .0.clone(), //TODO : convert to keystroke
            key: value.0 .1 .0,          //TODO : convert char to keystroke
            name: value.0 .3,
            symbol_table: variables,
        })
    }
}
