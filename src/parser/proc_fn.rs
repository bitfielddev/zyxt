use itertools::Either;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{
            block::Block,
            procedure::{Argument, Procedure},
            Element, ElementVariant,
        },
        errors::ZResult,
        position::{GetPosRaw, PosRaw},
        token::{Keyword, Token, TokenType},
        typeobj::unit_t::UNIT_T,
    },
};

impl Buffer {
    pub fn parse_args(&mut self) -> ZResult<Vec<Argument>> {
        let mut windows =
            self.get_split_between(TokenType::Bar, TokenType::Bar, TokenType::Comma)?;
        windows.with_as_buffers(&|buf| {
            let arg_sections = buf
                .get_split(TokenType::Colon)?
                .with_as_buffers(&|buf| buf.parse_as_expr())?;
            let name = if let Some(name) = arg_sections.first() {
                if let ElementVariant::Ident(ident) = &*name.data {
                    ident.name.to_owned()
                } else {
                    todo!()
                }
            } else {
                todo!()
            };
            let ty = if let Some(ele) = arg_sections.get(1) {
                ele.to_owned()
            } else {
                todo!()
            };
            let default = arg_sections.get(2).cloned();
            Ok(Argument { name, ty, default })
        })
    }
    pub fn parse_proc_fn(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(mut selected) = self.next() {
            let (tok_selected, ty) = if let Either::Right(selected) = &selected {
                if [
                    Some(TokenType::Keyword(Keyword::Proc)),
                    Some(TokenType::Keyword(Keyword::Fn)),
                    Some(TokenType::Bar),
                ]
                .contains(&selected.ty)
                {
                    (selected.to_owned(), selected.ty.unwrap())
                } else {
                    continue;
                }
            } else {
                continue;
            };
            let init_pos = tok_selected.pos_raw();
            let start = self.cursor;
            self.start_raw_collection();
            let is_fn = if ty != TokenType::Bar {
                ty == TokenType::Keyword(Keyword::Fn)
            } else {
                false
            };
            if ty != TokenType::Bar {
                selected = self.next_or_err()?;
            }
            let args = if let Either::Right(Token {
                ty: Some(TokenType::Bar),
                ..
            }) = &selected
            {
                self.parse_args()?
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
                        data: box ElementVariant::Block(..),
                        ..
                    })
                ) {
                    selected = self.next_or_err()?;
                }
                let range = start..self.cursor;
                BufferWindow {
                    slice: self.content[range.to_owned()].to_owned(),
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
                    data: box ElementVariant::Block(_),
                    ..
                },
            ) = selected
            {
                Element {
                    pos_raw: block.pos_raw.to_owned(),
                    data: Box::new(block.data.as_block().unwrap().to_owned()),
                }
            } else {
                self.window(self.cursor..self.content.len())
                    .with_as_buffer(&|buf| {
                        let ele = buf.parse_as_expr()?;
                        Ok(Element {
                            pos_raw: ele.pos_raw.to_owned(),
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
                slice: vec![Either::Left(ele)],
                range: start..self.next_cursor_pos(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
