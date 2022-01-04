use regex::Regex;
use derivative::Derivative;
use crate::syntax::{TokenEntry, TokenType, TOKEN_CATALOGUE, TokenCategory};
use crate::{errors, Token};

#[derive(Derivative)]
struct PositionTracker {
    #[derivative(Default(value = "[unknown]"))]
    filename: String,
    #[derivative(Default(value = "1"))]
    line: i32,
    #[derivative(Default(value = "1"))]
    column: i32,
    #[derivative(Default(value = "-1"))]
    prev_column: i32,
    char_pos: i32
}

#[derive(Derivative)]
pub(crate) struct StateTracker {
    position: PositionTracker,
    pub(crate) is_literal_string: bool,
    #[derivative(Default(value = "TokenType::Null"))]
    pub(crate) literal_string_type: TokenType,
    #[derivative(Default(value = "TokenType::Null"))]
    pub(crate) prev_type: TokenType,
    literal_string_line: i32,
    literal_string_column: i32,
    #[derivative(Default(value = "1"))]
    token_line: i32,
    #[derivative(Default(value = "1"))]
    token_column: i32,
    brackets: Vec<char>
}

fn get_next_char(c: char, input: &String, stack: &mut Vec<String>, states: &mut StateTracker) -> Result<char, bool> {
    if c == '\n' {
        states.position.line += 1;
        states.position.prev_column = states.position.column;
       states. position.column = 1;
    } else {states.position.column += 1;}
    states.position.char_pos += 1;
    match input.chars().nth(states.position.char_pos as usize) {
        Some(c) => {
            if (c == ' ' || c == '\n' || c == '\r') && states.is_literal_string {stack.push(String::from(c))}
            else if !(c == ' ' || c == '\n' || c == '\r') {stack.push(String::from(c))}
            Ok(c)
        }
        None => Err(false)
    }
}
fn get_next_char_noupdate(input: &String, states: &StateTracker) -> char {
    match input.chars().nth((states.position.char_pos + 1) as usize) {
        Some(c) => c,
        None => char::from(0)
    }
}

fn get_token_entry<'a>(stack: &Vec<String>, states: &StateTracker, input: &String) -> Option<(String, &'a TokenEntry<'a>)> {
    for (&prevalue, entry) in TOKEN_CATALOGUE.iter() {
        let mut value = prevalue;
        while value.len() != 0 && value.chars().nth(value.len()-1).unwrap() == ' ' {value = &value[..value.len() - 1]};
        let mut re1 = Regex::new(&*entry.next_prohibited).unwrap();
        let mut re2 = Regex::new(&*entry.prohibited).unwrap();

        if ((!entry.match_whole && stack.join("").ends_with(value))
            || (entry.match_whole && stack.join("") == value))
            && (entry.condition)(states)
            && (entry.next_prohibited.len() == 0
        || re1.is_match(&*get_next_char_noupdate(input, states).to_string()))
            && (entry.prohibited.len() == 0 || !re2.is_match(&*stack.join(""))) {
            return if value.len() == 0 {Some((stack.join(""), entry))}
                else {Some((String::from(value), entry))}
        }
    }
    None
}

pub fn lex(preinput: String, filename: &String) -> Vec<Token> {
    if preinput.trim().len() == 0 {return Vec::new()};
    let input = preinput + "\n";
    let mut out: Vec<Token> = vec![];
    let mut stack: Vec<String> = vec![];

    let mut states = StateTracker{
        position: PositionTracker {
            filename: "".to_string(),
            line: 0,
            column: 0,
            prev_column: 0,
            char_pos: 0
        },
        is_literal_string: false,
        literal_string_type: TokenType::Null,
        prev_type: TokenType::Null,
        literal_string_line: 0,
        literal_string_column: 0,
        token_line: 0,
        token_column: 0,
        brackets: vec![]
    };
    let mut c = input.chars().nth(0).unwrap();
    stack.push(String::from(c));

    'main: loop {
        if c == '\r' && !states.is_literal_string {
            if let Ok(nc) = get_next_char(c, &input, &mut stack, &mut states) {c = nc;}
            else {break 'main};
            continue
        }
        if let Some((token, token_entry)) = get_token_entry(&stack, &states, &input) {
            if token_entry.categories.contains(&TokenCategory::LiteralStringEnd) {
                out.push(Token {
                    value: String::from(&stack.join("")[0..stack.len() - token.len()]),
                    type_: *states.literal_string_type,
                    line: states.literal_string_line,
                    column: states.literal_string_column,
                    categories: vec![TokenCategory::Literal]
                });
                stack.clear();
                stack.append(&mut Vec::from_iter(token.split("").map(|s| s.to_string())));
                states.literal_string_line = 0;
                states.literal_string_column = 0;
            } else if token_entry.categories.contains(&TokenCategory::LiteralStringStart) {
                states.literal_string_line = states.position.line;
                states.literal_string_column = states.position.column + 1;
            }

            (token_entry.state_changes)(&mut states);
            states.prev_type = *token_entry.type_;

            out.push(Token{
                value: stack.join(""),
                type_: *token_entry.type_,
                line: states.position.line,
                column: states.position.column + 1 - token.len() as i32,
                categories: token_entry.categories.to_vec()
            });
            stack.clear();
        }

        if let Ok(nc) = get_next_char(c, &input, &mut stack, &mut states) {c = nc;}
        else {break 'main;}
    }

    if stack.join("").trim().len() != 0 {
        out.push(Token {
            value: stack.join(""),
            type_: TokenType::Variable,
            line: states.position.line,
            column: states.position.column + 1 - stack.join("").trim().len() as i32,
            categories: vec![]
        })
    }

    let mut cursor: i32 = 0;
    let mut selected: Token = Token{
        value: String::from(""),
        type_: TokenType::CommentStart,
        line: 0,
        column: 0,
        categories: vec![]
    };
    let mut new_out = vec![];
    while cursor < out.len() as i32 {
        selected = *out.get(cursor).unwrap();
        if selected.type_ == TokenType::DotOpr
            && (cursor != 0 && out.get(cursor-1).unwrap().type_ == TokenType::LiteralNumber)
            && (cursor != (out.len() - 1) as i32 && out.get(cursor+1).unwrap().type_ == TokenType::LiteralNumber) {
            new_out.pop();
            new_out.push(Token {
                value: format!("{}.{}", &*out.get(cursor - 1).unwrap().value, &*out.get(cursor + 1).unwrap().value),
                type_: TokenType::LiteralNumber,
                line: out.get(cursor - 1).unwrap().line,
                column: out.get(cursor - 1).unwrap().column,
                categories: vec![TokenCategory::Literal]
            });
            cursor += 1;
        } else {new_out.push(*out.get(&cursor).unwrap())}
        cursor += 1
    }

    if states.brackets.len() != 0 {
        errors::error_pos(filename, states.position.line, states.position.column);
        errors::error_2_0_1(states.brackets.last().unwrap().to_string())
    }

    new_out
}