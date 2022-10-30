use itertools::Either;
use tracing::{debug, trace};

use crate::{
    parser::buffer::Buffer,
    types::token::{TokenCategory, TokenType},
    ZResult,
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_parentheses(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let selected = if let Either::Right(selected) = selected {
                selected
            } else {
                continue;
            };

            if selected.ty == Some(TokenType::OpenParen) {
                if let Some(Either::Right(prev_ele)) = self.prev() {
                    if prev_ele
                        .ty
                        .map(|ty| ty.categories())
                        .unwrap_or_default()
                        .contains(&TokenCategory::ValueEnd)
                    {
                        debug!(pos = ?selected.pos, prev_ty = ?prev_ele.ty, "Found value call");
                        continue;
                    }
                };
                if let Some(Either::Right(next_ele)) = self.peek() {
                    if next_ele.ty == Some(TokenType::CloseParen) {
                        continue;
                    }
                }
                debug!(pos = ?selected.pos, "Parsing parentheses");

                self.start_raw_collection();
                let mut paren_window =
                    self.get_between(TokenType::OpenParen, TokenType::CloseParen)?;
                let raw = self.end_raw_collection();
                paren_window.with_as_buffer(&move |f| {
                    let mut ele = f.parse_as_expr()?;
                    ele.pos_raw.raw = raw.to_owned().into();
                    trace!(?ele);
                    f.content = vec![Either::Left(ele)];
                    Ok(())
                })?;
                self.splice_buffer(paren_window);
            } else if selected.ty == Some(TokenType::OpenCurlyParen) {
                debug!(pos = ?selected.pos, "Parsing curly braces");
                self.start_raw_collection();
                let paren_window =
                    self.get_between(TokenType::OpenCurlyParen, TokenType::CloseCurlyParen)?;
                let mut paren_window = self.window(paren_window.range); // TODO clean this up
                let raw = self.end_raw_collection();
                paren_window.with_as_buffer(&move |f| {
                    f.next_or_err()?;
                    let mut ele = f.parse_as_block()?;
                    trace!(?ele);
                    ele.pos_raw.raw = raw.to_owned().into();
                    f.content = vec![Either::Left(ele.as_variant())];
                    Ok(())
                })?;
                self.splice_buffer(paren_window);
            }
        }

        Ok(())
    }
}
