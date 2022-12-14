use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, Call},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        errors::ZResult,
        position::GetSpan,
        token::{Token, TokenType},
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_unparen_call(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            if self.cursor == self.content.len() - 1 {
                continue;
            }
            let function = if let Either::Left(selected) = &selected {
                selected.to_owned()
            } else {
                continue;
            };

            debug!(pos = ?function.span(), "Parsing unparenthesised call");
            let start = self.cursor;
            let mut args = vec![];
            let mut arg_start = self.cursor + 1;
            while let Some(selected) = self.next() {
                if matches!(
                    selected,
                    Either::Right(Token {
                        ty: Some(TokenType::Comma),
                        ..
                    })
                ) {
                    if arg_start == self.cursor {
                        todo!("error")
                    }
                    debug!(pos = ?selected.span(), "Comma detected");
                    args.push(
                        self.window(arg_start..self.cursor)
                            .with_as_buffer(&|buf| buf.parse_as_expr())?,
                    );
                    arg_start = self.cursor + 1
                }
            }
            if matches!(
                self.content.last(),
                Some(Either::Right(Token {
                    ty: Some(TokenType::Comma),
                    ..
                }))
            ) {
                todo!("error")
            }
            args.push(
                self.window(arg_start..self.cursor)
                    .with_as_buffer(&|buf| buf.parse_as_expr())?,
            );
            let ele = Ast::Call(Call {
                called: function.into(),
                paren_spans: None,
                args,
                kwargs: Default::default(),
            });
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: start..self.content.len(),
            };
            self.splice_buffer(buffer_window)
        }
        Ok(())
    }
}
