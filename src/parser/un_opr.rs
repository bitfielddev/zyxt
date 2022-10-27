use std::borrow::Cow;

use itertools::{Either, Itertools};

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{unary_opr::UnaryOpr, Element, ElementVariant},
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{Token, TokenType},
    },
};

impl<'a> Buffer<'a> {
    pub fn parse_un_opr(&mut self) -> Result<(), ZyxtError> {
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
            let init_pos = selected.pos_raw().position;
            let operand = self
                .rest_incl_curr()
                .with_as_buffer(&|buf| buf.parse_as_expr())?;
            let ele = Element {
                pos_raw: PosRaw {
                    position: init_pos,
                    raw: self.content[self.cursor - 1..]
                        .iter()
                        .map(|a| a.pos_raw().raw)
                        .join("")
                        .into(),
                },
                data: Box::new(ElementVariant::UnaryOpr(UnaryOpr {
                    ty: *opr_type,
                    operand,
                })),
            };
            let buffer_window = BufferWindow {
                slice: Cow::Owned(vec![Either::Left(ele)]),
                range: self.cursor - 1..self.content.len(),
            };
            self.splice_buffer(buffer_window);
        }
        Ok(())
    }
}
