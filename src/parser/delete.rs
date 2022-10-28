use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{delete::Delete, ident::Ident, unary_opr::UnaryOpr, Element, ElementVariant},
        errors::{ZError, ZResult},
        position::{GetPosRaw, PosRaw},
        token::{Keyword, OprType, Token, TokenType},
    },
};

impl Buffer {
    pub fn parse_delete(&mut self) -> ZResult<()> {
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
            let init_pos = selected.pos_raw().position;
            let start = self.cursor;
            self.start_raw_collection();
            let vars: Vec<Element<Ident>> =
                self.get_split(TokenType::Comma)?.with_as_buffers(&|buf| {
                    let ele = buf.parse_as_expr()?;
                    if let ElementVariant::Ident(data) = &*ele.data {
                        Ok(Element {
                            pos_raw: ele.pos_raw,
                            data: Box::new(data.to_owned()),
                        })
                    } else if let ElementVariant::UnaryOpr(UnaryOpr {
                        ty: OprType::Deref, ..
                    }) = *ele.data
                    {
                        Err(ZError::error_2_1_12(&ele.pos_raw.raw).with_element(&ele))
                    } else {
                        Err(ZError::error_2_1_11(&ele.pos_raw.raw).with_element(&ele))
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
                slice: vec![Either::Left(ele)],
                range: start..self.content.len(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
