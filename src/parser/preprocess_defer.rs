use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{defer::Defer, preprocess::Preprocess, ElementVariant},
        position::GetPosRaw,
        token::{Keyword, TokenType},
    },
    Element, ZResult,
};

impl Buffer {
    pub fn parse_preprocess_defer(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let (selected, kwd) = if let Either::Right(selected) = selected {
                if let Some(TokenType::Keyword(kwd)) = &selected.ty {
                    if [Keyword::Defer, Keyword::Pre].contains(kwd) {
                        (selected.to_owned(), *kwd)
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let mut raw = selected.get_raw();
            let start = self.cursor;
            let init_pos_raw = selected.pos_raw();
            let selected = self.next_or_err()?;

            raw += &*selected.pos_raw().raw;
            let (content, end) = if let Either::Left(selected) = selected {
                if let ElementVariant::Block(_) = &*selected.data {
                    (selected.to_owned(), self.next_cursor_pos())
                } else {
                    (
                        self.rest_incl_curr()
                            .with_as_buffer(&|buffer| buffer.parse_as_block())?
                            .as_variant(),
                        self.content.len(),
                    )
                }
            } else {
                (
                    self.rest_incl_curr()
                        .with_as_buffer(&|buffer| buffer.parse_as_block())?
                        .as_variant(),
                    self.content.len(),
                )
            };
            let ele = Element {
                pos_raw: init_pos_raw,
                data: Box::new(if kwd == Keyword::Pre {
                    ElementVariant::Preprocess(Preprocess {
                        content: Element {
                            pos_raw: content.pos_raw,
                            data: Box::new(content.data.as_block().unwrap().to_owned()),
                        },
                    })
                } else {
                    ElementVariant::Defer(Defer {
                        content: Element {
                            pos_raw: content.pos_raw,
                            data: Box::new(content.data.as_block().unwrap().to_owned()),
                        },
                    })
                }),
            };
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: start..end,
            };
            self.splice_buffer(buffer_window);
        }
        Ok(())
    }
}
