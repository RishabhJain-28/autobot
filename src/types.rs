use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Types {
    String,
    Number,
}
impl Display for Types {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number => write!(f, "Number"),
            Self::String => write!(f, "String"),
        }
    }
}
