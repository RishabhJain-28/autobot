use crate::runtime::{types::Type, value::Value};

type SymbolEntry = (String, Value);

#[derive(Debug)]
pub struct SymbolTable {
    //TODO : imporve use map instead of vector
    entities: Vec<SymbolEntry>,
}

impl SymbolTable {
    pub fn new() -> Self {
        return SymbolTable {
            entities: Vec::new(),
        };
    }

    pub fn insert_symbol(&mut self, identifier: &str, val_type: Type) -> Result<usize, String> {
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
                Type::Number => {
                    self.entities
                        .push((identifier.to_string(), Value::Number(0.0)));
                }
                Type::String => {
                    self.entities
                        .push((identifier.to_string(), Value::String(String::from(""))));
                }
            }
            Ok(self.entities.len() - 1)
        }
    }

    pub fn find_symbol(&self, identifier: &str) -> Result<(usize, Type), String> {
        if let Some(pos) = self.entities.iter().position(|val| val.0 == identifier) {
            let id_type = match self.entities[pos].1 {
                Value::Number(_) => Type::Number,
                Value::String(_) => Type::String,
            };

            Ok((pos, id_type))
        } else {
            Err(format!(
                "Identifier '{}' used before having been declared.",
                identifier
            ))
        }
    }
    pub fn get_value(&self, handle: usize) -> Value {
        Value::clone(&self.entities[handle].1)
    }
    pub fn set_value(&mut self, handle: usize, value: Value) {
        self.entities[handle].1 = value;
    }
    pub fn iter(&self) -> std::slice::Iter<SymbolEntry> {
        self.entities.iter()
    }

    pub fn get_name(&self, handle: usize) -> &String {
        &self.entities[handle].0
    }
    pub fn get_type(&self, handle: usize) -> Type {
        Value::clone(&self.entities[handle].1).into()
    }
}
