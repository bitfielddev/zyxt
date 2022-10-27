use std::borrow::Cow;

use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{
            r#if::{Condition, If},
            Element, ElementVariant,
        },
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{Keyword, Token, TokenType},
    },
};

impl<'a> Buffer<'a> {
    fn parse_if(&mut self) -> Result<(), ZyxtError> {
        self.reset_cursor();
        while let Some(mut selected) = self.next() {
            let kwd = if let Either::Right(Token {
                ty: Some(TokenType::Keyword(kwd)),
                ..
            }) = selected
            {
                kwd
            } else {
                continue;
            };
            if [Keyword::Elif, Keyword::Else].contains(kwd) {
                return Err(ZyxtError::error_2_1_9(
                    if *kwd == Keyword::Elif {
                        "elif"
                    } else {
                        "else"
                    }
                    .to_string(),
                )
                .with_pos_raw(&selected.pos_raw()));
            } else if *kwd != Keyword::If {
                continue;
            };

            let init_pos = selected.pos_raw().position;
            let mut conditions: Vec<Condition> = vec![];
            let mut prev_kwd = Keyword::If;
            self.start_raw_collection();
            while let mut selected = self.next_or_err()? {
                let kwd = if let Either::Right(Token {
                    ty: Some(TokenType::Keyword(prekwd)),
                    ..
                }) = catcher_selected
                {
                    match prekwd {
                        Keyword::If if *position == start_pos => Keyword::If,
                        Keyword::Elif if prev_kwd != Keyword::Else => Keyword::Elif,
                        Keyword::Else if prev_kwd != Keyword::Else => Keyword::Else,
                        Keyword::Elif if prev_kwd == Keyword::Else => {
                            return Err(ZyxtError::error_2_1_7(Keyword::Elif.to_string())
                                .with_pos_raw(&selected.pos_raw()))
                        }
                        Keyword::Else if prev_kwd == Keyword::Else => {
                            return Err(ZyxtError::error_2_1_7(Keyword::Else.to_string())
                                .with_pos_raw(&selected.pos_raw()))
                        }
                        _ => break,
                    }
                } else {
                    break;
                };
                prev_kwd = kwd;
                selected = self.next_or_err()?;
                let condition = if kwd == Keyword::Else {
                    None
                } else if let Either::Left(
                    ele @ Element {
                        data: ElementVariant::Block(_),
                        ..
                    },
                ) = selected
                {
                    Some(ele.to_owned())
                } else {
                    let start = self.cursor;
                    while let selected = self.next_or_err()? {
                        if matches!(
                            selected,
                            Either::Left(Element {
                                data: Box(ElementVariant::Block(_), ..),
                                ..
                            })
                        ) {
                            break;
                        }
                    }
                    self.cursor -= 1;
                    Some(
                        self.window(start..self.cursor + 1)
                            .with_as_buffer(&|buf| buf.parse_as_expr())?,
                    )
                };
                selected = self.next_or_err()?;
                let block = if let Either::Left(Element {
                    data: ElementVariant::Block(block),
                    ..
                }) = selected
                {
                    block
                } else {
                    return Err(
                        ZyxtError::error_2_1_8(catcher_selected.pos_raw.raw).with_element(selected)
                    );
                };
                conditions.push(Condition {
                    condition,
                    if_true: Element {
                        pos_raw: catcher_selected.pos_raw.to_owned(),
                        data: Box::new(block.to_owned()),
                    },
                });
            }
            self.cursor -= 1;
            let ele = Element {
                pos_raw: PosRaw {
                    position: init_pos,
                    raw: self.end_raw_collection().into(),
                },
                data: Box::new(ElementVariant::If(If { conditions })),
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
