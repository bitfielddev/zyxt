use crate::{lexer::buffer::Buffer, types::token::Token, ZyxtError};

pub fn lex_line_comment(iter: &mut Buffer, tokens: &mut [Token]) -> Result<(), ZyxtError> {
    let mut raw = "".to_string();
    while let Some((char, _)) = iter.next() {
        raw.push(*char);
        if *char == '\n' {
            tokens.last_mut().unwrap().value =
                format!("{}{raw}", tokens.last().unwrap().value).into();
            return Ok(());
        }
    }
    Ok(())
}

pub fn lex_block_comment(iter: &mut Buffer, tokens: &mut Vec<Token>) -> Result<(), ZyxtError> {
    let mut raw = "".to_string();
    while let Some((char, _)) = iter.next() {
        if *char == '*' {
            raw.push(*char);
            let (char, _) = iter.next().unwrap();
            raw.push(*char);
            if *char == '/' {
                tokens.last_mut().unwrap().value =
                    format!("{}{raw}", tokens.last().unwrap().value).into();
                return Ok(());
            }
        } else if *char == '/' {
            raw.push(*char);
            let (char, _) = iter.next().unwrap();
            if *char == '*' {
                tokens.last_mut().unwrap().value =
                    format!("{}{raw}", tokens.last().unwrap().value).into();
                raw = "".to_string();
                lex_block_comment(iter, tokens)?;
            } else {
                raw.push(*char);
            }
        } else {
            raw.push(*char);
        }
    }
    Ok(())
}
