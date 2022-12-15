use std::ops::Range;

use itertools::Either;
use tracing::trace;

use crate::{
    errors::ZError,
    types::{
        position::GetSpan,
        token::{Token, TokenType},
    },
    Ast, ZResult,
};

#[derive(Clone, Debug)]
pub struct Buffer {
    pub content: Vec<Either<Ast, Token>>,
    pub cursor: usize,
    pub started: bool,
}
impl Buffer {
    pub fn new(input: Vec<Token>) -> Self {
        Self {
            content: input
                .into_iter()
                .map(Either::Right)
                .collect::<Vec<Either<Ast, _>>>(),
            cursor: 0,
            started: false,
        }
    }
    pub fn this(&self) -> Option<Either<Ast, Token>> {
        self.content.get(self.cursor).cloned()
    }
    pub fn next(&mut self) -> Option<Either<Ast, Token>> {
        if self.started {
            self.cursor += 1;
        } else {
            self.started = true;
        }
        self.content.get(self.cursor).cloned()
    }
    pub fn next_or_err(&mut self) -> ZResult<Either<Ast, Token>> {
        if let Some(c) = self.next() {
            Ok(c)
        } else {
            let curr_span = match &self.content.last() {
                Some(Either::Left(c)) => c.span(),
                Some(Either::Right(c)) => c.span(),
                None => return Err(ZError::p007()),
            };
            Err(ZError::p007().with_span(curr_span))
        }
    }
    pub fn peek_prev(&mut self) -> Option<&Either<Ast, Token>> {
        if !self.started || self.cursor == 0 {
            None
        } else {
            self.content.get(self.cursor - 1)
        }
    }
    pub fn prev(&mut self) -> ZResult<()> {
        if self.cursor == 0 {
            if self.started {
                self.started = false;
            } else {
                return Err(if let Some(first) = self.content.first() {
                    ZError::p008().with_span(first)
                } else {
                    ZError::p008()
                });
            }
        } else {
            self.cursor -= 1;
        }
        Ok(())
    }
    pub fn rest_incl_curr(&mut self) -> BufferWindow {
        self.window(self.cursor..self.content.len())
    }
    pub const fn next_cursor_pos(&self) -> usize {
        if self.started {
            self.cursor + 1
        } else {
            0
        }
    }
    pub fn reset_cursor(&mut self) {
        self.started = false;
        self.cursor = 0;
    }
    pub fn peek(&self) -> Option<&Either<Ast, Token>> {
        self.content.get(self.next_cursor_pos())
    }
    pub fn window(&self, range: Range<usize>) -> BufferWindow {
        BufferWindow {
            slice: self.content[range.to_owned()].to_owned(),
            range,
        }
    }
    #[tracing::instrument(skip(self))]
    pub fn get_between(
        &mut self,
        start_token: TokenType,
        end_token: TokenType,
    ) -> ZResult<BufferWindow> {
        let mut nest_level = 1usize;
        let start = self.cursor;
        while let Some(ele) = self.next() {
            if let Either::Right(ele) = &ele {
                if start_token == end_token {
                    nest_level = 0 /*usize::from(nest_level != 1)*/;
                } else if ele.ty == Some(start_token) {
                    nest_level += 1;
                } else if ele.ty == Some(end_token) {
                    nest_level -= 1;
                }
            }
            trace!(?ele, nest_level);
            if nest_level == 0 {
                break;
            }
        }
        if nest_level != 0 {
            return Err(ZError::p009(end_token).with_span(&self.content[self.cursor]));
        }
        Ok(BufferWindow {
            slice: self.content[start + 1..self.cursor].to_owned(),
            range: start..self.next_cursor_pos(),
        })
    }
    pub fn get_split(&mut self, divider: TokenType) -> ZResult<BufferWindows> {
        let mut start = self.cursor;
        let mut buffer_windows = vec![];
        while let Some(ele) = self.next() {
            if let Either::Right(ele) = ele {
                if ele.ty == Some(divider) {
                    trace!(pos = ?ele.span(), "Split");
                    buffer_windows.push(self.window(start..self.cursor).to_owned());
                    start = self.next_cursor_pos();
                }
            }
        }
        if start != self.cursor {
            buffer_windows.push(self.window(start..self.cursor));
        }
        Ok(BufferWindows {
            buffer_windows,
            range: start..self.cursor,
        })
    }
    pub fn get_split_between(
        &mut self,
        start_token: TokenType,
        end_token: TokenType,
        divider: TokenType,
    ) -> ZResult<BufferWindows> {
        let mut nest_level = 1usize;
        let bet_start = self.cursor;
        let mut start = self.cursor + 1;
        let mut buffer_windows = vec![];
        while let Some(ele) = self.next() {
            if let Either::Right(ele) = &ele {
                if start_token == end_token {
                    nest_level = 0 /*usize::from(nest_level != 1)*/;
                } else if ele.ty == Some(start_token) {
                    nest_level += 1;
                } else if ele.ty == Some(end_token) {
                    nest_level -= 1;
                }
                if nest_level == 1 && ele.ty == Some(divider) {
                    trace!(pos = ?ele.span(), "Split");
                    buffer_windows.push(self.window(start..self.cursor).to_owned());
                    start = self.next_cursor_pos();
                }
            }
            trace!(?ele, nest_level);
            if nest_level == 0 {
                break;
            }
        }
        if nest_level != 0 {
            return Err(ZError::p009(end_token).with_span(&self.content[self.cursor]));
        }
        if start != self.cursor {
            buffer_windows.push(self.window(start..self.cursor));
        }
        Ok(BufferWindows {
            buffer_windows,
            range: bet_start..self.next_cursor_pos(),
        })
    }
    pub fn splice_buffer(&mut self, buffer: BufferWindow) {
        self.content = self.content.to_owned();
        self.cursor = buffer.range.end - 1 + buffer.slice.len() - buffer.range.len();
        self.content.splice(buffer.range, buffer.slice);
    }
}

#[derive(Clone)]
pub struct BufferWindow {
    pub slice: Vec<Either<Ast, Token>>,
    pub range: Range<usize>,
}
impl BufferWindow {
    pub fn as_buffer(&self) -> Buffer {
        Buffer {
            content: self.slice.to_owned(),
            cursor: 0,
            started: false,
        }
    }
    pub fn with_as_buffer<T>(&mut self, f: &impl Fn(&mut Buffer) -> ZResult<T>) -> ZResult<T> {
        let mut buffer = self.as_buffer();
        let res = f(&mut buffer)?;
        let bw = Self {
            slice: buffer.content,
            range: self.range.to_owned(),
        };
        *self = bw;
        Ok(res)
    }
}

#[derive(Clone)]
pub struct BufferWindows {
    pub buffer_windows: Vec<BufferWindow>,
    pub range: Range<usize>,
}
impl BufferWindows {
    pub fn with_as_buffers<T>(
        &mut self,
        f: &impl Fn(&mut Buffer) -> ZResult<T>,
    ) -> ZResult<Vec<T>> {
        self.buffer_windows
            .iter_mut()
            .map(move |b| b.with_as_buffer(&f))
            .collect::<ZResult<Vec<_>>>()
    }
}
