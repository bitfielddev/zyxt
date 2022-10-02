use crate::{
    lexer::{buffer::Buffer, NUMERIC},
    types::token::{Token, TokenType},
    ZyxtError,
};

pub fn lex_number(iter: &mut Buffer, tokens: &mut Vec<Token>) -> Result<(), ZyxtError> {
    let mut raw = "".to_string();
    let pos = iter.peek().unwrap().1;
    let mut dotted = false;
    while let Some((char, _)) = iter.peek() {
        if NUMERIC.is_match(char) {
            raw.push_str(char);
            iter.next().unwrap();
        } else if char == "." && !dotted {
            dotted = true;
            raw.push_str(char);
            iter.next().unwrap();
        } else {
            tokens.push(Token {
                ty: Some(TokenType::LiteralNumber),
                value: raw.into(),
                position: pos,
                ..Default::default()
            });
            return Ok(());
        }
    }
    Ok(())
}
