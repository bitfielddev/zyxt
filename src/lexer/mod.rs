mod buffer;
mod comments;
mod number;
mod symbol;
mod text_literal;
mod whitespace;
mod word;

use std::sync::Arc;

use once_cell::sync::Lazy;
use regex::Regex;
use smol_str::SmolStr;
use tracing::{debug, trace};

use crate::{
    lexer::{
        buffer::Buffer,
        number::lex_number,
        symbol::lex_symbol,
        text_literal::lex_text_literal,
        whitespace::{clean_whitespaces, lex_whitespace},
        word::lex_word,
    },
    types::{errors::ZResult, position::Position, token::Token},
};

static ALPHANUMERIC: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());
static NUMERIC: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9]+$").unwrap());
static WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\s+$").unwrap());
static ALPHABETIC: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z_]+$").unwrap());

#[tracing::instrument(skip_all)]
pub fn lex(mut input: String, filename: SmolStr) -> ZResult<Vec<Token>> {
    if input.trim().is_empty() {
        return Ok(vec![]);
    };
    input.push('\n');

    let pos = Position {
        filename: Some(Arc::new(filename)),
        ..Default::default()
    };
    let mut iter = Buffer::new(&input, pos);
    let mut tokens = vec![];
    while let Some((char, pos)) = iter.to_owned().peek() {
        trace!(?char, ?pos);
        if char == '"' {
            debug!(?char, ?pos, "Text literal detected");
            lex_text_literal(&mut iter, &mut tokens)?;
        } else if ALPHABETIC.is_match(&char.to_string()) {
            debug!(?char, ?pos, "Word detected");
            lex_word(&mut iter, &mut tokens)?;
        } else if WHITESPACE.is_match(&char.to_string()) {
            debug!(?char, ?pos, "Whitespace detected");
            lex_whitespace(&mut iter, &mut tokens)?;
        } else if NUMERIC.is_match(&char.to_string()) {
            debug!(?char, ?pos, "Number detected");
            lex_number(&mut iter, &mut tokens)?;
        } else {
            debug!(?char, ?pos, "Symbol detected");
            lex_symbol(&mut iter, &mut tokens)?;
        }
    }
    debug!("Cleaning up whitespaces");
    tokens = clean_whitespaces(tokens);
    Ok(tokens)
}
