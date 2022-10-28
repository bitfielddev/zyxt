use itertools::{Either, Itertools};
use tracing::{debug, trace};

use crate::{
    parser::buffer::Buffer,
    types::{
        element::{binary_opr::BinaryOpr, Element, ElementVariant},
        errors::{ZError, ZResult},
        position::{GetPosRaw, PosRaw},
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
                return Err(
                    ZError::error_2_1_3(selected.pos_raw().raw).with_pos_raw(&selected.pos_raw())
                );
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
        debug!(pos = ?tok.pos, "Parsing binary operator");
        let operand1 = self
            .window(0..highest_order_index)
            .with_as_buffer(&|buf| buf.parse_as_expr())?;
        let operand2 = self
            .window(highest_order_index + 1..self.content.len())
            .with_as_buffer(&|buf| buf.parse_as_expr())?;
        let ele = Element {
            pos_raw: PosRaw {
                pos: operand1.pos_raw.pos.to_owned(),
                raw: self.content.iter().map(|a| a.pos_raw().raw).join("").into(),
            },
            data: Box::new(ElementVariant::BinaryOpr(BinaryOpr {
                ty: *opr_type,
                operand1,
                operand2,
            })),
        };
        trace!(?ele);
        self.content = vec![Either::Left(ele)];
        Ok(())
    }
}
