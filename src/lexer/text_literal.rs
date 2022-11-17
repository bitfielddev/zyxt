use tracing::trace;

use crate::{
    lexer::buffer::Buffer,
    types::token::{Token, TokenType},
    ZResult,
};

#[tracing::instrument(skip_all)]
pub fn lex_text_literal(iter: &mut Buffer, tokens: &mut Vec<Token>) -> ZResult<()> {
    iter.next().unwrap();
    let mut raw = "\"".to_string();
    let init_pos = iter.peek().ok_or_else(|| todo!())?.1;
    while let Some((char, pos)) = iter.next() {
        trace!(?char, ?pos);
        if *char == '"' {
            raw.push('"');
            tokens.push(Token {
                ty: Some(TokenType::LiteralString),
                value: raw.into(),
                pos: init_pos,
                ..Default::default()
            });
            return Ok(());
        } else {
            raw.push(*char);
        }
        /*if *char == '\\' { TODO move this to parsing
            if let Some((char, pos)) = iter.next() {
                trace!(?char, ?pos);
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
        }
        else*/
    }
    Ok(())
}
