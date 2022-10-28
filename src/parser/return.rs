use itertools::Either;
use tracing::{debug, trace};

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{r#return::Return, Element, ElementVariant},
        errors::ZResult,
        position::{GetPosRaw, PosRaw},
        token::{Keyword, Token, TokenType},
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
            debug!(pos = ?selected.pos_raw().pos);
            let value = self
                .rest_incl_curr()
                .with_as_buffer(&|buf| buf.parse_as_expr())?;
            let ele = Element {
                pos_raw: PosRaw {
                    pos: selected.pos_raw().pos,
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new(ElementVariant::Return(Return { value })),
            };
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: self.cursor..self.content.len(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
