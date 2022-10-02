use unicode_segmentation::UnicodeSegmentation;

use crate::types::position::Position;

#[derive(Clone)]
pub struct Buffer<'a> {
    content: Vec<(&'a str, Position)>,
    cursor: usize,
    started: bool,
}
impl<'a> Buffer<'a> {
    pub fn new(input: &'a String, mut pos: Position) -> Self {
        Self {
            content: input
                .graphemes(true)
                .map(|c| {
                    let this_pos = pos.clone();
                    pos.next_str(c);
                    (c, this_pos)
                })
                .collect::<Vec<_>>(),
            cursor: 0,
            started: false,
        }
    }
    pub fn next(&mut self) -> Option<&(&'a str, Position)> {
        if self.started {
            self.cursor += 1;
        } else {
            self.started = true;
        }
        self.content.get(self.cursor)
    }
    pub fn peek(&self) -> Option<(&str, Position)> {
        self.content
            .get(if self.started { self.cursor + 1 } else { 0 })
            .cloned()
    }
}
