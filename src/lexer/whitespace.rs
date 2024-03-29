use tracing::trace;

use crate::{
    errors::ToZResult,
    lexer::{buffer::Buffer, WHITESPACE},
    types::{
        position::Span,
        token::{Token, TokenType},
    },
    ZResult,
};

#[tracing::instrument(skip_all)]
pub fn lex_whitespace(iter: &mut Buffer, tokens: &mut Vec<Token>) -> ZResult<()> {
    let mut raw = String::new();
    let Some((_, init_pos)) = iter.peek() else {
        return Ok(());
    };
    while let Some((char, pos)) = iter.peek() {
        trace!(?char, ?pos);
        if WHITESPACE.is_match(&char.to_string()) {
            raw.push(char);
            iter.next().z()?;
        } else {
            tokens.push(Token {
                ty: Some(TokenType::Whitespace),
                value: (&raw).into(),
                span: Span::new(init_pos, &raw),
                ..Default::default()
            });
            return Ok(());
        }
    }
    Ok(())
}

pub fn clean_whitespaces(input: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = vec![];
    let mut whitespace_stack = String::new();

    for mut t in input {
        if t.ty == Some(TokenType::Whitespace) {
            whitespace_stack += &*t.value;
        } else {
            t.whitespace = whitespace_stack.into();
            whitespace_stack = String::new();
            out.push(t);
        }
    }
    out
}
