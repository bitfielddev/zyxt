use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, BinaryOpr},
    parser::buffer::Buffer,
    types::{
        errors::ZResult,
        token::{Token, TokenType},
    },
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
            let (tok, opr_type) = if let Either::Right(
                tok @ Token {
                    ty: Some(TokenType::BinaryOpr(opr_type)),
                    ..
                },
            ) = selected
            {
                (tok, opr_type)
            } else {
                continue;
            };
            if i == 0 || i == self.content.len() - 1 {
                todo!();
                // return Err(
                //     ZError::error_2_1_3(selected.span().raw)
                // );
            }
            if opr_type.order() >= highest_order {
                highest_order_index = i;
                highest_order = opr_type.order();
                opr_ref = Some((tok, opr_type))
            }
        }
        let (tok, opr_type) = if let Some(tot) = opr_ref {
            tot
        } else {
            return Ok(());
        };
        debug!(pos = ?tok.span, "Parsing binary operator");
        let operand1 = self
            .window(0..highest_order_index)
            .with_as_buffer(&|buf| buf.parse_as_expr())?;
        let operand2 = self
            .window(highest_order_index + 1..self.content.len())
            .with_as_buffer(&|buf| buf.parse_as_expr())?;
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
