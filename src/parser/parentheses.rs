use itertools::Either;
use tracing::{debug, trace};

use crate::{
    parser::buffer::Buffer,
    types::{
        element::ElementData,
        token::{TokenCategory, TokenType},
    },
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
                if let Some(Either::Right(prev_ele)) = self.peek_prev() {
                    if prev_ele
                        .ty
                        .map(|ty| ty.categories())
                        .unwrap_or_default()
                        .contains(&TokenCategory::ValueEnd)
                    {
                        debug!(pos = ?selected.span, prev_ty = ?prev_ele.ty, "Found value call");
                        continue;
                    }
                };
                if let Some(Either::Right(next_ele)) = self.peek() {
                    if next_ele.ty == Some(TokenType::CloseParen) {
                        continue;
                    }
                }
                debug!(pos = ?selected.span, "Parsing parentheses");

                let mut paren_window =
                    self.get_between(TokenType::OpenParen, TokenType::CloseParen)?;
                paren_window.with_as_buffer(&move |f| {
                    let ele = f.parse_as_expr()?;
                    trace!(?ele);
                    f.content = vec![Either::Left(ele)];
                    Ok(())
                })?;
                self.splice_buffer(paren_window);
            } else if selected.ty == Some(TokenType::OpenCurlyParen) {
                debug!(pos = ?selected.span, "Parsing curly braces");
                let paren_window =
                    self.get_between(TokenType::OpenCurlyParen, TokenType::CloseCurlyParen)?;
                let mut paren_window = self.window(paren_window.range); // TODO clean this up
                paren_window.with_as_buffer(&move |f| {
                    f.next_or_err()?;
                    let ele = f.parse_as_block()?;
                    trace!(?ele);
                    f.content = vec![Either::Left(ele.as_variant())];
                    Ok(())
                })?;
                self.splice_buffer(paren_window);
            }
        }

        Ok(())
    }
}
