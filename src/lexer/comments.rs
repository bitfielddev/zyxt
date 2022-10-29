use tracing::{debug, trace};

use crate::{lexer::buffer::Buffer, types::token::Token, ZResult};

#[tracing::instrument(skip_all)]
pub fn lex_line_comment(iter: &mut Buffer, tokens: &mut [Token]) -> ZResult<()> {
    let mut raw = "".to_string();
    while let Some((char, pos)) = iter.next() {
        trace!(?char, ?pos);
        raw.push(*char);
        if *char == '\n' {
            debug!("Ending line comment");
            tokens.last_mut().unwrap().value =
                format!("{}{raw}", tokens.last().unwrap().value).into();
            return Ok(());
        }
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn lex_block_comment(iter: &mut Buffer, tokens: &mut Vec<Token>) -> ZResult<()> {
    let mut raw = "".to_string();
    while let Some((char, pos)) = iter.next() {
        trace!(?char, ?pos);
        raw.push(*char);
        if *char == '*' {
            if let Some((char @ '/', _)) = iter.peek() {
                iter.next().unwrap();
                raw.push(char);
                debug!("Ending block comment");
                tokens.last_mut().unwrap().value =
                    format!("{}{raw}", tokens.last().unwrap().value).into();
                return Ok(());
            }
        } else if *char == '/' {
            if let Some((char @ '*', _)) = iter.peek() {
                iter.next().unwrap();
                raw.push(char);
                debug!("Detected nested block comment");
                tokens.last_mut().unwrap().value =
                    format!("{}{raw}", tokens.last().unwrap().value).into();
                raw = "".to_string();
                lex_block_comment(iter, tokens)?;
            }
        }
    }
    Ok(())
}
