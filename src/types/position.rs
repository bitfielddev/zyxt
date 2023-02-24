use std::{
    cmp::Ordering,
    fmt::{Debug, Display, Formatter},
    sync::Arc,
};

use itertools::Either;
use smol_str::SmolStr;

#[derive(Clone, PartialEq, Eq)]
pub struct Position {
    pub filename: Option<Arc<SmolStr>>,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Span {
    pub start_pos: Position,
    pub end_pos: Position,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            filename: None,
            line: 1.try_into().unwrap(),
            column: 1.try_into().unwrap(),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.filename.as_deref().map_or("[unknown]", |a| &**a),
            self.line,
            self.column
        )
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}

impl Position {
    #[must_use]
    pub fn end_pos(&self, string: &str) -> Self {
        Self {
            filename: self.filename.to_owned(),
            line: self.line + string.chars().filter(|c| *c == '\n').count(),
            column: if string.contains('\n') {
                string.split('\n').last().unwrap().chars().count()
            } else {
                self.column + string.chars().count() - 1
            },
        }
    }
    pub fn next_char(&mut self, c: char) {
        if c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.filename != other.filename {
            return None;
        }
        if self.line == other.line {
            self.column.partial_cmp(&other.column)
        } else {
            self.line.partial_cmp(&other.line)
        }
    }
}

impl Span {
    #[must_use]
    pub fn new(pos: Position, raw: &str) -> Self {
        Self {
            end_pos: pos.end_pos(raw),
            start_pos: pos,
        }
    }
}

pub trait GetSpan: Clone + Debug {
    fn span(&self) -> Option<Span>;
    fn merge_span(&self, other: impl GetSpan) -> Option<Span> {
        let opt_span1 = self.span();
        let opt_span2 = other.span();
        let Some(span1) = &opt_span1 else {
            return opt_span2;
        };
        let Some(span2) = &opt_span2 else {
            return opt_span1;
        };
        if span1.start_pos.filename != span2.start_pos.filename {
            return None;
        };
        Some(Span {
            start_pos: if span1.start_pos < span2.start_pos {
                span1.start_pos.to_owned()
            } else {
                span2.start_pos.to_owned()
            },
            end_pos: if span1.end_pos > span2.end_pos {
                span1.end_pos.to_owned()
            } else {
                span2.end_pos.to_owned()
            },
        })
    }
}
impl<T: GetSpan, U: GetSpan> GetSpan for Either<T, U> {
    fn span(&self) -> Option<Span> {
        match self {
            Self::Left(t) => t.span(),
            Self::Right(u) => u.span(),
        }
    }
}
impl GetSpan for Span {
    fn span(&self) -> Option<Span> {
        Some(self.to_owned())
    }
}
impl<T: GetSpan> GetSpan for Box<T> {
    fn span(&self) -> Option<Span> {
        (**self).span()
    }
}
impl<T: GetSpan> GetSpan for &T {
    fn span(&self) -> Option<Span> {
        (*self).span()
    }
}
impl<T: GetSpan> GetSpan for Option<T> {
    fn span(&self) -> Option<Span> {
        self.as_ref().and_then(GetSpan::span)
    }
}
impl<T: GetSpan> GetSpan for &[T] {
    fn span(&self) -> Option<Span> {
        let mut s: Option<Option<Span>> = None;
        for i in self.iter() {
            if let Some(s) = &mut s {
                if let Some(is) = s {
                    *s = is.merge_span(i);
                }
            } else {
                s = Some(i.span());
            }
        }
        s.unwrap_or(None)
    }
}
impl<T: GetSpan> GetSpan for Vec<T> {
    fn span(&self) -> Option<Span> {
        self.as_slice().span()
    }
}
