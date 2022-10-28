use crate::{
    lexer::{buffer::Buffer, WHITESPACE},
    types::token::{Token, TokenType},
    ZyxtError,
};

pub fn lex_whitespace(iter: &mut Buffer, tokens: &mut Vec<Token>) -> Result<(), ZyxtError> {
    let mut raw = "".to_string();
    let pos = if let Some((_, pos)) = iter.peek() {
        pos
    } else {
        return Ok(());
    };
    while let Some((char, _)) = iter.peek() {
        if WHITESPACE.is_match(&char.to_string()) {
            raw.push(char);
            iter.next().unwrap();
        } else {
            tokens.push(Token {
                ty: Some(TokenType::Whitespace),
                value: raw.into(),
                position: pos,
                ..Default::default()
            });
            return Ok(());
        }
    }
    Ok(())
}

pub fn clean_whitespaces(input: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = vec![];
    let mut whitespace_stack = "".to_string();

    for mut t in input {
        if t.ty != Some(TokenType::Whitespace) {
            t.whitespace = whitespace_stack.into();
            whitespace_stack = "".into();
            out.push(t);
        } else {
            whitespace_stack += &*t.value;
        }
    }
    out
}
