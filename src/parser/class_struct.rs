use itertools::Either;

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
            let start = self.cursor;
            self.start_raw_collection();
            let mut selected = self.next_or_err()?;
            let args = if let Either::Right(Token {
                ty: Some(TokenType::Bar),
                ..
            }) = selected
            {
                if kwd == Keyword::Class {
                    return Err(ZError::error_2_1_17().with_pos_raw(&selected.pos_raw()));
                }
                let args = self.parse_args()?;
                selected = self.next_or_err()?;
                Some(args)
            } else {
                None
            };
            let content = if let Either::Left(Element {
                data: box ElementVariant::Block(block),
                ..
            }) = selected
            {
                Some(block)
            } else if kwd == Keyword::Class {
                return Err(ZError::error_2_1_18(&kwd).with_pos_raw(&selected.pos_raw()));
            } else {
                None
            };
            let ele = Element {
                pos_raw: PosRaw {
                    position: init_pos_raw.position.to_owned(),
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new({
                    ElementVariant::Class(Class {
                        is_struct: kwd == Keyword::Class,
                        implementations: Default::default(),
                        inst_fields: Default::default(), // TODO
                        content: content.map(|block| Element {
                            pos_raw: init_pos_raw,
                            data: Box::new(block),
                        }),
                        args,
                    })
                }),
            };
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: start..self.next_cursor_pos(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
