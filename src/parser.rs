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

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{block::Block, comment::Comment, Element},
        errors::{ZError, ZResult},
        position::GetPosRaw,
        token::{Token, TokenType},
        value::Value,
    },
};

impl Buffer {
    fn parse_as_block(&mut self) -> ZResult<Element<Block>> {
        let mut buffers = self.get_split_between(
            TokenType::OpenCurlyParen,
            TokenType::CloseCurlyParen,
            TokenType::StatementEnd,
        )?;
        let block = buffers.with_as_buffers(&|buffer| buffer.parse_as_expr())?;
        let ele = Element {
            pos_raw: self.content.get(0).map(|c| c.pos_raw()).unwrap_or_default(),
            data: Box::new(Block { content: block }),
        };
        let buffer_window = BufferWindow {
            slice: vec![Either::Left(ele.as_variant())],
            range: buffers.range,
        };
        self.splice_buffer(buffer_window);
        Ok(ele)
    }
    fn parse_as_expr(&mut self) -> ZResult<Element> {
        self.parse_parentheses()?;
        self.parse_if()?;
        self.parse_proc_fn()?;
        self.parse_preprocess_defer()?;
        self.parse_class_struct()?;
        //self.parse_enum()?;
        self.parse_var_literal_call()?;
        self.parse_delete()?;
        self.parse_return()?;
        self.parse_declaration()?;
        self.parse_assignment_opr()?;
        self.parse_bin_opr()?;
        self.parse_un_opr()?;
        self.parse_unparen_call()?;
        if let Some(ele) = self.content.get(2) {
            return Err(ZError::error_2_1_0(ele.pos_raw().raw).with_pos_raw(&ele.pos_raw()));
        }
        match self
            .content
            .first()
            .unwrap_or(&Either::Left(Value::Unit.as_element()))
        {
            Either::Left(c) => Ok(c.to_owned()),
            Either::Right(c) => {
                Err(ZError::error_2_1_0(c.pos_raw().raw).with_pos_raw(&c.pos_raw()))
            }
        }
    }
}

pub fn parse_token_list(mut input: Vec<Token>) -> ZResult<Vec<Element>> {
    let mut comments: Vec<Element<Comment>> = vec![];

    // detect & remove comments
    for token in input.iter() {
        if token.ty == Some(TokenType::Comment) {
            comments.push(Element {
                pos_raw: token.pos_raw(),
                data: Box::new(Comment {
                    content: token.value.to_owned(),
                }),
            })
        } else if [
            Some(TokenType::CommentStart),
            Some(TokenType::CommentEnd),
            Some(TokenType::MultilineCommentStart),
            Some(TokenType::MultilineCommentEnd),
        ]
        .contains(&token.ty)
        {
            return Err(ZError::error_2_1_10(token.value.to_owned()).with_token(token));
        }
    }

    input.retain(|token| token.ty != Some(TokenType::Comment));

    Buffer::new(input)
        .get_split(TokenType::StatementEnd)?
        .with_as_buffers(&|buf| buf.parse_as_expr()) // TODO merge all errors into one here
}
