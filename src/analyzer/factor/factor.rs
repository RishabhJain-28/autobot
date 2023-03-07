use serde::{Deserialize, Serialize};

use crate::{analyzer::AnalyzedExpr, runtime::types::Type};

use super::literal::AnalyzedLiteral;

#[derive(Debug, Serialize, Deserialize)]
pub struct AnalyzedFactor {
    pub factor: AnalyzedFactorEnum,
    pub type_info: Type,
}
impl AnalyzedFactor {
    pub fn from_literal(literal: AnalyzedLiteral) -> Self {
        match literal {
            AnalyzedLiteral::Number(_) => Self {
                factor: AnalyzedFactorEnum::Literal(literal),
                type_info: Type::Number,
            },
            AnalyzedLiteral::String(_) => Self {
                factor: AnalyzedFactorEnum::Literal(literal),
                type_info: Type::String,
            },
        }
    }

    pub fn get_identifier(handle: usize, type_info: Type) -> Self {
        Self {
            factor: AnalyzedFactorEnum::Identifier(handle),
            type_info,
        }
    }

    pub fn from_analysed_expression(expr: AnalyzedExpr) -> Self {
        Self {
            type_info: expr.type_info,
            factor: AnalyzedFactorEnum::SubExpression(Box::<AnalyzedExpr>::new(expr)),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AnalyzedFactorEnum {
    Literal(AnalyzedLiteral),
    Identifier(usize),
    //TODO : remove static
    SubExpression(Box<AnalyzedExpr>),
}
