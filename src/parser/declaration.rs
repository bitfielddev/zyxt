use std::borrow::Cow;

use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{declare::Declare, Element, ElementVariant},
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{Token, TokenType},
    },
};

impl<'a> Buffer<'a> {
    pub(crate) fn parse_declaration(&mut self) -> Result<(), ZyxtError> {
        self.reset_cursor();
        let mut flag_pos = None;
        let mut start;
        while let Some(selected) = self.next() {
            if matches!(
                selected,
                Either::Right(Token {
                    ty: Some(TokenType::Flag(_)),
                    ..
                })
            ) {
                flag_pos = Some(self.cursor);
                self.start_raw_collection();
                start = self.cursor;
                continue;
            } else if !matches!(
                selected,
                Either::Right(Token {
                    ty: Some(TokenType::DeclarationOpr),
                    ..
                })
            ) {
                continue;
            }
            let declared_var = if let Some(Either::Left(d)) = self.prev() {
                d
            } else {
                return Err(ZyxtError::error_2_1_5().with_pos_raw(&selected.pos_raw()));
            };

            self.cursor -= 1;
            self.start_raw_collection();
            start = self.cursor;
            self.cursor += 1;

            let flags = if let Some(flag_pos) = flag_pos {
                self.content[flag_pos..self.cursor - 1]
                    .iter()
                    .map(|ele| {
                        if let Either::Right(Token {
                            ty: Some(TokenType::Flag(flag)),
                            ..
                        }) = ele
                        {
                            Ok(flag.to_owned())
                        } else {
                            return Err(ZyxtError::error_2_1_6(ele.pos_raw().raw)
                                .with_pos_raw(&ele.pos_raw()));
                        }
                    })
                    .collect::<Result<_, _>>()?
            } else {
                vec![]
            };
            self.next_or_err()?;
            let content = self
                .rest_incl_curr()
                .with_as_buffer(&|buf| buf.parse_as_expr())?;
            let ele = Element {
                pos_raw: PosRaw {
                    position: self.content[start].pos_raw().position,
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new(ElementVariant::Declare(Declare {
                    variable: declared_var.to_owned(),
                    content,
                    flags,
                    ty: None,
                })),
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
