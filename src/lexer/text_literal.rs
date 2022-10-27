use crate::{
    lexer::buffer::Buffer,
    types::token::{Token, TokenType},
    ZyxtError,
};

pub fn lex_text_literal(iter: &mut Buffer, tokens: &mut Vec<Token>) -> Result<(), ZyxtError> {
    iter.next().unwrap();
    let mut raw = "\"".to_string();
    let pos = iter.peek().ok_or_else(|| todo!())?.1;
    while let Some((char, _)) = iter.next() {
        if *char == '"' {
            raw.push('"');
            tokens.push(Token {
                ty: Some(TokenType::LiteralString),
                value: raw.into(),
                position: pos,
                ..Default::default()
            });
            return Ok(());
        } else if *char == '\\' {
            if let Some((char, _)) = iter.next() {
                let new = match *char {
                    '"' => '"',
                    '\\' => '\\',
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t', // TODO more escapes
                    _ => {
                        raw.push('\\');
                        *char
                    }
                };
                raw.push(new);
            } else {
                todo!()
            }
        } else {
            raw.push(*char);
        }
    }
    Ok(())
}
