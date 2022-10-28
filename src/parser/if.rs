use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{
            r#if::{Condition, If},
            Element, ElementVariant,
        },
        errors::{ZError, ZResult},
        position::{GetPosRaw, PosRaw},
        token::{Keyword, Token, TokenType},
    },
};

impl Buffer {
    pub fn parse_if(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let kwd = if let Either::Right(Token {
                ty: Some(TokenType::Keyword(kwd)),
                ..
            }) = selected
            {
                kwd
            } else {
                continue;
            };
            if [Keyword::Elif, Keyword::Else].contains(&kwd) {
                return Err(ZError::error_2_1_9(
                    if kwd == Keyword::Elif { "elif" } else { "else" }.to_string(),
                )
                .with_pos_raw(&selected.pos_raw()));
            } else if kwd != Keyword::If {
                continue;
            };

            let init_pos = selected.pos_raw().position;
            let start = self.cursor;
            let mut conditions: Vec<Condition> = vec![];
            let mut prev_kwd = Keyword::If;
            self.start_raw_collection();
            loop {
                let mut selected = self.next_or_err()?;
                let kwd = if let Either::Right(Token {
                    ty: Some(TokenType::Keyword(prekwd)),
                    ..
                }) = selected
                {
                    match prekwd {
                        Keyword::If if self.cursor == start => Keyword::If,
                        Keyword::Elif if prev_kwd != Keyword::Else => Keyword::Elif,
                        Keyword::Else if prev_kwd != Keyword::Else => Keyword::Else,
                        Keyword::Elif if prev_kwd == Keyword::Else => {
                            return Err(
                                ZError::error_2_1_7("elif").with_pos_raw(&selected.pos_raw())
                            )
                        }
                        Keyword::Else if prev_kwd == Keyword::Else => {
                            return Err(
                                ZError::error_2_1_7("else").with_pos_raw(&selected.pos_raw())
                            )
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
                        data: box ElementVariant::Block(_),
                        ..
                    },
                ) = selected
                {
                    Some(ele.to_owned())
                } else {
                    let start = self.cursor;
                    loop {
                        let selected = self.next_or_err()?;
                        if matches!(
                            selected,
                            Either::Left(Element {
                                data: box ElementVariant::Block(_),
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
                    data: box ElementVariant::Block(block),
                    ..
                }) = &selected
                {
                    block.to_owned()
                } else {
                    return Err(ZError::error_2_1_8(selected.pos_raw().raw)
                        .with_pos_raw(&selected.pos_raw()));
                };
                conditions.push(Condition {
                    condition,
                    if_true: Element {
                        pos_raw: selected.pos_raw().to_owned(),
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
                slice: vec![Either::Left(ele)],
                range: start..self.next_cursor_pos(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
