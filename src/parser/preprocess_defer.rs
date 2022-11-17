use itertools::{Either, Itertools};
use tracing::{debug, trace};

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{defer::Defer, preprocess::Preprocess, ElementVariant},
        position::{GetPosRaw, PosRaw},
        token::{Keyword, TokenType},
    },
    Element, ZResult,
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
            let mut raw = selected.get_raw();
            let start = self.cursor;
            let init_pos_raw = selected.pos_raw();
            debug!(pos = ?init_pos_raw.pos, ?kwd, "Parsing preprocess/defer");
            let selected = self.next_or_err()?;

            raw += &*selected.pos_raw().raw;
            let (content, end) = if let Either::Left(selected) = selected {
                if let ElementVariant::Block(_) = &*selected.data {
                    debug!(pos = ?selected.pos_raw.pos, "Block detected");
                    (selected.to_owned(), self.next_cursor_pos())
                } else {
                    debug!(pos = ?selected.pos_raw.pos, "Expression not in {{}} detected");
                    (
                        self.rest_incl_curr()
                            .with_as_buffer(|buffer| buffer.parse_as_expr())?
                            .as_variant(),
                        self.content.len(),
                    )
                }
            } else {
                debug!(pos = ?selected.pos_raw().pos, "Block not in {{}} detected");
                (
                    self.rest_incl_curr()
                        .with_as_buffer(|buffer| buffer.parse_as_expr())?
                        .as_variant(),
                    self.content.len(),
                )
            };
            let ele = Element {
                pos_raw: PosRaw {
                    pos: init_pos_raw.pos,
                    raw: self.content[start..end]
                        .iter()
                        .map(|a| a.pos_raw().raw)
                        .join("")
                        .into(),
                },
                data: Box::new(if kwd == Keyword::Pre {
                    ElementVariant::Preprocess(Preprocess { content })
                } else {
                    ElementVariant::Defer(Defer { content })
                }),
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
