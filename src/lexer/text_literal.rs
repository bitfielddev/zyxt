use tracing::trace;

use crate::{
    errors::ToZResult,
    lexer::buffer::Buffer,
    types::{
        position::Span,
        token::{Token, TokenType},
    },
    ZResult,
};

#[tracing::instrument(skip_all)]
pub fn lex_text_literal(iter: &mut Buffer, tokens: &mut Vec<Token>) -> ZResult<()> {
    let (_, init_pos) = iter.next().z()?.to_owned();
    let mut raw = "\"".to_owned();
    while let Some((char, pos)) = iter.next() {
        trace!(?char, ?pos);
        if *char == '"' {
            raw.push('"');
            tokens.push(Token {
                ty: Some(TokenType::LiteralString),
                value: (&raw).into(),
                span: Span::new(init_pos, &raw),
                ..Default::default()
            });
            return Ok(());
        }
        raw.push(*char);
    }
    Ok(())
}
