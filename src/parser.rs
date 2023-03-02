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

use crate::types::{StringLiteral, Types};

pub type ParsedProgram<'a> = Vec<ParsedStatement<'a>>;

#[derive(Debug)]
pub enum ParsedStatement<'a> {
    Declaration(&'a str, Types),
    InputOperation(&'a str),
    OutputOperation(ParsedRightSideValue<'a>),
    Assignment(&'a str, ParsedRightSideValue<'a>),
    // OutputOperation(ParsedExpr<'a>),
    // Assignment(&'a str, ParsedExpr<'a>),
}

pub type ParsedExpr<'a> = (ParsedTerm<'a>, Vec<(ExprOperator, ParsedTerm<'a>)>);

#[derive(Debug, Clone, Copy)]
pub enum ExprOperator {
    Add,
    Subtract,
}
pub type ParsedTerm<'a> = (ParsedFactor<'a>, Vec<(TermOperator, ParsedFactor<'a>)>);

#[derive(Debug, Clone, Copy)]
pub enum TermOperator {
    Multiply,
    Divide,
}

#[derive(Debug)]
pub enum ParsedFactor<'a> {
    Literal(f64),
    Identifier(&'a str),
    SubExpression(Box<ParsedExpr<'a>>),
}

#[derive(Debug)]
pub enum ParsedRightSideValue<'a> {
    Expression(ParsedExpr<'a>),
    String(StringLiteral),
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
    tuple((
        char('<'),
        skip_spaces,
        alt((parse_string, parse_expression)),
    ))(input)
    .map(|(input, output)| (input, ParsedStatement::OutputOperation(output.2)))
}

fn parse_assignment(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((
        parse_identifier,
        skip_spaces,
        tag(":="),
        skip_spaces,
        alt((parse_string, parse_expression)),
    ))(input)
    .map(|(input, output)| (input, ParsedStatement::Assignment(output.0, output.4)))
}

fn parse_expression(input: &str) -> IResult<&str, ParsedRightSideValue> {
    map(
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
        )),
        |v| ParsedRightSideValue::Expression(v),
    )(input)
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
            map(double, ParsedFactor::Literal),
            map(parse_subexpr, |expr| {
                ParsedFactor::SubExpression(Box::new(expr))
            }),
        )),
    )(input)
}

fn parse_subexpr<'a>(input: &str) -> IResult<&str, ParsedExpr> {
    map(
        delimited(
            preceded(skip_spaces, char('(')),
            parse_expression,
            preceded(skip_spaces, char('(')),
        ),
        |v| match v {
            ParsedRightSideValue::Expression(expr) => expr,

            ParsedRightSideValue::String(_) => {
                // return IResult::Err("Expected expression forund string");
                panic!("Expected expression forund string")
            }
        },
    )(input)
}

fn parse_type(input: &str) -> IResult<&str, Types> {
    alt((
        map(tag("number"), |_| Types::Number),
        map(tag("string"), |_| Types::String),
    ))(input)
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}
fn skip_spaces(input: &str) -> IResult<&str, &str> {
    let chars = "\t \r \n";
    take_while(move |ch| chars.contains(ch))(input)
}
