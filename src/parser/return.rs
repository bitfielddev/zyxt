use itertools::{Either, Itertools};
use tracing::{debug, trace};

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{r#return::Return, Element, ElementVariant},
        errors::ZResult,
        position::{GetPosRaw, PosRaw},
        token::{Keyword, Token, TokenType},
        typeobj::unit_t::UNIT_T,
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_return(&mut self) -> ZResult<()> {
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
            let init_pos = selected.pos_raw().pos;
            debug!(pos = ?init_pos, "Parsing return");
            let value = if self.next().is_some() {
                self.rest_incl_curr()
                    .with_as_buffer(|buf| buf.parse_as_expr())?
            } else {
                UNIT_T.as_type().as_type_element().as_literal()
            };

            let ele = Element {
                pos_raw: PosRaw {
                    pos: init_pos,
                    raw: self.content[self.cursor - 1..self.content.len()]
                        .iter()
                        .map(|a| a.pos_raw().raw)
                        .join("")
                        .into(),
                },
                data: Box::new(ElementVariant::Return(Return { value })),
            };
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: self.cursor - 1..self.content.len(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
