use itertools::Either;
use tracing::{debug, trace};

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{r#return::Return, Element},
        errors::ZResult,
        position::GetSpan,
        token::{Keyword, Token, TokenType},
        typeobj::unit_t::UNIT_T,
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_return(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            if !matches!(
                selected,
                Either::Right(Token {
                    ty: Some(TokenType::Keyword(Keyword::Return)),
                    ..
                })
            ) {
                continue;
            }
            let kwd_span = selected.span();
            debug!(pos = ?kwd_span, "Parsing return");
            let value = if self.next().is_some() {
                self.rest_incl_curr()
                    .with_as_buffer(&|buf| buf.parse_as_expr())?
            } else {
                UNIT_T.as_type().as_type_element().as_literal()
            }
            .into();

            let ele = Element::Return(Return {
                kwd_span: kwd_span,
                value,
            });
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: self.cursor - 1..self.content.len(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
