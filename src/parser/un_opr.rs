use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, UnaryOpr},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        errors::ZResult,
        position::GetSpan,
        token::{Token, TokenType},
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_un_opr(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let opr_type = if let Either::Right(Token {
                ty: Some(TokenType::UnaryOpr(opr_type)),
                ..
            }) = selected
            {
                opr_type
            } else {
                continue;
            };
            let opr_span = selected.span();
            debug!(pos = ?opr_span);
            let operand = self
                .rest_incl_curr()
                .with_as_buffer(&|buf| buf.parse_as_expr())?
                .into();
            let ele = Ast::UnaryOpr(UnaryOpr {
                ty: opr_type,
                opr_span,
                operand,
            });
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
