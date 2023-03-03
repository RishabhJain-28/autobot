use std::fmt::Display;

use crate::{types::Types, value::Value};

// type SymbolValue = Value;
// type SymbolEntry = (String, SymbolValue);
type SymbolEntry = (String, Value);

// #[derive(Debug, Clone)]
// pub enum SymbolValue {
//     String(String),
//     Number(f64),
// }

// impl Display for SymbolValue {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Number(val) => write!(f, "number: {}", val),
//             Self::String(val) => write!(f, "string: {}", val),
//         }
//     }
// }

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
                        .push((identifier.to_string(), Value::Number(0.0)));
                }
                Types::String => {
                    self.entities
                        .push((identifier.to_string(), Value::String(String::from(""))));
                }
            }
            Ok(self.entities.len() - 1)
        }
    }
    //imporve

    pub fn find_symbol(&self, identifier: &str) -> Result<(usize, Types), String> {
        if let Some(pos) = self.entities.iter().position(|val| val.0 == identifier) {
            let id_type = match self.entities[pos].1 {
                Value::Number(_) => Types::Number,
                Value::String(_) => Types::String,
            };

            Ok((pos, id_type))
        } else {
            Err(format!(
                "Error: Identifier '{}' used before having been declared.",
                identifier
            ))
        }
    }
    pub fn get_value(&self, handle: usize) -> &Value {
        &self.entities[handle].1
    }
    pub fn set_value(&mut self, handle: usize, value: Value) {
        self.entities[handle].1 = value;
    }
    pub fn iter(&self) -> std::slice::Iter<SymbolEntry> {
        self.entities.iter()
    }
    pub fn get_name(&self, handle: usize) -> String {
        self.entities[handle].0.clone()
    }
}

impl Into<Types> for Value {
    fn into(self) -> Types {
        match self {
            Self::Number(_) => Types::Number,
            Self::String(_) => Types::String,
        }
    }
}
