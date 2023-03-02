use std::fmt::Display;

use crate::types::Types;

type SymbolEntry = (String, SymbolValue);

#[derive(Debug, Clone)]
pub enum SymbolValue {
    String(String),
    Number(f64),
}

impl Display for SymbolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.)
        match self {
            Self::Number(val) => write!(f, "number: {}", val),
            Self::String(val) => write!(f, "string: {}", val),
        }
    }
}

#[derive(Debug)]

pub struct SymbolTable {
    entities: Vec<SymbolEntry>,
}

impl<'a> SymbolTable {
    pub fn new() -> Self {
        return SymbolTable {
            entities: Vec::new(),
        };
    }

    //imporve
    pub fn insert_symbol(&mut self, identifier: &str, val_type: Types) -> Result<usize, String> {
        if self
            .entities
            .iter()
            .find(|val| val.0 == identifier)
            .is_some()
        {
            Err(format!(
                "Error: Identifier '{}' declared several times.",
                identifier
            ))
        } else {
            match val_type {
                Types::Number => {
                    self.entities
                        .push((identifier.to_string(), SymbolValue::Number(0.0)));
                }
                Types::String => {
                    self.entities.push((
                        identifier.to_string(),
                        SymbolValue::String(String::from("")),
                    ));
                }
            }
            Ok(self.entities.len() - 1)
        }
    }
    //imporve

    pub fn find_symbol(&self, identifier: &str) -> Result<(usize, Types), String> {
        if let Some(pos) = self.entities.iter().position(|val| val.0 == identifier) {
            let id_type = match self.entities[pos].1 {
                SymbolValue::Number(_) => Types::Number,
                SymbolValue::String(_) => Types::String,
            };

            Ok((pos, id_type))
        } else {
            Err(format!(
                "Error: Identifier '{}' used before having been declared.",
                identifier
            ))
        }
    }
    pub fn get_value(&self, handle: usize) -> &SymbolValue {
        &self.entities[handle].1
    }
    pub fn set_value(&mut self, handle: usize, value: SymbolValue) {
        self.entities[handle].1 = value;
    }
    pub fn iter(&self) -> std::slice::Iter<SymbolEntry> {
        self.entities.iter()
    }
    pub fn get_name(&self, handle: usize) -> String {
        self.entities[handle].0.clone()
    }

    pub fn get_type(&self, handle: usize) -> Types {
        match self.entities[handle].1 {
            SymbolValue::Number(_) => Types::Number,
            SymbolValue::String(_) => Types::String,
        }
    }
}
