use itertools::Either;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, Delete, Ident, UnaryOpr},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        errors::ZResult,
        position::GetSpan,
        token::{Keyword, OprType, Token, TokenType},
    },
};

impl Buffer {
    #[tracing::instrument(skip_all)]
    pub fn parse_delete(&mut self) -> ZResult<()> {
        self.reset_cursor();
        while let Some(selected) = self.next() {
            if !matches!(
                selected,
                Either::Right(Token {
                    ty: Some(TokenType::Keyword(Keyword::Delete)),
                    ..
                })
            ) {
                continue;
            }
            let init_span = selected.span();
            debug!(pos = ?init_span, "Parsing delete");
            let start = self.cursor;
            self.next_or_err()?;
            let vars: Vec<Ident> = self.get_split(TokenType::Comma)?.with_as_buffers(&|buf| {
                let ele = buf.parse_as_expr()?;
                if let Ast::Ident(data) = &ele {
                    Ok(data.to_owned())
                } else if let Ast::UnaryOpr(UnaryOpr {
                    ty: OprType::Deref, ..
                }) = &ele
                {
                    todo!()
                    //Err(ZError::error_2_1_12(&ele.raw).with_element(&ele))
                } else {
                    todo!()
                    //Err(ZError::error_2_1_11(&ele.span.raw).with_element(&ele))
                }
            })?;
            let ele = Ast::Delete(Delete {
                kwd_span: init_span,
                names: vars,
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
