use std::fmt::{Display, Formatter};
use regex::{Error, Regex};
use crate::syntax::lexing::{singular_token_entries, TokenType};
use crate::{errors, Token};

#[derive(Clone, PartialEq)]
pub struct Position {
    pub filename: String,
    pub line: u32,
    pub column: u32,
}
impl Default for Position {
    fn default() -> Self {
        Position {
            filename: String::from("[unknown]"),
            line: 1,
            column: 1,
        }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}

impl Position {
    fn next(&mut self, c: &char) {
        if *c == '\n' {
            self.line += 1;
            self.column = 1;
        } else {self.column += 1}
    }
}

#[derive(Clone)]
pub struct StateTracker {
    pub position: Position,
    pub is_literal_string: bool,
    pub literal_string_type: TokenType,
    pub prev_type: TokenType,
}
impl Default for StateTracker {
    fn default() -> Self {
        StateTracker {
            position: Position::default(),
            is_literal_string: false,
            literal_string_type: TokenType::Null,
            prev_type: TokenType::Null,
        }
    }
}

fn lex_stage1(input: String, filename: &String) -> Result<Vec<Token>, Error> {
    let mut out: Vec<Token> = vec![];
    let mut pos = Position {
        filename: filename.clone(),
        ..Default::default()
    };
    for c in input.chars() {
        let mut found = false;
        for entry in singular_token_entries() {
            if {
                if let Some(re) = entry.re {re.is_match(&*c.to_string())}
                else {c == entry.value}
            } {
                out.push(Token{
                    value: c.to_string(),
                    type_: entry.type_,
                    position: pos.clone(),
                    categories: entry.categories
                });
                pos.next(&c);
                found = true;
                break;
            }
        }
        if !found {
            errors::error_pos(&pos);
            errors::error_2_1(c.to_string());
        }
    }
    Ok(out)
}

pub fn lex(preinput: String, filename: &String) -> Result<Vec<Token>, Error> {
    if preinput.trim().len() == 0 {return Ok(vec![])};
    let input = preinput + "\n";

    let out1 = lex_stage1(input, filename)?;
    for token in out1.iter() {println!("{}", token);}
    Ok(vec![])
}