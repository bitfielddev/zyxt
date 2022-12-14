use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{class::Class, Element},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        errors::{ZError, ZResult},
        position::GetSpan,
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
            let init_span = selected.span();
            debug!(pos = ?init_span, "Parsing");
            let start = self.cursor;
            let mut selected = self.next_or_err()?;
            let args = if let Either::Right(Token {
                ty: Some(TokenType::Bar),
                ..
            }) = selected
            {
                debug!(pos = ?selected.span(), "Argument list detected");
                if kwd == Keyword::Class {
                    return Err(ZError::error_2_1_17());
                }
                let args = self.parse_args()?;
                selected = self.next_or_err()?;
                Some(args)
            } else {
                None
            };
            let content = if let Either::Left(Element::Block(block)) = &selected {
                debug!(pos = ?selected.span(), "Block detected");
                Some(block.to_owned())
            } else if kwd == Keyword::Class {
                return Err(ZError::error_2_1_18(&kwd));
            } else {
                self.prev();
                None
            };
            let ele = Element::Class(Class {
                is_struct: kwd == Keyword::Struct,
                implementations: Default::default(),
                inst_fields: Default::default(), // TODO
                content,
                args,
            });
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
