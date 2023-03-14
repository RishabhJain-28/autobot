use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    character::complete::char,
    combinator::map,
    combinator::opt,
    multi::many1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

use super::{parse_identifier, parse_program, skip_spaces, ParsedProgram, ParsedStatement};
use crate::runtime::keyboard::KeyModes;

#[derive(Debug, PartialEq)]
pub struct ParsedShortcut<'a>(pub ParsedShortcutHead<'a>, pub ParsedShortcutBody<'a>);
#[derive(Debug, PartialEq)]
pub struct ParsedShortcutHead<'a>(pub ParsedModes, pub ParsedKey, pub ParsedFlag, pub &'a str);
#[derive(Debug, PartialEq)]
// TODO : shortcuts are right no nestable? is that okay
pub struct ParsedShortcutBody<'a>(pub Box<ParsedProgram<'a>>);
#[derive(Debug, PartialEq)]
pub struct ParsedModes(pub Vec<KeyModes>);
#[derive(Debug, PartialEq)]

//TODO : replace char with enum
pub struct ParsedKey(pub char);
#[derive(Debug, PartialEq)]

pub struct ParsedFlag(pub bool);

pub fn parse_shortcut(input: &str) -> IResult<&str, ParsedStatement> {
    tuple((tag("on"), parse_shortcut_head, parse_shortcut_body))(input).map(|(input, output)| {
        (
            input,
            ParsedStatement::Shortcut(ParsedShortcut(output.1, output.2)),
        )
    })
}

pub fn parse_shortcut_head(input: &str) -> IResult<&str, ParsedShortcutHead> {
    preceded(
        skip_spaces,
        tuple((
            parse_modes,
            parse_key,
            parse_flag,
            skip_spaces,
            char(':'),
            skip_spaces,
            parse_identifier,
        )),
    )(input)
    .map(|(input, output)| {
        (
            input,
            ParsedShortcutHead(output.0, output.1, output.2, output.6),
        )
    })
}

pub fn parse_modes(input: &str) -> IResult<&str, ParsedModes> {
    many1(preceded(
        skip_spaces,
        alt((
            map(tag("alt"), |_| KeyModes::ALT),
            map(tag("ctrl"), |_| KeyModes::CTRL),
            map(tag("shift"), |_| KeyModes::SHIFT),
        )),
    ))(input)
    .map(|(input, output)| (input, ParsedModes(output)))
}

pub fn parse_key(input: &str) -> IResult<&str, ParsedKey> {
    //TODO :FIX => make parse_alpha, parse_didgit, add error ahndling
    preceded(skip_spaces, map(anychar, |c| ParsedKey(c)))(input)
}
pub fn parse_flag(input: &str) -> IResult<&str, ParsedFlag> {
    preceded(
        skip_spaces,
        map(opt(tag("on_hold_repeat")), |res| {
            if res.is_none() {
                ParsedFlag(false)
            } else {
                ParsedFlag(true)
            }
        }),
    )(input)
}
pub fn parse_shortcut_body(input: &str) -> IResult<&str, ParsedShortcutBody> {
    delimited(
        preceded(skip_spaces, char('{')),
        map(preceded(skip_spaces, parse_program), |prog| {
            ParsedShortcutBody(Box::new(prog))
        }),
        preceded(skip_spaces, char('}')),
    )(input)
}
