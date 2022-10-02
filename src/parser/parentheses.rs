use itertools::Either;

use crate::{
    parser::buffer::Buffer,
    types::token::{TokenCategory, TokenType},
    ZyxtError,
};

impl<'a> Buffer<'a> {
    fn parse_parentheses(&mut self) -> Result<(), ZyxtError> {
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
                        .unwrap_or(vec![])
                        .contains(&TokenCategory::ValueEnd)
                    {
                        continue;
                    }
                    prev_ele
                } else {
                    continue;
                };
                if let Some(Either::Right(next_ele)) = self.peek() {
                    if next_ele.ty == Some(TokenType::CloseParen) {
                        todo!("Unit")
                    }
                }

                self.start_raw_collection();
                let mut paren_window =
                    self.get_between(TokenType::OpenParen, TokenType::CloseParen)?;
                let raw = self.end_raw_collection();
                paren_window.with_as_buffer(&|f| {
                    let mut ele = f.parse_as_expr()?;
                    ele.pos_raw.raw = raw.into();
                    Ok(ele)
                })?;
                self.splice_buffer(paren_window);
            } else if selected.ty == Some(TokenType::OpenCurlyParen) {
                // blocks, {
                self.start_raw_collection();
                let mut paren_window =
                    self.get_between(TokenType::OpenCurlyParen, TokenType::CloseCurlyParen)?;
                let raw = self.end_raw_collection();
                paren_window.with_as_buffer(&|f| {
                    let mut ele = f.parse_as_block()?;
                    ele.pos_raw.raw = raw.into();
                    Ok(ele)
                })?;
                self.splice_buffer(paren_window);
            }
        }

        Ok(())
    }
}
