use tracing::{debug, trace};

use crate::{errors::ToZResult, lexer::buffer::Buffer, types::token::Token, ZResult};

#[tracing::instrument(skip_all)]
pub fn lex_line_comment(iter: &mut Buffer, tokens: &mut [Token]) -> ZResult<()> {
    let mut raw = String::new();
    while let Some((char, pos)) = iter.next() {
        trace!(?char, ?pos);
        raw.push(*char);
        if *char == '\n' {
            debug!("Ending line comment");
            tokens.last_mut().z()?.value = format!("{}{raw}", tokens.last().z()?.value).into();
            return Ok(());
        }
    }
    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn lex_block_comment(iter: &mut Buffer, tokens: &mut Vec<Token>) -> ZResult<()> {
    let mut raw = String::new();
    while let Some((char, pos)) = iter.next() {
        trace!(?char, ?pos);
        raw.push(*char);
        if *char == '*' {
            if let Some((char @ '/', _)) = iter.peek() {
                iter.next().z()?;
                raw.push(char);
                debug!("Ending block comment");
                tokens.last_mut().z()?.value = format!("{}{raw}", tokens.last().z()?.value).into();
                return Ok(());
            }
        } else if *char == '/' {
            if let Some((char @ '*', _)) = iter.peek() {
                iter.next().z()?;
                raw.push(char);
                debug!("Detected nested block comment");
                tokens.last_mut().z()?.value = format!("{}{raw}", tokens.last().z()?.value).into();
                raw = String::new();
                lex_block_comment(iter, tokens)?;
            }
        }
    }
    Ok(())
}
