mod assignment_opr;
mod bin_opr;
mod buffer;
mod class_struct;
mod declaration;
mod delete;
mod r#if;
mod parentheses;
mod preprocess_defer;
mod proc_fn;
mod r#return;
mod un_opr;
mod unparen_call;
mod var_literal_call;

use itertools::Either;
use tracing::{debug, info};

use crate::{
    ast::{Ast, AstData, Block, Comment},
    errors::{ZError, ZResult},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        token::{Token, TokenType},
        value::Value,
    },
};

impl Buffer {
    fn parse_as_block(&mut self) -> ZResult<Block> {
        let mut buffers = self.get_split_between(
            TokenType::OpenCurlyParen,
            TokenType::CloseCurlyParen,
            TokenType::StatementEnd,
        )?;
        let block = buffers.with_as_buffers(&Self::parse_as_expr)?;
        let ele = Block {
            brace_spans: None,
            content: block,
        }; // TODO brace_spans
        let buffer_window = BufferWindow {
            slice: vec![Either::Left(ele.as_variant())],
            range: buffers.range,
        };
        self.splice_buffer(buffer_window);
        Ok(ele)
    }
    fn parse_as_expr(&mut self) -> ZResult<Ast> {
        self.parse_parentheses()?;
        self.parse_if()?;
        self.parse_class_struct()?;
        self.parse_proc_fn()?;
        self.parse_preprocess_defer()?;
        //self.parse_enum()?;
        self.parse_var_literal_call()?;
        self.parse_delete()?;
        self.parse_return()?;
        self.parse_declaration()?;
        self.parse_assignment_opr()?;
        self.parse_bin_opr()?;
        self.parse_un_opr()?;
        self.parse_unparen_call()?;
        if let Some(ele) = self.content.get(1) {
            return Err(ZError::p002().with_span(ele));
        }
        match self
            .content
            .first()
            .unwrap_or(&Either::Left(Value::Unit.as_element()))
        {
            Either::Left(c) => Ok(c.to_owned()),
            Either::Right(c) => Err(ZError::p003().with_span(c)),
        }
    }
}

#[tracing::instrument(skip_all)]
pub fn parse_token_list(mut input: Vec<Token>) -> ZResult<Vec<Ast>> {
    let mut comments: Vec<Comment> = vec![];

    info!("Removing comments");
    for token in &input {
        if token.ty == Some(TokenType::Comment) {
            debug!(?token.span, "Comment detected");
            comments.push(Comment {
                content: token.value.to_owned(),
            });
        } else if [
            Some(TokenType::CommentStart),
            Some(TokenType::CommentEnd),
            Some(TokenType::MultilineCommentStart),
            Some(TokenType::MultilineCommentEnd),
        ]
        .contains(&token.ty)
        {
            return Err(ZError::p001().with_span(token));
        }
    }
    input.retain(|token| token.ty != Some(TokenType::Comment));

    Buffer::new(input)
        .get_split(TokenType::StatementEnd)?
        .with_as_buffers(&Buffer::parse_as_expr) // TODO merge all errors into one here
}
