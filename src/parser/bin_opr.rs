use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, BinaryOpr},
    errors::{ZError, ZResult},
    parser::buffer::Buffer,
    types::token::{Token, TokenType},
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_bin_opr(&mut self) -> ZResult<()> {
        self.reset_cursor();
        if self.content.is_empty() {
            return Ok(());
        }
        let mut highest_order_index: usize = 0;
        let mut highest_order = 0;
        let mut opr_ref = None;
        for (i, selected) in self.content.iter().enumerate() {
            let Either::Right(
                tok @ Token {
                    ty: Some(TokenType::BinaryOpr(opr_type)),
                    ..
                },
            ) = selected else {
                continue;
            };
            if i == 0 || i == self.content.len() - 1 {
                return Err(ZError::p006().with_span(selected));
            }
            if opr_type.order() >= highest_order {
                highest_order_index = i;
                highest_order = opr_type.order();
                opr_ref = Some((tok, opr_type));
            }
        }
        let Some((tok, opr_type)) = opr_ref else {
            return Ok(());
        };
        debug!(pos = ?tok.span, "Parsing binary operator");
        let operand1 = self
            .window(0..highest_order_index)
            .with_as_buffer(&Self::parse_as_expr)?;
        let operand2 = self
            .window(highest_order_index + 1..self.content.len())
            .with_as_buffer(&Self::parse_as_expr)?;
        let ele = Ast::BinaryOpr(BinaryOpr {
            ty: *opr_type,
            opr_span: Some(tok.span.to_owned()),
            operand1: operand1.into(),
            operand2: operand2.into(),
        });
        trace!(?ele);
        self.content = vec![Either::Left(ele)];
        Ok(())
    }
}
