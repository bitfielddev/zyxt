use std::fmt::{Display, Formatter};
use regex::{Error, Regex};
use crate::syntax::lexing::{compound_token_entries_1, compound_token_entries_2, CompoundTokenEntry, Pattern, singular_token_entries, TokenType};
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
    let token_entries = singular_token_entries();
    for c in input.chars() {
        let mut found = false;
        for entry in token_entries.iter() {
            if {
                if let Some(re) = &entry.re {re.is_match(&*c.to_string())}
                else {c == entry.value}
            } {
                out.push(Token{
                    value: c.to_string(),
                    type_: entry.type_,
                    position: pos.clone(),
                    categories: entry.categories,
                    ..Default::default()
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

fn is_literal_match(out: &Vec<Token>, entry: &CompoundTokenEntry) -> Option<usize> {
    let mut cursor = out.len()-1;
    let mut selected = out.last().unwrap();
    let mut match_count = 1usize;
    let mut indent = 0u8;
    while selected.type_ != entry.pair.unwrap() || indent != 0 {
        if selected.type_ == entry.type_ && selected != out.last().unwrap() {indent += 1;}
        if selected.type_ == entry.pair.unwrap() && indent != 0 {indent -= 1;}

        match_count += 1;
        if cursor == 0 {return None} // raise error
        cursor -= 1;
        selected = &out.get(cursor).unwrap();
    }
    Some(match_count)
}


fn is_match(combination: &[Pattern<'_>], out: &Vec<Token>) -> Option<usize> {
    let mut _cursor = out.len()-1;
    let mut selected = out.last().unwrap();
    let mut match_count = 0usize;
    for (i, p) in combination.iter().rev().enumerate() {
        macro_rules! update_cursor {
            () => {
                match_count += 1;
                if _cursor == 0 && combination.len() != i+1 {return None}
                else if _cursor == 0 {return Some(match_count)}
                _cursor -= 1;
                selected = &out.get(_cursor).unwrap();
            };
        }
        match p {
            Pattern::Value(token_type, value) => {
                if i == 0 {update_cursor!(); continue;}
                if token_type != &TokenType::Null
                    && token_type != &selected.type_ {return None}
                if value != &selected.value {return None}
                update_cursor!();
            }
            Pattern::Token(token_type) => {
                if i == 0 {update_cursor!(); continue;}
                if token_type != &TokenType::Null
                    && token_type != &selected.type_ {return None}
                update_cursor!();
            }
            Pattern::Vartokens(token_type) => {
                while token_type == &TokenType::Null || token_type == &selected.type_ {
                    update_cursor!();
                }
            }
            Pattern::Re(token_type, re) => {
                if i == 0 {update_cursor!(); continue;}
                if token_type != &TokenType::Null
                    && token_type != &selected.type_ {return None}
                if !Regex::new(re).unwrap().is_match(&selected.value) {return None}
                update_cursor!();
            }
        }
    }
    if match_count == 0 {None} else {Some(match_count)}
}

fn lex_stage2(input: Vec<Token>) -> Result<Vec<Token>, Error>{
    let mut out: Vec<Token> = vec![];

    let token_entries = compound_token_entries_1();
    for t in input {
        out.push(t.clone());
        for entry in token_entries.iter() {
            let (Pattern::Value(token_type, ..)
            | Pattern::Token(token_type)
            | Pattern::Vartokens(token_type)
            | Pattern::Re(token_type, ..)) = entry.combination.last().unwrap();
            if token_type == &TokenType::Null
                || token_type == &t.type_ && {
                if let Pattern::Value(_, value) = entry.combination.last().unwrap() {
                    t.value == value.to_string()
                } else { true }
            } { if let Some(count) = is_match(entry.combination, &out) {
                    let pos = out.get(out.len() - count).unwrap().position.clone();
                    let value = out.drain(out.len() - count..)
                        .map(|t| t.value).collect();
                    out.push(Token {
                        value,
                        type_: entry.type_,
                        position: pos,
                        categories: entry.categories,
                        ..Default::default()
                    });
                }

            }
        }
    }
    Ok(out)
}

fn lex_stage3(input: Vec<Token>) -> Result<Vec<Token>, Error>{
    let mut out: Vec<Token> = vec![];

    let token_entries = compound_token_entries_2();
    for t in input {
        out.push(t.clone());
        for entry in token_entries.iter() {
            let (Pattern::Value(token_type, ..)
            | Pattern::Token(token_type)
            | Pattern::Vartokens(token_type)
            | Pattern::Re(token_type, ..)) = entry.combination.last().unwrap();
            if token_type == &TokenType::Null
                || token_type == &t.type_ && {
                if let Pattern::Value(_, value) = entry.combination.last().unwrap() {
                    t.value == value.to_string()
                } else { true }
            } { if entry.literal {
                if let Some(count) = is_literal_match(&out, entry) {
                    let pos = out.get(out.len() - count).unwrap().position.clone();
                    let value = out.drain(out.len() - count..)
                        .map(|t| t.value).collect();
                    out.push(Token {
                        value,
                        type_: entry.type_,
                        position: pos,
                        categories: entry.categories,
                        ..Default::default()
                    });
                }} else if let Some(count) = is_match(entry.combination, &out) {
                    let pos = out.get(out.len() - count).unwrap().position.clone();
                    let value = out.drain(out.len() - count..)
                        .map(|t| t.value).collect();
                    out.push(Token {
                        value,
                        type_: entry.type_,
                        position: pos,
                        categories: entry.categories,
                        ..Default::default()
                    });
                }
            }
        }
    }
    Ok(out)
}

fn clean_whitespaces(input: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = vec![];
    let mut whitespace_stack = "".to_string();

    for mut t in input {
        if t.type_ != TokenType::Whitespace {
            t.whitespace = whitespace_stack.clone();
            whitespace_stack = "".to_string();
            out.push(t);
        }
        else {
            whitespace_stack.push_str(&*t.value);
        }
    }
    out
}

pub fn lex(preinput: String, filename: &String) -> Result<Vec<Token>, Error> {
    if preinput.trim().len() == 0 {return Ok(vec![])};
    let input = preinput + "\n";

    let out1 = lex_stage1(input, filename)?;
    let out2 = lex_stage2(out1)?;
    let out3 = lex_stage3(out2)?;
    let out4 = clean_whitespaces(out3);
    Ok(out4)
}