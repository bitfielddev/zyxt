use std::borrow::Cow;

use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{delete::Delete, ident::Ident, unary_opr::UnaryOpr, Element, ElementVariant},
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{OprType, TokenType},
    },
};

impl<'a> Buffer<'a> {
    fn parse_delete(&mut self) -> Result<(), ZyxtError> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            if !matches!(
                selected,
                Either::Right(Token {
                    ty: Some(TokenType::Keyword(Keyword::Delete)),
                    ..
                })
            ) {
                continue;
            }
            let start = self.cursor;
            let init_pos = selected.pos_raw().position;
            self.start_raw_collection();
            let vars: Vec<Element<Ident>> =
                self.get_split(TokenType::Comma)?.with_as_buffers(&|buf| {
                    let ele = buf.parse_as_expr()?;
                    if let Some(data) = ele.data.as_ident() {
                        Ok(Element {
                            pos_raw: ele.pos_raw,
                            data,
                        })
                    } else if let ElementVariant::UnaryOpr(UnaryOpr {
                        ty: OprType::Deref, ..
                    }) = ele
                    {
                        Err(ZyxtError::error_2_1_12(raw.to_owned()).with_element(&ele))
                    } else {
                        Err(ZyxtError::error_2_1_11(ele.pos_raw.raw).with_element(&ele))
                    }
                })?;
            let ele = Element {
                pos_raw: PosRaw {
                    position: init_pos,
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new(ElementVariant::Delete(Delete { names: vars })),
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
