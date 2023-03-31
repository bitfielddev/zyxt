use crate::types::position::Position;

#[derive(Clone)]
pub struct Buffer {
    content: Vec<(char, Position)>,
    cursor: usize,
    started: bool,
}
impl Buffer {
    pub fn new(input: &str, mut pos: Position) -> Self {
        Self {
            content: input
                .chars()
                .map(|c| {
                    let this_pos = pos.clone();
                    pos.next_char(c);
                    (c, this_pos)
                })
                .collect::<Vec<_>>(),
            cursor: 0,
            started: false,
        }
    }
    pub fn next(&mut self) -> Option<&(char, Position)> {
        if self.started {
            self.cursor += 1;
        } else {
            self.started = true;
        }
        self.content.get(self.cursor)
    }
    pub fn peek(&self) -> Option<(char, Position)> {
        self.content
            .get(if self.started { self.cursor + 1 } else { 0 })
            .cloned()
    }
}
