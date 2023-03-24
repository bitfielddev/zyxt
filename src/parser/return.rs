use std::sync::Arc;

use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, Return},
    errors::ZResult,
    parser::buffer::{Buffer, BufferWindow},
    primitives::UNIT_T,
    types::{
        position::GetSpan,
        token::{Keyword, Token, TokenType},
        value::Value,
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
                self.rest_incl_curr().with_as_buffer(&Self::parse_as_expr)?
            } else {
                Value::Unit.as_ast().into()
            }
            .into();

            let ele = Ast::Return(Return { kwd_span, value });
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: self.cursor - 1..self.content.len(),
            };
            self.splice_buffer(buffer_window);
        }
        Ok(())
    }
}
