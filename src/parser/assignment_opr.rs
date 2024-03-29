use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, AstData, BinaryOpr, Set},
    errors::{ZError, ZResult},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        position::GetSpan,
        token::{Token, TokenType},
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_assignment_opr(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let (opr_type, opr_span) = if let Either::Right(Token {
                ty: Some(TokenType::AssignmentOpr(opr_type)),
                span: opr_span,
                ..
            }) = &selected
            {
                (*opr_type, opr_span.to_owned())
            } else {
                continue;
            };
            debug!(pos = ?selected.span(), "Parsing assignment operator");
            let var = if let Some(Either::Left(var)) = self.peek_prev() {
                var.to_owned()
            } else {
                return Err(ZError::p004().with_span(opr_span));
            };
            self.next_or_err()?;
            let mut content = self.rest_incl_curr().with_as_buffer(&|buf| {
                if buf.content.is_empty() {
                    return Err(ZError::p005().with_span(&opr_span));
                }
                buf.parse_as_expr()
            })?;
            if let Some(opr_type) = opr_type {
                debug!(?opr_type, "Desugaring");
                content = BinaryOpr {
                    ty: opr_type,
                    opr_span: None,
                    operand1: var.to_owned().into(),
                    operand2: content.into(),
                }
                .as_variant();
            }
            let ele = Ast::Set(Set {
                variable: var.to_owned().into(),
                eq_span: Some(opr_span),
                content: content.into(),
            });
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: self.cursor - 2..self.content.len(),
            };
            self.splice_buffer(buffer_window);
        }
        Ok(())
    }
}
