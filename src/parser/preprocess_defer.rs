use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, Defer, Preprocess},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        position::GetSpan,
        token::{Keyword, TokenType},
    },
    ZResult,
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_preprocess_defer(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            let (selected, kwd) = if let Either::Right(selected) = selected {
                if let Some(TokenType::Keyword(kwd)) = &selected.ty {
                    if [Keyword::Defer, Keyword::Pre].contains(kwd) {
                        (selected.to_owned(), *kwd)
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
            let kwd_span = selected.span;
            debug!(pos = ?kwd_span, ?kwd, "Parsing preprocess/defer");
            let selected = self.next_or_err()?;

            let (content, end) = if let Either::Left(selected) = selected {
                if let Ast::Block(_) = &selected {
                    debug!(pos = ?selected.span(), "Block detected");
                    (selected.to_owned(), self.next_cursor_pos())
                } else {
                    debug!(pos = ?selected.span(), "Expression not in {{}} detected");
                    (
                        self.rest_incl_curr().with_as_buffer(&Self::parse_as_expr)?,
                        self.content.len(),
                    )
                }
            } else {
                debug!(pos = ?selected.span(), "Block not in {{}} detected");
                (
                    self.rest_incl_curr().with_as_buffer(&Self::parse_as_expr)?,
                    self.content.len(),
                )
            };
            let ele = if kwd == Keyword::Pre {
                Ast::Preprocess(Preprocess {
                    kwd_span,
                    content: content.into(),
                })
            } else {
                Ast::Defer(Defer {
                    kwd_span,
                    content: content.into(),
                })
            };
            trace!(?ele);
            let buffer_window = BufferWindow {
                slice: vec![Either::Left(ele)],
                range: start..end,
            };
            self.splice_buffer(buffer_window);
        }
        Ok(())
    }
}
