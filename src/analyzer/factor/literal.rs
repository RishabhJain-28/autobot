use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AnalyzedLiteral {
    String(String),
    Number(f64),
}

impl ToString for AnalyzedLiteral {
    fn to_string(&self) -> String {
        match self {
            Self::Number(num) => num.to_string() + "f64",
            Self::String(string) => format!("String::from(r#\"{}\"#)", string),
        }
    }
}
