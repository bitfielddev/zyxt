use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{declare::Declare, Ast},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        errors::{ZError, ZResult},
        position::GetSpan,
        token::{Token, TokenType},
    },
};

impl Buffer {
    #[allow(unused_assignments)]
    #[tracing::instrument(skip_all)]
    pub fn parse_declaration(&mut self) -> ZResult<()> {
        self.reset_cursor();
        let mut flag_pos = None;
        let mut start = None;
        while let Some(selected) = self.next() {
            if matches!(
                selected,
                Either::Right(Token {
                    ty: Some(TokenType::Flag(_)),
                    ..
                })
            ) {
                debug!(pos = ?selected.span(), "Flag detected");
                flag_pos = Some(self.cursor);
                start = Some(self.cursor);
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

            start.get_or_insert(self.cursor - 1);

            let declared_var = if let Some(Either::Left(d)) = self.peek_prev() {
                d.to_owned()
            } else {
                return Err(ZError::error_2_1_5());
            };
            debug!(pos = ?declared_var.span(), "Parsing declaration");

            let flags = if let Some(flag_pos) = flag_pos {
                self.content[flag_pos..self.cursor - 1]
                    .iter()
                    .map(|ele| {
                        if let Either::Right(Token {
                            ty: Some(TokenType::Flag(flag)),
                            span,
                            ..
                        }) = ele
                        {
                            debug!(?flag, "Flag detected");
                            Ok((flag.to_owned(), span.to_owned()))
                        } else {
                            todo!()
                            //Err(ZError::error_2_1_6(ele.span().raw))
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
            let ele = Ast::Declare(Declare {
                variable: declared_var.to_owned().into(),
                content: content.into(),
                flags,
                ty: None,
                eq_span: None, // TODO
            });
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: start.take().unwrap()..self.content.len(),
            };
            self.splice_buffer(buffer_window);
        }
        Ok(())
    }
}
