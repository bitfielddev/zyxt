use std::borrow::Cow;

use itertools::Either;
use num::BigInt;

use crate::{
    parser::buffer::{Buffer, BufferWindow},
    types::{
        element::{call::Call, ident::Ident, literal::Literal, Element, ElementVariant},
        errors::ZyxtError,
        position::GetPosRaw,
        token::{Token, TokenType},
        value::Value,
    },
};

impl Buffer {
    fn parse_ident(token: &Token) -> Option<Element<Ident>> {
        if token.ty != Some(TokenType::Ident) {
            return None;
        }
        Some(Element {
            pos_raw: token.pos_raw(),
            data: Box::new(Ident {
                name: token.value.to_owned(),
                parent: None,
            }),
        })
    }
    pub fn parse_var_literal_call(&mut self) -> Result<(), ZyxtError> {
        self.reset_cursor();
        let mut catcher: Option<(Element, usize)> = None;
        let mut clear_catcher = |s: &mut Self, catcher: &mut Option<(Element, usize)>| {
            if let Some((mut catcher, start)) = catcher.take() {
                let buffer_window = BufferWindow {
                    slice: vec![Either::Left(catcher)],
                    range: start..s.cursor,
                };
                s.splice_buffer(buffer_window);
            }
        };
        while let Some(selected) = self.next() {
            let selected = match selected {
                Either::Left(s) => {
                    clear_catcher(self, &mut catcher);
                    catcher = Some((s.to_owned(), self.cursor));
                    continue;
                }
                Either::Right(s) => s,
            };
            match selected.ty {
                Some(TokenType::DotOpr) => {
                    let catcher = if let Some((catcher, _)) = &mut catcher {
                        catcher
                    } else {
                        return Err(ZyxtError::error_2_1_0(String::from(".")).with_token(&selected));
                    };
                    let selected = match self.next_or_err()? {
                        Either::Left(c) => {
                            if let ElementVariant::Ident(ident) = *c.data {
                                Element {
                                    pos_raw: c.pos_raw.to_owned(),
                                    data: Box::new(ident.to_owned()),
                                }
                            } else {
                                todo!("get item")
                            }
                        }
                        Either::Right(c) => {
                            if let Some(ident) = Buffer::parse_ident(&c) {
                                ident
                            } else {
                                todo!("get item")
                            }
                        }
                    };
                    *catcher = Element {
                        pos_raw: selected.pos_raw.to_owned(),
                        data: Box::new(ElementVariant::Ident(Ident {
                            name: selected.data.name,
                            parent: Some(catcher.to_owned()),
                        })),
                    }
                }
                Some(TokenType::Ident) => {
                    clear_catcher(self, &mut catcher);
                    catcher = Some((
                        Buffer::parse_ident(&selected).unwrap().as_variant(),
                        self.cursor,
                    ))
                }
                Some(TokenType::LiteralNumber)
                | Some(TokenType::LiteralMisc)
                | Some(TokenType::LiteralString) => {
                    clear_catcher(self, &mut catcher);
                    catcher = Some((
                        Element {
                            pos_raw: selected.pos_raw(),
                            data: Box::new(ElementVariant::Literal(Literal {
                                content: match selected.ty {
                                    Some(TokenType::LiteralMisc) => match &*selected.value {
                                        "true" => Value::Bool(true),
                                        "false" => Value::Bool(false),
                                        "unit" => Value::Unit,
                                        "inf" => Value::F64(f64::INFINITY),
                                        _ => unreachable!("{}", selected.value),
                                    },
                                    Some(TokenType::LiteralNumber) => {
                                        if selected.value.contains('.') {
                                            Value::F64(selected.value.parse().unwrap())
                                        // TODO Decimal
                                        } else if let Ok(val) = selected.value.parse::<i32>() {
                                            Value::I32(val)
                                        } else if let Ok(val) = selected.value.parse::<i64>() {
                                            Value::I64(val)
                                        } else if let Ok(val) = selected.value.parse::<i128>() {
                                            Value::I128(val)
                                        } else if let Ok(val) = selected.value.parse::<u128>() {
                                            Value::U128(val)
                                        } else if let Ok(val) = selected.value.parse::<BigInt>() {
                                            Value::Ibig(val)
                                        } else {
                                            unreachable!()
                                        }
                                    }
                                    Some(TokenType::LiteralString) => Value::Str(
                                        selected.value[1..selected.value.len() - 1].to_string(),
                                    ),
                                    _ty => unreachable!("{_ty:?}"),
                                },
                            })),
                        },
                        self.cursor,
                    ))
                }
                Some(TokenType::CloseParen) => {
                    return Err(ZyxtError::error_2_0_2(')'.to_string()).with_token(&selected))
                }
                Some(TokenType::OpenParen) => {
                    let catcher = if let Some((catcher, _)) = &mut catcher {
                        catcher
                    } else {
                        return Err(ZyxtError::error_2_1_0(String::from("(")).with_token(&selected));
                        // parens should have been settled in the first part
                    };
                    let mut contents = self.get_split_between(
                        TokenType::OpenParen,
                        TokenType::CloseParen,
                        TokenType::Comma,
                    )?;
                    let args = contents.with_as_buffers(&|f| {
                        let mut ele = f.parse_as_expr()?;
                        Ok(ele)
                    })?;
                    *catcher = Element {
                        pos_raw: catcher.pos_raw.to_owned(),
                        data: Box::new(ElementVariant::Call(Call {
                            called: catcher.to_owned(),
                            args,
                            kwargs: Default::default(),
                        })),
                    }
                }
                _ => clear_catcher(self, &mut catcher),
            }
        }
        println!("{:#?}", self.content);
        clear_catcher(self, &mut catcher);
        Ok(())
    }
}
