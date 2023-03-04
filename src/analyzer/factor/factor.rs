use crate::{analyzer::AnalyzedExpr, runtime::types::Type};

use super::literal::AnalyzedLiteral;

#[derive(Debug)]
pub struct AnalyzedFactor<'a> {
    pub factor: AnalyzedFactorEnum<'a>,
    pub type_info: Type,
}
impl<'a> AnalyzedFactor<'a> {
    pub fn from_literal(literal: AnalyzedLiteral<'a>) -> Self {
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

    pub fn from_analysed_expression(expr: AnalyzedExpr<'a>) -> Self {
        Self {
            type_info: expr.type_info,
            factor: AnalyzedFactorEnum::SubExpression(Box::<AnalyzedExpr<'a>>::new(expr)),
        }
    }
}

#[derive(Debug)]
pub enum AnalyzedFactorEnum<'a> {
    Literal(AnalyzedLiteral<'a>),
    Identifier(usize),
    SubExpression(Box<AnalyzedExpr<'a>>),
}
