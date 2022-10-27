mod buffer;
mod comments;
mod number;
mod symbol;
mod text_literal;
mod whitespace;
mod word;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    lexer::{
        buffer::Buffer,
        number::lex_number,
        symbol::lex_symbol,
        text_literal::lex_text_literal,
        whitespace::{clean_whitespaces, lex_whitespace},
        word::lex_word,
    },
    types::{errors::ZyxtError, position::Position, token::Token},
};

static ALPHANUMERIC: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());
static NUMERIC: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+$").unwrap());
static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+$").unwrap());
static ALPHABETIC: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_]+$").unwrap());

pub fn lex(preinput: String, filename: &str) -> Result<Vec<Token>, ZyxtError> {
    if preinput.trim().is_empty() {
        return Ok(vec![]);
    };
    let input = preinput + "\n";

    let pos = Position {
        filename: filename.to_string(),
        ..Default::default()
    };
    let mut iter = Buffer::new(&input, pos);
    let mut tokens = vec![];
    while let Some((char, _)) = iter.to_owned().peek() {
        if char == "\"" {
            lex_text_literal(&mut iter, &mut tokens)?;
        } else if ALPHABETIC.is_match(char) {
            lex_word(&mut iter, &mut tokens)?;
        } else if WHITESPACE.is_match(char) {
            lex_whitespace(&mut iter, &mut tokens)?;
        } else if NUMERIC.is_match(char) {
            lex_number(&mut iter, &mut tokens)?;
        } else {
            lex_symbol(&mut iter, &mut tokens)?;
        }
    }
    tokens = clean_whitespaces(tokens);
    Ok(tokens)
}
