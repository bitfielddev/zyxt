use std::borrow::Cow;

use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{call::Call, Element, ElementVariant},
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{Token, TokenType},
    },
};

impl Buffer {
    pub fn parse_unparen_call(&mut self) -> Result<(), ZyxtError> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            if self.cursor == self.content.len() - 1 {
                continue;
            }
            let function = if let Either::Left(selected) = &selected {
                selected.to_owned()
            } else {
                continue;
            };

            let init_pos = selected.pos_raw().position;
            self.start_raw_collection();
            let mut args = vec![];
            let mut start = self.cursor + 1;
            while let Some(selected) = self.next() {
                if matches!(
                    selected,
                    Either::Right(Token {
                        ty: Some(TokenType::Comma),
                        ..
                    })
                ) {
                    if start == self.cursor {
                        todo!("error")
                    }
                    args.push(
                        self.window(start..self.cursor)
                            .with_as_buffer(&|buf| buf.parse_as_expr())?,
                    );
                    start = self.cursor + 1
                }
            }
            if matches!(
                self.content.last(),
                Some(Either::Right(Token {
                    ty: Some(TokenType::Comma),
                    ..
                }))
            ) {
                todo!("error")
            }
            args.push(
                self.window(start..self.cursor)
                    .with_as_buffer(&|buf| buf.parse_as_expr())?,
            );
            let ele = Element {
                pos_raw: PosRaw {
                    position: init_pos.to_owned(),
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new(ElementVariant::Call(Call {
                    called: function.to_owned(),
                    args,
                    kwargs: Default::default(),
                })),
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
