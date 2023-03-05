#[derive(Debug)]
pub enum AnalyzedLiteral<'a> {
    String(&'a String),
    Number(f64),
}

impl<'a> ToString for AnalyzedLiteral<'a> {
    fn to_string(&self) -> String {
        match self {
            Self::Number(num) => num.to_string() + "f64",
            Self::String(string) => format!("String::from(r#\"{}\"#)", string),
        }
    }
}
