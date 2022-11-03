use itertools::Either;
use tracing::{debug, trace};

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{class::Class, Element, ElementVariant},
        errors::{ZError, ZResult},
        position::{GetPosRaw, PosRaw},
        token::{Keyword, Token, TokenType},
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_class_struct(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let kwd = if let Either::Right(selected) = &selected {
                if let Some(TokenType::Keyword(kwd)) = &selected.ty {
                    if [Keyword::Class, Keyword::Struct].contains(kwd) {
                        *kwd
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let init_pos_raw = selected.pos_raw();
            debug!(pos = ?init_pos_raw.pos, "Parsing");
            let start = self.cursor;
            self.start_raw_collection();
            let mut selected = self.next_or_err()?;
            let args = if let Either::Right(Token {
                ty: Some(TokenType::Bar),
                ..
            }) = selected
            {
                debug!(pos = ?selected.pos_raw().pos, "Argument list detected");
                if kwd == Keyword::Class {
                    return Err(ZError::error_2_1_17().with_pos_raw(&selected.pos_raw()));
                }
                let args = self.parse_args()?;
                selected = self.next_or_err()?;
                Some(args)
            } else {
                None
            };
            let content_pos_raw = selected.pos_raw();
            let content = if let Either::Left(Element {
                data: box ElementVariant::Block(block),
                pos_raw,
            }) = selected
            {
                debug!(pos = ?pos_raw.pos, "Block detected");
                Some(block)
            } else if kwd == Keyword::Class {
                return Err(ZError::error_2_1_18(&kwd).with_pos_raw(&selected.pos_raw()));
            } else {
                self.prev();
                None
            };
            let ele = Element {
                pos_raw: PosRaw {
                    pos: init_pos_raw.pos.to_owned(),
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new({
                    ElementVariant::Class(Class {
                        is_struct: kwd == Keyword::Struct,
                        implementations: Default::default(),
                        inst_fields: Default::default(), // TODO
                        content: content.map(|block| Element {
                            pos_raw: content_pos_raw,
                            data: Box::new(block),
                        }),
                        args,
                    })
                }),
            };
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: start..self.next_cursor_pos(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
