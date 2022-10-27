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

use std::{borrow::Cow, cmp::min, collections::HashMap};

use itertools::Either;
use num::BigInt;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{
            block::Block, comment::Comment, ident::Ident, procedure::Argument, r#if::Condition,
            Element, ElementData, ElementVariant, VecElementRaw,
        },
        errors::ZyxtError,
        position::{GetPosRaw, PosRaw},
        token::{get_order, Keyword, OprType, Side, Token, TokenCategory, TokenType},
        typeobj::unit_t::UNIT_T,
        value::Value,
    },
    Type,
};

impl<'a> Buffer<'a> {
    fn parse_as_block(&mut self) -> Result<Element<Block>, ZyxtError> {
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
            slice: Cow::Owned(vec![Either::Left(ele.as_variant())]),
            range: buffers.range,
        };
        self.splice_buffer(buffer_window);
        Ok(ele)
    }
    fn parse_as_expr(&mut self) -> Result<Element, ZyxtError> {
        self.parse_parentheses()?;
        self.parse_if_expr()?;
        self.parse_procs_and_fns()?;
        self.parse_preprocess_and_defer()?;
        self.parse_classes_structs_and_mixins()?;
        //self.parse_enums()?;
        self.parse_vars_literals_and_calls()?;
        self.parse_delete_expr()?;
        self.parse_return_expr()?;
        self.parse_declaration_expr()?;
        self.parse_assignment_oprs()?;
        self.parse_normal_oprs()?;
        self.parse_unparen_calls()?;
        self.parse_un_oprs()?;
        if let Some(ele) = self.content.get(2) {
            return Err(ZyxtError::error_2_1_0(ele.pos_raw().raw).with_pos_raw(&ele.pos_raw()));
        }
        match self
            .content
            .first()
            .unwrap_or(&Either::Left(Value::Unit.as_element()))
        {
            Either::Left(c) => Ok(c.to_owned()),
            Either::Right(c) => {
                Err(ZyxtError::error_2_1_0(c.pos_raw().raw).with_pos_raw(&c.pos_raw()))
            }
        }
    }
}

pub fn parse_token_list(mut input: Vec<Token>) -> Result<Vec<Element>, ZyxtError> {
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
            return Err(ZyxtError::error_2_1_10(token.value.to_owned()).with_token(token));
        }
    }

    input.retain(|token| token.ty != Some(TokenType::Comment));

    let buffer = Buffer::new(input);
    buffer
        .content
        .into_iter()
        .map(|e| {
            if let Either::Left(e) = e {
                Ok(e.to_owned())
            } else {
                todo!()
            }
        })
        .collect::<Result<Vec<_>, ZyxtError>>()
}
