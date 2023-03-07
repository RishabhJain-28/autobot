use crate::{parser::ParsedShortcut, runtime::keyboard::KeyModes, symbol_table};

use super::{analyze_program, AnalyzedProgram};

#[derive(Debug)]
pub struct AnalyzedShortcut<'a> {
    mode: &'a Vec<KeyModes>,
    key: char,
    name: &'a str,
    body: Box<AnalyzedProgram<'a>>,
    flag: bool,
}

impl<'a> AnalyzedShortcut<'a> {
    pub fn from_parsed_shortcut(value: &'a ParsedShortcut) -> Result<AnalyzedShortcut<'a>, String> {
        let mut variables = symbol_table::SymbolTable::new();
        let analysed_body = analyze_program(&mut variables, &*value.1 .0)?;
        Ok(Self {
            body: Box::new(analysed_body),
            flag: value.0 .2 .0,  //TODO : convert to NO_REPEAT flag
            mode: &value.0 .0 .0, //TODO : convert to keystroke
            key: value.0 .1 .0,   //TODO : convert char to keystroke
            name: value.0 .3,
        })
    }
}
