use std::borrow::Cow;

use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{block::Block, literal::Literal, procedure::Procedure, Element, ElementVariant},
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{Keyword, Token, TokenType},
        typeobj::unit_t::UNIT_T,
        value::Value,
    },
};

impl<'a> Buffer<'a> {
    fn parse_proc_fn(&mut self) -> Result<(), ZyxtError> {
        self.reset_cursor();
        while let Some(mut selected) = self.next() {
            let (tok_selected, ty) = if let Either::Right(selected) = selected {
                if [
                    Some(TokenType::Keyword(Keyword::Proc)),
                    Some(TokenType::Keyword(Keyword::Fn)),
                    Some(TokenType::Bar),
                ]
                .contains(&selected.ty)
                {
                    (selected, selected.ty.unwrap())
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let init_pos = tok_selected.pos_raw();
            self.start_raw_collection();
            let is_fn = if *ty != Some(TokenType::Bar) {
                *ty == Some(TokenType::Keyword(Keyword::Fn))
            } else {
                false
            };
            if *ty != Some(TokenType::Bar) {
                selected = self.next_or_err()?;
            }
            let args = if let Either::Right(Token {
                ty: Some(TokenType::Bar),
                ..
            }) = selected
            {
                // TODO get arguments
                vec![]
            } else {
                self.cursor -= 1;
                vec![]
            };
            selected = self.next_or_err()?;
            let return_type = if let Either::Right(Token {
                ty: Some(TokenType::Colon),
                ..
            }) = selected
            {
                let start = self.cursor;
                while !matches!(
                    selected,
                    Either::Left(Element {
                        data: Box(ElementVariant::Block(..))
                    })
                ) {
                    selected = self.next_or_err()?;
                }
                let range = start..self.cursor;
                BufferWindow {
                    slice: Cow::Borrowed(&self.content[range.to_owned()]),
                    range,
                }
                .with_as_buffer(&|buf| {
                    let ele = buf.parse_as_expr()?;
                    Ok(ele)
                })?
            } else {
                UNIT_T.as_type_element().as_type().as_literal()
            };
            let block: Element<Block> = if let Either::Left(
                block @ Element {
                    data: Box(ElementVariant::Block(_), ..),
                    ..
                },
            ) = selected
            {
                Element {
                    pos_raw: block.pos_raw.to_owned(),
                    data: Box::new(block.data.as_block().unwrap()),
                }
            } else {
                BufferWindow {
                    slice: Cow::Borrowed(&self.content[self.cursor..self.content.len()]),
                    range,
                }
                .with_as_buffer(&|buf| {
                    let ele = buf.parse_as_expr()?;
                    Ok(Element {
                        pos_raw: ele.pos_raw,
                        data: Box::new(Block { content: vec![ele] }),
                    })
                })?
            };
            let raw = self.end_raw_collection();
            let ele = Element {
                pos_raw: PosRaw {
                    position: init_pos.position,
                    raw: raw.into(),
                },
                data: Box::new(ElementVariant::Procedure(Procedure {
                    is_fn,
                    args,
                    return_type,
                    content: block,
                })),
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
