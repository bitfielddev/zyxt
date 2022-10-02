use std::{borrow::Cow, ops::Range};

use itertools::Either;

use crate::{
    types::token::{Token, TokenType},
    Element, ZyxtError,
};

#[derive(Clone)]
pub struct Buffer<'a> {
    pub content: Cow<'a, [Either<Element, Token>]>,
    cursor: usize,
    started: bool,
    raw: Option<String>,
}
impl<'a> Buffer<'a> {
    fn next(&mut self) -> Option<&Either<Element, Token>> {
        if self.started {
            self.cursor += 1;
        } else {
            self.started = true;
        }
        self.content.get(self.cursor)
    }
    fn next_cursor_pos(&self) -> usize {
        if self.started {
            self.cursor + 1
        } else {
            0
        }
    }
    fn reset_cursor(&mut self) {
        self.started = false;
        self.cursor = 0;
    }
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            content: Cow::Owned(
                input
                    .into_iter()
                    .map(Either::Right)
                    .collect::<Vec<Either<Element, _>>>(),
            ),
            cursor: 0,
            started: false,
            raw: None,
        }
    }
    pub fn peek(&self) -> Option<&Either<Element, Token>> {
        self.content.get(self.next_cursor_pos())
    }
    pub fn start_raw_collection(&mut self) {
        self.raw = Some("".into());
    }
    pub fn end_raw_collection(&mut self) -> String {
        self.raw.take().unwrap_or("".into())
    }
    pub fn get_between(
        &mut self,
        start_token: TokenType,
        end_token: TokenType,
    ) -> Result<BufferWindow, ZyxtError> {
        let mut nest_level = 0usize;
        let start = self.next_cursor_pos();
        while let Some(ele) = self.next() {
            if let Either::Right(ele) = ele {
                if start_token == end_token {
                    nest_level == if nest_level == 1 { 0 } else { 1 };
                } else if ele.ty == Some(start_token) {
                    nest_level += 1
                } else if ele.ty == Some(end_token) {
                    nest_level -= 1
                }
            }
            if nest_level == 0 {
                break;
            }
        }
        if nest_level != 0 {
            todo!("err")
        }
        Ok(BufferWindow {
            slice: Cow::Borrowed(&self.content[start + 1..self.cursor]),
            range: start..self.next_cursor_pos(),
        })
    }
    pub fn get_split_between(
        &mut self,
        start_token: TokenType,
        end_token: TokenType,
        divider: TokenType,
        start_end_has_token: bool,
    ) -> Result<BufferWindows, ZyxtError> {
        let mut nest_level = if start_end_has_token { 1usize } else { 0usize };
        let bet_start = self.next_cursor_pos();
        let mut start = self.next_cursor_pos() + 1;
        let mut buffer_windows = vec![];
        while let Some(ele) = self.next() {
            if let Either::Right(ele) = ele {
                if start_token == end_token {
                    nest_level == if nest_level == 1 { 0 } else { 1 };
                } else if ele.ty == Some(start_token) {
                    nest_level += 1
                } else if ele.ty == Some(end_token) {
                    nest_level -= 1
                }
                if nest_level == 1 && ele.ty == Some(divider) {
                    buffer_windows.push(BufferWindow {
                        slice: Cow::Borrowed(&self.content[start..self.cursor]),
                        range: start..self.cursor,
                    });
                    start = self.next_cursor_pos();
                }
            }
            if nest_level == 0 {
                break;
            }
        }
        if nest_level != 0 && !start_end_has_token {
            todo!("err")
        }
        Ok(BufferWindows {
            buffer_windows: buffer_windows,
            range: bet_start..self.next_cursor_pos(),
        })
    }
    pub fn splice_buffer(&mut self, buffer: BufferWindow) {
        self.content = self.content.to_owned();
        self.content
            .to_vec()
            .splice(buffer.range, buffer.slice.to_vec());
    }
    pub fn splice_buffers(&mut self, buffers: BufferWindows) {
        self.content = self.content.to_owned();
        self.content
            .to_vec()
            .splice(
                buffers.range,
                buffers.buffer_windows.into_iter().flat_map(|b| b.slice.to_vec()),
            )
            .collect::<Vec<_>>();
    }
}

pub struct BufferWindow<'a> {
    pub slice: Cow<'a, [Either<Element, Token>]>,
    pub range: Range<usize>,
}
impl<'a> BufferWindow<'a> {
    pub fn as_buffer(&self) -> Buffer {
        Buffer {
            content: Cow::Borrowed(self.slice.as_ref()),
            cursor: 0,
            started: false,
            raw: None,
        }
    }
}

pub struct BufferWindows<'a> {
    pub buffer_windows: Vec<BufferWindow<'a>>,
    pub range: Range<usize>,
}
impl<'a> BufferWindows<'a> {
    pub fn as_buffers(&self) -> Vec<Buffer> {
        self.buffer_windows.iter().map(|a| a.as_buffer()).collect()
    }
    pub fn with_as_buffers(&mut self, f: &dyn Fn(Buffer) -> Result<Buffer, ZyxtError>) -> Result<(), ZyxtError>{
        self.buffer_windows = self.buffer_windows.iter().map(|b| {
            Ok(BufferWindow {
                slice: f(b.as_buffer())?.content,
                range: b.range.to_owned()
            })
        }).collect()?;
        Ok(())
    }
}
