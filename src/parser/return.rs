use std::borrow::Cow;

use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{
            delete::Delete, ident::Ident, r#return::Return, unary_opr::UnaryOpr, Element,
            ElementVariant,
        },
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{Keyword, OprType, TokenType},
    },
};

impl<'a> Buffer<'a> {
    fn parse_return(&mut self) -> Result<(), ZyxtError> {
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
            let mut value = self
                .rest_incl_curr()
                .with_as_buffer(&|buf| buf.parse_as_expr())?;
            let ele = Element {
                pos_raw: PosRaw {
                    position: selected.pos_raw().position,
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new(ElementVariant::Return(Return { value })),
            };
            let buffer_window = BufferWindow {
                slice: Cow::Owned(vec![Either::Left(ele)]),
                range: start..self.content.len(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
