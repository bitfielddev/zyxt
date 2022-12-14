use tracing::trace;

use crate::{
    lexer::{buffer::Buffer, NUMERIC},
    types::{
        position::Span,
        token::{Token, TokenType},
    },
    ZResult,
};

#[tracing::instrument(skip_all)]
pub fn lex_number(iter: &mut Buffer, tokens: &mut Vec<Token>) -> ZResult<()> {
    let mut raw = String::new();
    let init_pos = iter.peek().unwrap().1;
    let mut dotted = false;
    while let Some((char, pos)) = iter.peek() {
        trace!(?char, ?pos);
        if NUMERIC.is_match(&char.to_string()) {
            raw.push(char);
            iter.next().unwrap();
        } else if char == '.' && !dotted {
            dotted = true;
            raw.push(char);
            iter.next().unwrap();
        } else {
            tokens.push(Token {
                ty: Some(TokenType::LiteralNumber),
                value: (&raw).into(),
                span: Span::new(init_pos, &raw),
                ..Default::default()
            });
            return Ok(());
        }
    }
    Ok(())
}
