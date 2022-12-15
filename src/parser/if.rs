use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, Condition, If},
    errors::{ZError, ZResult},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        position::GetSpan,
        token::{Keyword, Token, TokenType},
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_if(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let Either::Right(Token {
                ty: Some(TokenType::Keyword(kwd)),
                ..
            }) = selected else {
                continue;
            };
            if [Keyword::Elif, Keyword::Else].contains(&kwd) {
                return Err(
                    ZError::p016(if kwd == Keyword::Elif { "elif" } else { "else" })
                        .with_span(selected),
                );
            } else if kwd != Keyword::If {
                continue;
            };

            let init_pos = selected.span();
            debug!(pos = ?init_pos, "Parsing if");
            let start = self.cursor;
            let mut conditions: Vec<Condition> = vec![];
            let mut prev_kwd = Keyword::If;
            self.prev()?;
            while let Some(mut selected) = self.next() {
                let kwd = if let Either::Right(Token {
                    ty: Some(TokenType::Keyword(prekwd)),
                    ..
                }) = selected
                {
                    match prekwd {
                        Keyword::If if self.cursor == start => Keyword::If,
                        Keyword::Elif if prev_kwd != Keyword::Else => Keyword::Elif,
                        Keyword::Else if prev_kwd != Keyword::Else => Keyword::Else,
                        Keyword::Elif if prev_kwd == Keyword::Else => {
                            return Err(ZError::p017("elif").with_span(selected))
                        }
                        Keyword::Else if prev_kwd == Keyword::Else => {
                            return Err(ZError::p017("else").with_span(selected))
                        }
                        _ => break,
                    }
                } else {
                    break;
                };
                debug!(?kwd, pos = ?selected.span(), "Parsing condition");
                prev_kwd = kwd;
                selected = self.next_or_err()?;
                let condition = if kwd == Keyword::Else {
                    None
                } else if let Either::Left(ele @ Ast::Block(_)) = &selected {
                    debug!(pos = ?ele.span(), "Detected condition expr in {{}}");
                    Some(ele.to_owned())
                } else {
                    debug!(pos = ?selected.span(), "Detected condition expr not in {{}}");
                    let start = self.cursor;
                    loop {
                        let selected = self.next_or_err()?;
                        if matches!(selected, Either::Left(Ast::Block(_))) {
                            break;
                        }
                    }
                    self.prev()?;

                    selected = self.next_or_err()?;
                    Some(
                        self.window(start..self.cursor)
                            .with_as_buffer(&Self::parse_as_expr)?,
                    )
                };
                let block = if let Either::Left(Ast::Block(block)) = &selected {
                    debug!(pos = ?selected.span(), "Detected block");
                    block.to_owned()
                } else {
                    return Err(ZError::p018().with_span(selected));
                };
                conditions.push(Condition {
                    kwd_span: None, // TODO
                    condition,
                    if_true: block.to_owned(),
                });
            }
            self.prev()?;
            let ele = Ast::If(If { conditions });
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: start..self.next_cursor_pos(),
            };
            self.splice_buffer(buffer_window);
        }
        Ok(())
    }
}
