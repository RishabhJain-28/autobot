// CFG parser
mod string_parser;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{alpha1, char},
    combinator::map,
    multi::many0,
    number::complete::double,
    sequence::{delimited, preceded, tuple},
    IResult,
};
use string_parser::parse_string;

use crate::runtime::types::Type;

pub type ParsedProgram<'a> = Vec<ParsedStatement<'a>>;

#[derive(Debug, PartialEq)]
pub enum ParsedStatement<'a> {
    Declaration(&'a str, Type),
    InputOperation(&'a str),
    OutputOperation(ParsedExpr<'a>),
    Assignment(&'a str, ParsedExpr<'a>),
}

pub type ParsedExpr<'a> = (ParsedTerm<'a>, Vec<(ExprOperator, ParsedTerm<'a>)>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExprOperator {
    Add,
    Subtract,
}
pub type ParsedTerm<'a> = (ParsedFactor<'a>, Vec<(TermOperator, ParsedFactor<'a>)>);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TermOperator {
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum ParsedFactor<'a> {
    Literal(ParsedLiteral),
    Identifier(&'a str),
    SubExpression(Box<ParsedExpr<'a>>),
}

#[derive(Debug, PartialEq)]
pub enum ParsedLiteral {
    String(String),
    Number(f64),
}

pub fn parse_program(input: &str) -> IResult<&str, ParsedProgram> {
    many0(preceded(
        skip_spaces,
        alt((
            parse_decleration,
            parse_input_statement,
            parse_output_statement,
            parse_assignment,
        )),
    ))(input)
}

fn parse_decleration(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((
        char('@'),
        skip_spaces,
        parse_identifier,
        skip_spaces,
        parse_type,
    ))(input)
    .map(|(input, output)| (input, ParsedStatement::Declaration(output.2, output.4)))
}

fn parse_input_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('>'), skip_spaces, parse_identifier))(input)
        .map(|(input, output)| (input, ParsedStatement::InputOperation(output.2)))
}

fn parse_output_statement(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((char('<'), skip_spaces, parse_expression))(input)
        .map(|(input, output)| (input, ParsedStatement::OutputOperation(output.2)))
}

fn parse_assignment(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((
        parse_identifier,
        skip_spaces,
        tag(":="),
        skip_spaces,
        parse_expression,
    ))(input)
    .map(|(input, output)| (input, ParsedStatement::Assignment(output.0, output.4)))
}

fn parse_expression(input: &str) -> IResult<&str, ParsedExpr> {
    tuple((
        parse_term,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char('+'), |_| ExprOperator::Add),
                    map(char('-'), |_| ExprOperator::Subtract),
                )),
            ),
            parse_term,
        ))),
    ))(input)
}

fn parse_term(input: &str) -> IResult<&str, ParsedTerm> {
    tuple((
        parse_factor,
        many0(tuple((
            preceded(
                skip_spaces,
                alt((
                    map(char('*'), |_| TermOperator::Multiply),
                    map(char('/'), |_| TermOperator::Divide),
                )),
            ),
            parse_factor,
        ))),
    ))(input)
}

fn parse_factor(input: &str) -> IResult<&str, ParsedFactor> {
    preceded(
        skip_spaces,
        alt((
            map(parse_identifier, ParsedFactor::Identifier),
            map(parse_literal, ParsedFactor::Literal),
            map(parse_subexpr, |expr| {
                ParsedFactor::SubExpression(Box::new(expr))
            }),
        )),
    )(input)
}

fn parse_literal(input: &str) -> IResult<&str, ParsedLiteral> {
    alt((
        map(parse_string, |v| ParsedLiteral::String(v)),
        map(double, ParsedLiteral::Number),
    ))(input)
}

fn parse_subexpr<'a>(input: &str) -> IResult<&str, ParsedExpr> {
    preceded(
        skip_spaces,
        delimited(
            char('('),
            preceded(skip_spaces, parse_expression),
            preceded(skip_spaces, char(')')),
        ),
    )(input)
}

fn parse_type(input: &str) -> IResult<&str, Type> {
    alt((
        map(tag("number"), |_| Type::Number),
        map(tag("string"), |_| Type::String),
    ))(input)
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    // TODO: remove keywords like string, number, (), etc
    alpha1(input)
}

fn skip_spaces(input: &str) -> IResult<&str, &str> {
    let chars = "\t \r \n";
    take_while(move |ch| chars.contains(ch))(input)
}

// #[cfg(test)]
// mod test {

//     use super::*;
//     use nom::error::ErrorKind;
//     use nom::error_position;

// #[test]
// fn valid_decleration() {
//     assert_eq!(
//         parse_decleration("@a number"),
//         Ok(("", ParsedStatement::Declaration("a", Types::Number)))
//     );
//     assert_eq!(
//         parse_decleration("@a string"),
//         Ok(("", ParsedStatement::Declaration("a", Types::String)))
//     );
// }
// #[test]
// fn invalid_decleration() {
//     assert_eq!(
//         parse_decleration("@a"),
//         Err(nom::Err::Error(error_position!("", ErrorKind::Tag)))
//     );
//     assert_eq!(
//         parse_decleration("@a invalid_type"),
//         Err(nom::Err::Error(error_position!(
//             "invalid_type",
//             ErrorKind::Tag
//         )))
//     );
// }
// #[test]
// fn valid_assignment() {
//     assert_eq!(
//         parse_assignment("a:=\"This is a string\""),
//         Ok((
//             "",
//             ParsedStatement::Assignment(
//                 "a",
//                 (
//                     (
//                         ParsedFactor::Literal(ParsedLiteral::String(String::from(
//                             "This is a string"
//                         ))),
//                         Vec::new()
//                     ),
//                     Vec::new()
//                 )
//             )
//         ))
//     );
//     assert_eq!(
//         parse_assignment("a:=2"),
//         Ok((
//             "",
//             ParsedStatement::Assignment(
//                 "a",
//                 (
//                     (ParsedFactor::Literal(ParsedLiteral::Number(2.)), Vec::new()),
//                     Vec::new()
//                 )
//             )
//         ))
//     );
// }

// #[test]
// fn invalid_assigment() {
//     match parse_assignment("a:=\"sdaasd") {
//         Ok(_) => {
//             panic!("Invalid assignemnt: missing semicolon\n")
//         }
//         Err(_) => (),
//     }
// }
// }
