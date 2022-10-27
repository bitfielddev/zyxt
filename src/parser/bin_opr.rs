use std::borrow::Cow;

use itertools::{Either, Itertools};

use crate::{
    parser::buffer::Buffer,
    types::{
        element::{binary_opr::BinaryOpr, Element, ElementVariant},
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{get_order, Token, TokenType},
    },
};

impl<'a> Buffer<'a> {
    pub fn parse_bin_opr(&mut self) -> Result<(), ZyxtError> {
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
                return Err(ZyxtError::error_2_1_3(selected.pos_raw().raw)
                    .with_pos_raw(&selected.pos_raw()));
            }
            if get_order(&opr_type) >= highest_order {
                highest_order_index = i;
                highest_order = get_order(&opr_type);
                opr_ref = Some((tok, opr_type))
            }
        }
        let (tok, opr_type) = if let Some(tot) = opr_ref {
            tot
        } else {
            return Ok(());
        };
        let ele = Element {
            pos_raw: PosRaw {
                position: tok.position.to_owned(),
                raw: self.content.iter().map(|a| a.pos_raw().raw).join("").into(),
            },
            data: Box::new(ElementVariant::BinaryOpr(BinaryOpr {
                ty: *opr_type,
                operand1: self
                    .window(0..highest_order_index)
                    .with_as_buffer(&|buf| buf.parse_as_expr())?,
                operand2: self
                    .window(highest_order_index + 1..self.content.len())
                    .with_as_buffer(&|buf| buf.parse_as_expr())?,
            })),
        };
        self.content = Cow::Owned(vec![Either::Left(ele)]);
        Ok(())
    }
}
