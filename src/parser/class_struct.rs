use std::borrow::Cow;

use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{block::Block, class::Class, Element, ElementVariant},
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{Keyword, Token, TokenType},
    },
};

impl<'a> Buffer<'a> {
    fn parse_class_struct(&mut self) -> Result<(), ZyxtError> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let kwd = if let Either::Right(selected) = selected {
                if let Some(TokenType::Keyword(kwd)) = &selected.ty {
                    if [Keyword::Class, Keyword::Struct].contains(kwd) {
                        kwd
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let start = self.cursor;
            let init_pos_raw = selected.pos_raw();
            self.start_raw_collection();
            let mut selected = self.next_or_err()?;
            if let Either::Right(Token {
                ty: Some(TokenType::Bar),
                ..
            }) = selected
            {
                if *kwd == Keyword::Class {
                    return Err(ZyxtError::error_2_1_17().with_pos_raw(&selected.pos_raw()));
                }
                // TODO get_arguments
                selected = self.next_or_err()?;
            }
            let content = if let Either::Left(Element {
                data: Box(ElementVariant::Block(block), ..),
                ..
            }) = selected
            {
                Some(block)
            } else if *kwd == Keyword::Class {
                return Err(ZyxtError::error_2_1_18(keyword).with_pos_raw(&selected.pos_raw()));
            } else {
                None
            };
            let ele = Element {
                pos_raw: PosRaw {
                    position: init_pos,
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new({
                    ElementVariant::Class(Class {
                        is_struct: *kwd == Keyword::Class,
                        implementations: Default::default(),
                        inst_fields: Default::default(), // TODO
                        content: content.map(|block| Element {
                            pos_raw: init_pos_raw,
                            data: Box::new(block.to_owned()),
                        }),
                        args: None,
                    })
                }),
            };
            let buffer_window = BufferWindow {
                slice: Cow::Owned[vec![Either::Left(ele)]],
                range: start..self.next_cursor_pos(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}