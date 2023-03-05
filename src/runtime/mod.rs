pub mod input_utill;
pub mod keyword;
pub mod types;
pub mod value;

pub use keyword::OpenKeyword;

#[allow(dead_code)]
pub fn input_string() -> String {
    //Infallible
    input_utill::input_type::<String>(None).unwrap()
}

#[allow(dead_code)]
pub fn input_number() -> f64 {
    //Infallible
    input_utill::input_type::<f64>(None).unwrap()
}
