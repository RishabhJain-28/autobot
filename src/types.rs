pub trait TypedValue {
    // fn get_value(&self) -> Types2;
    // fn get_type(&self) -> &self {
    //     Types2::Number(0.0)
    // }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Types {
    String,
    Number,
}

//DEAL WITH COPY FOR STRING
#[derive(Debug, Clone, PartialEq)]
pub enum AnalysedTypes {
    String(String),
    Number(f64),
}

impl TypedValue for AnalysedTypes {}

// pub type NumberLiteral = f64;

// impl NumberLiteral {
//     pub fn new(Value: f64)->Self{
//         Self::new(Value)
//     }
// }

// impl TypedValue for NumberLiteral {
//     fn get_value(&self) -> Types2 {
//         Types2::Number(*self)
//     }
// }

// pub type StringLiteral = String;
