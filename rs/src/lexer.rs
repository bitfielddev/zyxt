use std::any::Any;
use std::borrow::Borrow;
use std::ops::Deref;
use regex::Regex;
use crate::syntax::{TokenEntry, TokenType, TOKEN_CATALOGUE, TokenCategory};
use crate::{errors, Token};

#[derive(Derivative)]
struct PositionTracker {
    #[derivative(Default(value = "[unknown]"))]
    filename: string,
    #[derivative(Default(value = 1))]
    line: i32,
    #[derivative(Default(value = 1))]
    column: i32,
    #[derivative(Default(value = -1))]
    prev_column: i32,
    char_pos: int
}

struct StateTracker {
    position: PositionTracker,
    is_literal_string: bool,
    #[derivative(Default(value = TokenType::Null))]
    literal_string_type: TokenType,
    #[derivative(Default(value = TokenType::Null))]
    prev_type: TokenType,
    literal_string_line: i32,
    literal_string_column: i32,
    #[derivative(Default(value = 1))]
    token_line: i32,
    #[derivative(Default(value = 1))]
    token_column: i32,
    brackets: [char]
}

fn get_next_char(c: char, input: &String, stack: &mut Vec<String>, states: &mut StateTracker) -> Result<char, E> {
    if c == '\n' {
        states.position.line += 1;
        states.position.prev_column = position.column;
       states. position.column = 1;
    } else {states.position.column += 1;}
    states.position.char_pos += 1;
    match input.chars().nth(position.char_pos) {
        Some(c) => {
            if (c == " " || c == "\n" || c == "\r") && states.is_literal_string {stack.push(*c)}
            else if !(c == " " || c == "\n" || c == "\r") {stack.push(*c)}
            Ok(c)
        }
        None => Err(0)
    }
}
fn get_next_char_noupdate(input: &String, states: &StateTracker) -> char {
    match input.chars().nth(states.position.char_pos+1) {
        Some(c) => c,
        None => char::from(0)
    }
}

fn get_token_entry<'a>(stack: &Vec<String>, states: &StateTracker, input: &String) -> Option<(String, &'a TokenEntry)> {
    for (&prevalue, entry) in TOKEN_CATALOGUE.iter() {
        let mut value = prevalue;
        while value.len() != 0 && value.chars().nth(value.len()-1).unwrap() == "" {value = &value[..value.len() - 1]};
        let mut re1 = Regex::new(&*entry.next_prohibited).unwrap();
        let mut re2 = Regex::new(&*entry.prohibited).unwrap();

        if ((!entry.match_whole && stack.join("").ends_with(value))
            || (entry.match_whole && stack.join("") == value))
            && entry.condition(states)
            && (entry.next_prohibited.len() == 0
        || re1.is_match(&*get_next_char_noupdate(input, states).to_string()))
            && (entry.prohibited.len() == 0 || !re2.is_match(&*stack.join(""))) {
            return if value.len() == 0 {Some((stack.join(""), entry))}
                else {Some((String::from(value), entry))}
        }
    }
    None
}

fn lex(preinput: String, filename: String) -> Vec<Token> {
    if preinput.trim().len() == 0 {return vec![]};
    let input = preinput + "\n";
    let mut out: Vec<Token> = vec![];
    let mut stack: Vec<String> = vec![];

    let mut states = StateTracker;
    let mut c = input.chars().nth(0)?;
    stack.push(String::from(c));

    'main: loop {
        if c == "\r" && !states.is_literal_string {
            if let Ok(nc) = get_next_char(c, &input, &mut stack, &mut states) {c = nc;}
            else {break 'main};
            continue
        }
        if let Some((token, token_entry)) = get_token_entry(&stack, &states, &input) {
            if token_entry.categories.contains(&TokenCategory::LiteralStringEnd) {
                out.push(Token(
                    value: stack.join("")[0..stack.len() - token.len()],
                    type_: states.literal_string_type,
                    line: states.literal_string_line,
                    column: states.literal_string_column,
                    categories: [TokenCategory::Literal]
                ));
                stack.clear();
                stack.append(token.split("").collect());
                states.literal_string_line = 0;
                states.literal_string_column = 0;
            } else if token_entry.categories.contains(&TokenCategory::LiteralStringStart) {
                states.literal_string_line = states.position.line;
                states.literal_string_column = states.position.column + 1;
            }

            token_entry.state_changes(&states);
            states.prev_type = &token_entry.type_;

            out.push(Token(
                value: stack.join(""),
                type_: token_entry.type_,
                line: position.line,
                column: position.column+1-token.len(),
                categories: token_entry.categories
            ));
            stack.clear();
        }

        if let Ok(nc) = get_next_char(c, &input, &mut stack, &mut states) {c = nc;}
        else {break 'main;}
    }

    if stack.join("").trim().len() != 0 {
        out.push(Token(
            value: stack.join(""),
            type_: TokenType::Variable,
            line: position.line,
            column: position.column+1-stack.join("").trim().len(),
            categories: vec![]
        ))
    }

    let mut cursor = 0;
    let mut selected: Token = Token();
    let mut new_out = vec![];
    while cursor < out.len() {
        Some(&selected) = out.get(cursor);
        if selected.type_ == TokenType::DotOpr
            && (cursor != 0 && out.get(cursor-1)?.type_ == TokenType::LiteralNumber)
            && (cursor != out.len()-1 && out.get(cursor+1)?.type_ == TokenType::LiteralNumber) {
            new_out.pop();
            new_out.push(Token(
                value: out.get(cursor-1)?.value+"."+out.get(cursor+1).value,
                type_: TokenType::LiteralNumber,
                line: out.get(cursor-1)?.line,
                column: out.get(cursor-1)?.column,
                categories: [TokenCategory::Literal]
            ));
            cursor += 1;
        } else {new_out.push(out.get(cursor)?)}
        cursor += 1
    }

    if states.brackets.len != 0 {
        errors::error_pos(filename, states.position.line, states.position.column);
        errors::error_2_0_1(states.brackets.last())
    }

    new_out
}