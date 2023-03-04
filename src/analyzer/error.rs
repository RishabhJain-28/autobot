use std::fmt::Display;

use crate::runtime::types::Type;

#[derive(Debug)]
pub enum AnalyzerError {
    MismatchedTypes(MismatchedTypeError),
    UnexpectedNumberOfArgs(UnexpectedNumberOfArgsError),
}

#[derive(Debug)]
pub struct MismatchedTypeError {
    pub expected: Type,
    pub found: Type,
}
#[derive(Debug)]
pub struct UnexpectedNumberOfArgsError {
    pub expected: i32,
    pub found: i32,
}

impl AnalyzerError {
    pub fn mismatched_type(expected: Type, found: Type) -> Self {
        Self::MismatchedTypes(MismatchedTypeError { expected, found })
    }
    pub fn unexpected_number_of_args(expected: i32, found: i32) -> Self {
        Self::UnexpectedNumberOfArgs(UnexpectedNumberOfArgsError { expected, found })
    }
}

impl Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MismatchedTypes(MismatchedTypeError { expected, found }) => {
                writeln!(
                    f,
                    "Mismatched types=> Expected {:?}, found {:?}",
                    expected, found
                )
            }
            Self::UnexpectedNumberOfArgs(UnexpectedNumberOfArgsError {
                expected: expected_num,
                found: found_num,
            }) => {
                writeln!(
                    f,
                    "Unexpected number of args=> required {}, got {}",
                    expected_num, found_num
                )
            }
        }
    }
}
