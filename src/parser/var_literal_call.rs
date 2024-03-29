use std::collections::HashMap;

use itertools::Either;
use num::BigInt;
use tracing::{debug, trace};

use crate::{
    ast::{Ast, AstData, Call, Ident, Literal, Member},
    errors::{ToZResult, ZError, ZResult},
    parser::buffer::{Buffer, BufferWindow},
    types::{
        position::GetSpan,
        token::{Token, TokenType},
        value::Value,
    },
};

impl Buffer {
    fn parse_str_literal(s: &str) -> ZResult<String> {
        let mut out = String::new();
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            if c != '\\' {
                out.push(c);
                continue;
            }
            out.push(match chars.next().z()? {
                '\\' => '\\',
                'n' => '\n',
                't' => '\t',
                'r' => '\r',
                n => n,
            });
        }
        Ok(out)
    }
    fn parse_ident(token: &Token) -> Option<Ident> {
        if token.ty != Some(TokenType::Ident) {
            return None;
        }
        Some(Ident {
            name: token.value.to_owned(),
            name_span: Some(token.span.to_owned()),
        })
    }
    #[tracing::instrument(skip_all)]
    pub fn parse_var_literal_call(&mut self) -> ZResult<()> {
        self.reset_cursor();
        let mut catcher: Option<(Ast, usize)> = None;
        let clear_catcher = |s: &mut Self, catcher: &mut Option<(Ast, usize)>, _end: bool| {
            if let Some((catcher, start)) = catcher.take() {
                let buffer_window = BufferWindow {
                    slice: vec![Either::Left(catcher)],
                    range: start..s.cursor,
                };
                s.splice_buffer(buffer_window);
                s.cursor += 1;
            }
        };
        while let Some(selected) = self.next() {
            let selected = match selected {
                Either::Left(s) => {
                    clear_catcher(self, &mut catcher, false);
                    catcher = Some((s.to_owned(), self.cursor));
                    continue;
                }
                Either::Right(s) => s,
            };
            match selected.ty {
                Some(TokenType::DotOpr(access_ty)) => {
                    let dot_span = selected.span;
                    debug!(pos = ?dot_span, "Parsing dot operator");
                    let Some((catcher, _)) = &mut catcher else {
                        return Err(ZError::p022().with_span(dot_span));
                    };
                    let selected = match self.next_or_err()? {
                        Either::Left(c) => {
                            if let Ast::Ident(ident) = c {
                                ident.to_owned()
                            } else {
                                todo!("get item")
                            }
                        }
                        Either::Right(c) => {
                            if let Some(ident) = Self::parse_ident(&c) {
                                ident
                            } else {
                                todo!("get item")
                            }
                        }
                    };
                    debug!(pos = ?selected.span(), "Parsing ident");
                    *catcher = Ast::Member(Member {
                        ty: access_ty,
                        name: selected.name.to_owned(),
                        parent: Box::new(catcher.to_owned()),
                        name_span: selected.span(),
                        dot_span: Some(dot_span),
                    });
                    trace!(?catcher);
                }
                Some(TokenType::Ident) => {
                    debug!(pos = ?selected.span, "Parsing ident");
                    clear_catcher(self, &mut catcher, false);
                    let ident = Self::parse_ident(&selected)
                        .unwrap_or_else(|| unreachable!())
                        .as_variant();
                    catcher = Some((ident, self.cursor));
                    trace!(catcher = ?catcher.as_ref().unwrap_or_else(|| unreachable!()).0);
                }
                Some(
                    TokenType::LiteralNumber | TokenType::LiteralMisc | TokenType::LiteralString,
                ) => {
                    clear_catcher(self, &mut catcher, false);
                    catcher = Some((
                        Ast::Literal(Literal {
                            span: Some(selected.span),
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
                                        Value::F64(
                                            selected
                                                .value
                                                .parse()
                                                .unwrap_or_else(|_| unreachable!()),
                                        )
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
                                Some(TokenType::LiteralString) => Value::Str({
                                    let str = &selected.value[1..selected.value.len() - 1];
                                    Self::parse_str_literal(str)?
                                }),
                                _ty => unreachable!("{_ty:?}"),
                            },
                        }),
                        self.cursor,
                    ));
                    trace!(catcher = ?catcher.as_ref().unwrap_or_else(|| unreachable!()).0);
                }
                Some(TokenType::CloseParen) => return Err(ZError::p023().with_span(&selected)),
                Some(TokenType::OpenParen) => {
                    let open_paren_span = selected.span;
                    debug!(pos = ?open_paren_span, "Parsing call");
                    let Some((catcher, _)) = &mut catcher else {
                        return Err(
                            ZError::p024().with_span(open_paren_span)
                        );
                        // parens should have been settled in the first part
                    };
                    let mut contents = self.get_split_between(
                        TokenType::OpenParen,
                        TokenType::CloseParen,
                        TokenType::Comma,
                    )?;
                    let args = contents.with_as_buffers(&|f| {
                        let ele = f.parse_as_expr()?;
                        Ok(ele)
                    })?;
                    let close_paren_span = self
                        .this()
                        .and_then(|e| e.span())
                        .unwrap_or_else(|| unreachable!());
                    *catcher = Ast::Call(Call {
                        called: catcher.to_owned().into(),
                        paren_spans: Some((open_paren_span, close_paren_span)),
                        args,
                        kwargs: HashMap::default(),
                    });
                    trace!(?catcher);
                }
                _ => clear_catcher(self, &mut catcher, false),
            }
        }
        clear_catcher(self, &mut catcher, true);
        Ok(())
    }
}
