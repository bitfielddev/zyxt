use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Argument, Ast, Block, Procedure},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        errors::ZResult,
        position::GetSpan,
        token::{Keyword, Token, TokenType},
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_args(&mut self) -> ZResult<Vec<Argument>> {
        let mut windows =
            self.get_split_between(TokenType::Bar, TokenType::Bar, TokenType::Comma)?;
        windows.with_as_buffers(&|buf| {
            let arg_sections = buf
                .get_split(TokenType::Colon)?
                .with_as_buffers(&Self::parse_as_expr)?;
            let name = if let Some(name) = arg_sections.first() {
                if let Ast::Ident(ident) = name {
                    debug!(pos = ?name.span(), "Name detected");
                    ident.to_owned()
                } else {
                    todo!()
                }
            } else {
                todo!()
            };
            let ty = if let Some(ele) = arg_sections.get(1) {
                debug!(pos = ?ele.span(), "Type detected");
                ele.to_owned().into()
            } else {
                todo!()
            };
            let default = arg_sections.get(2).cloned();
            debug!(pos = ?default.as_ref().map(GetSpan::span), "Default may be detected");
            Ok(Argument { name, ty, default })
        })
    }
    #[tracing::instrument(skip_all)]
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
            let kwd_span = (ty != TokenType::Bar).then_some(tok_selected.span);
            let start = self.cursor;
            debug!(pos = ?kwd_span, "Parsing proc / fn");

            let is_fn = if ty == TokenType::Bar {
                false
            } else {
                ty == TokenType::Keyword(Keyword::Fn)
            };
            if ty != TokenType::Bar {
                selected = self.next_or_err()?;
            }
            debug!(is_fn);
            let args = if let Either::Right(Token {
                ty: Some(TokenType::Bar),
                span: pos,
                ..
            }) = &selected
            {
                debug!(?pos, "Argument list detected");
                self.parse_args()?
            } else {
                self.cursor -= 1;
                vec![]
            };
            selected = self.next_or_err()?;
            let return_type = if let Either::Right(Token {
                ty: Some(TokenType::Colon),
                span: pos,
                ..
            }) = &selected
            {
                debug!(?pos, "Return type detected");
                let start = self.cursor + 1;
                while !matches!(selected, Either::Left(Ast::Block(..))) {
                    selected = self.next_or_err()?;
                }
                let range = start..self.cursor;
                Some(
                    BufferWindow {
                        slice: self.content[range.to_owned()].to_owned(),
                        range,
                    }
                    .with_as_buffer(&|buf| {
                        let ele = buf.parse_as_expr()?;
                        Ok(ele)
                    })?,
                )
            } else {
                None
            };
            let block: Block = if let Either::Left(block @ Ast::Block(_)) = &selected {
                debug!(pos = ?block.span(), "Block detected");
                block.as_block().unwrap().to_owned()
            } else {
                debug!(pos = ?selected.span(), "Expression detected");
                self.window(self.cursor..self.content.len())
                    .with_as_buffer(&|buf| {
                        let ele = buf.parse_as_expr()?;
                        Ok(Block {
                            brace_spans: None,
                            content: vec![ele],
                        })
                    })?
            };
            let ele = Ast::Procedure(Procedure {
                is_fn,
                kwd_span,
                args,
                return_type: return_type.map(Into::into),
                content: block,
            });
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: start..self.next_cursor_pos(),
            };
            self.splice_buffer(buffer_window);
        }
        Ok(())
    }
}
