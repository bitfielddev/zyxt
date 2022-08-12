use crate::objects::errors::ZyxtError;
use crate::objects::position::Position;
use crate::objects::token::{Keyword, OprType, Side, Token, TokenCategory, TokenType};
use crate::objects::token_entries::{
    compound_token_entries_1, compound_token_entries_2, side_dependent_token_entries,
    singular_token_entries, CompoundTokenEntry, Pattern,
};
use lazy_static::lazy_static;
use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref ALPHANUMERIC: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    static ref NUMERIC: Regex = Regex::new(r"^[0-9]+$").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"^\s+$").unwrap();
    static ref ALPHABETIC: Regex = Regex::new(r"^[a-zA-Z_]+$").unwrap();
}

fn lex_stage1(input: String, filename: &str) -> Result<Vec<Token>, ZyxtError> {
    let mut out: Vec<Token> = vec![];
    let mut pos = Position {
        filename: filename.to_string(),
        ..Default::default()
    };
    let token_entries = singular_token_entries();
    for c in input.chars() {
        let mut found = false;
        for entry in token_entries.iter() {
            if if let Some(re) = &entry.re {
                re.is_match(&*c.to_string())
            } else {
                c == entry.value
            } {
                out.push(Token {
                    value: c.to_string(),
                    type_: entry.type_,
                    position: pos.to_owned(),
                    ..Default::default()
                });
                pos.next_char(c);
                found = true;
                break;
            }
        }
        if !found {
            out.push(Token {
                value: c.to_string(),
                type_: TokenType::Null,
                position: pos.to_owned(),
                ..Default::default()
            });
            pos.next_char(c);
        }
    }
    Ok(out)
}

fn is_literal_match(out: &[Token], entry: &CompoundTokenEntry) -> Option<usize> {
    let mut cursor = out.len() - 1;
    let mut selected = out.last().unwrap();
    let mut match_count = 1usize;
    let mut indent = 0u8;
    while selected.type_ != entry.pair? || cursor == out.len() - 1 || indent != 0 {
        if selected.type_ == entry.type_ && selected != out.last().unwrap() {
            indent += 1;
        }
        if selected.type_ == entry.pair? && indent != 0 {
            indent -= 1;
        }

        match_count += 1;
        if cursor == 0 {
            return None;
        } // raise error
        cursor -= 1;
        selected = out.get(cursor)?;
    }
    Some(match_count)
}

fn is_match(combination: &[Pattern<'_>], out: &[Token]) -> Option<usize> {
    let mut _cursor = out.len() - 1;
    let mut selected = out.last().unwrap();
    let mut match_count = 0usize;
    for (i, p) in combination.iter().rev().enumerate() {
        macro_rules! update_cursor {
            () => {
                match_count += 1;
                if _cursor == 0 && combination.len() != i + 1 {
                    return None;
                } else if _cursor == 0 {
                    return Some(match_count);
                }
                _cursor -= 1;
                selected = out.get(_cursor).unwrap();
            };
        }
        match p {
            Pattern::Value(token_type, value) => {
                if i == 0 {
                    update_cursor!();
                    continue;
                }
                if token_type != &TokenType::Null && token_type != &selected.type_ {
                    return None;
                }
                if value != &selected.value {
                    return None;
                }
                update_cursor!();
            }
            Pattern::Token(token_type) => {
                if i == 0 {
                    update_cursor!();
                    continue;
                }
                if token_type != &TokenType::Null && token_type != &selected.type_ {
                    return None;
                }
                update_cursor!();
            }
            Pattern::Vartokens(token_type) => {
                while token_type == &TokenType::Null || token_type == &selected.type_ {
                    update_cursor!();
                }
            }
            Pattern::Re(token_type, re) => {
                if i == 0 {
                    update_cursor!();
                    continue;
                }
                if token_type != &TokenType::Null && token_type != &selected.type_ {
                    return None;
                }
                if !Regex::new(re).unwrap().is_match(&selected.value) {
                    return None;
                }
                update_cursor!();
            }
        }
    }
    if match_count == 0 {
        None
    } else {
        Some(match_count)
    }
}

fn lex_stage2(input: Vec<Token>) -> Result<Vec<Token>, ZyxtError> {
    let mut out: Vec<Token> = vec![];

    let token_entries = compound_token_entries_1();
    for t in input {
        out.push(t.to_owned());
        for entry in token_entries.iter() {
            let (Pattern::Value(token_type, ..)
            | Pattern::Token(token_type)
            | Pattern::Vartokens(token_type)
            | Pattern::Re(token_type, ..)) = entry.combination.last().unwrap();
            if token_type == &TokenType::Null
                || token_type == &t.type_ && {
                    if let Pattern::Value(_, value) = entry.combination.last().unwrap() {
                        t.value == *value
                    } else {
                        true
                    }
                }
            {
                if let Some(count) = is_match(entry.combination, &out) {
                    let pos = out.get(out.len() - count).unwrap().position.to_owned();
                    let value = out.drain(out.len() - count..).map(|t| t.value).collect();
                    out.push(Token {
                        value,
                        type_: entry.type_,
                        position: pos,
                        ..Default::default()
                    });
                }
            }
        }
    }
    Ok(out)
}

fn lex_stage3(input: Vec<Token>) -> Result<Vec<Token>, ZyxtError> {
    let mut out: Vec<Token> = vec![];

    let token_entries = compound_token_entries_2();
    for t in input {
        out.push(t.to_owned());
        for entry in token_entries.iter() {
            let (Pattern::Value(token_type, ..)
            | Pattern::Token(token_type)
            | Pattern::Vartokens(token_type)
            | Pattern::Re(token_type, ..)) = entry.combination.last().unwrap();
            if token_type == &TokenType::Null
                || token_type == &t.type_ && {
                    if let Pattern::Value(_, value) = entry.combination.last().unwrap() {
                        t.value == *value
                    } else {
                        true
                    }
                }
            {
                if entry.literal {
                    if let Some(count) = is_literal_match(&out, entry) {
                        let pos = out.get(out.len() - count).unwrap().position.to_owned();
                        let value = out.drain(out.len() - count..).map(|t| t.value).collect();
                        out.push(Token {
                            value,
                            type_: entry.type_,
                            position: pos,
                            ..Default::default()
                        });
                    }
                } else if let Some(count) = is_match(entry.combination, &out) {
                    let pos = out.get(out.len() - count).unwrap().position.to_owned();
                    let value = out.drain(out.len() - count..).map(|t| t.value).collect();
                    out.push(Token {
                        value,
                        type_: entry.type_,
                        position: pos,
                        ..Default::default()
                    });
                }
            }
        }
    }
    Ok(out)
}

fn lex_stage4(input: Vec<Token>) -> Result<Vec<Token>, ZyxtError> {
    let mut out: Vec<Token> = vec![];

    let token_entries = side_dependent_token_entries();
    let mut type_list = token_entries.iter().map(|e| e.from);
    for (i, t) in input.iter().enumerate() {
        if !type_list.any(|a| a == t.type_) {
            out.push(t.to_owned());
            continue;
        }
        let token_entry = token_entries.iter().find(|e| e.from == t.type_).unwrap();
        let prev_token = if i != 0 { Some(&out[i - 1]) } else { None };
        let next_token = if i != input.len() - 1 {
            Some(&input[i + 1])
        } else {
            None
        };
        if (token_entry.side == Side::Left
            && (/*next_token != None
                && next_token.unwrap().categories.contains(&TokenCategory::ValueStart)
            && (*/prev_token == None
                || !prev_token
                    .unwrap()
                    .type_
                    .categories()
                    .contains(&TokenCategory::ValueEnd)/*)*/))
            || (token_entry.side == Side::Right
                && (/*prev_token != None
                    && prev_token.unwrap().categories.contains(&TokenCategory::ValueStart)
                && (*/next_token == None
                    || !next_token
                        .unwrap()
                        .type_
                        .categories()
                        .contains(&TokenCategory::ValueStart)/*)*/))
        {
            out.push(Token {
                type_: token_entry.type_,
                ..t.to_owned()
            })
        } else {
            out.push(t.to_owned())
        }
    }
    Ok(out)
}

fn clean_whitespaces(input: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = vec![];
    let mut whitespace_stack = "".to_string();

    for mut t in input {
        if t.type_ != TokenType::Whitespace {
            t.whitespace = whitespace_stack.to_owned();
            whitespace_stack = "".to_string();
            out.push(t);
        } else {
            whitespace_stack.push_str(&*t.value);
        }
    }
    out
}

fn check_no_unknown_tokens(input: &[Token]) -> Result<(), ZyxtError> {
    for token in input.iter() {
        if token.type_ == TokenType::Null {
            return Err(ZyxtError::error_2_1_1(token.value.to_owned())
                .with_pos_and_raw(&token.position, &token.value));
        }
    }
    Ok(())
}

pub fn old_lex(preinput: String, filename: &str) -> Result<Vec<Token>, ZyxtError> {
    if preinput.trim().is_empty() {
        return Ok(vec![]);
    };
    let input = preinput + "\n";

    let out1 = lex_stage1(input, filename)?;
    let out2 = lex_stage2(out1)?;
    let out3 = lex_stage3(out2)?;
    let out4 = clean_whitespaces(out3);
    let out5 = lex_stage4(out4)?;
    check_no_unknown_tokens(&out5)?;
    Ok(out5)
}

#[derive(Clone)]
pub struct Buffer<'a> {
    content: Vec<(&'a str, Position)>,
    cursor: usize,
    started: bool,
}
impl<'a> Iterator for Buffer<'a> {
    type Item = (&'a str, Position);

    fn next(&mut self) -> Option<Self::Item> {
        if self.started {
            self.cursor += 1;
        } else {
            self.started = true;
        }
        self.content.get(self.cursor).cloned()
    }
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
    pub fn peek(&self) -> Option<(&str, Position)> {
        self.content
            .get(if self.started { self.cursor + 1 } else { 0 })
            .cloned()
    }
}

trait Lexer {
    fn lex(
        &self,
        iter: &mut Buffer,
        tokens: &mut Vec<Token>,
    ) -> Result<Option<&'static dyn Lexer>, ZyxtError>;
}

struct TextLiteralLexer;
impl Lexer for TextLiteralLexer {
    fn lex(
        &self,
        iter: &mut Buffer,
        tokens: &mut Vec<Token>,
    ) -> Result<Option<&'static dyn Lexer>, ZyxtError> {
        iter.next().unwrap();
        let mut raw = "\"".to_string();
        let pos = iter.peek().ok_or_else(|| todo!())?.1;
        while let Some((char, _)) = iter.next() {
            if char == "\"" {
                raw.push('"');
                tokens.push(Token {
                    // TODO auto-get categories and whitespace
                    type_: TokenType::LiteralString,
                    value: raw,
                    position: pos,
                    ..Default::default()
                });
                return Ok(Some(&MainLexer));
            } else if char == "\\" {
                if let Some((char, _)) = iter.next() {
                    let new_str = match char {
                        "\"" => "\"",
                        "\\" => "\\",
                        "n" => "\n",
                        "r" => "\r",
                        "t" => "\t", // TODO more escapes
                        _ => {
                            raw.push('\\');
                            char
                        }
                    };
                    raw.push_str(new_str);
                } else {
                    todo!()
                }
            } else {
                raw.push_str(char);
            }
        }
        Ok(None)
    }
}

struct WordLexer;
impl Lexer for WordLexer {
    fn lex(
        &self,
        iter: &mut Buffer,
        tokens: &mut Vec<Token>,
    ) -> Result<Option<&'static dyn Lexer>, ZyxtError> {
        let mut raw = "".to_string();
        let pos = iter.peek().unwrap().1;
        while let Some((char, _)) = iter.peek() {
            if ALPHANUMERIC.is_match(char) {
                raw.push_str(char);
                iter.next().unwrap();
            } else {
                tokens.push(Token {
                    // TODO auto-get categories and whitespace
                    type_: match raw.as_str() {
                        "true" => TokenType::LiteralMisc,
                        "false" => TokenType::LiteralMisc,
                        "if" => TokenType::Keyword(Keyword::If),
                        "else" => TokenType::Keyword(Keyword::Else),
                        "elif" => TokenType::Keyword(Keyword::Elif),
                        "do" => TokenType::Keyword(Keyword::Do),
                        "while" => TokenType::Keyword(Keyword::While),
                        "for" => TokenType::Keyword(Keyword::For),
                        "del" => TokenType::Keyword(Keyword::Delete),
                        "ret" => TokenType::Keyword(Keyword::Return),
                        "proc" => TokenType::Keyword(Keyword::Proc),
                        "fn" => TokenType::Keyword(Keyword::Fn),
                        "pre" => TokenType::Keyword(Keyword::Pre),
                        "defer" => TokenType::Keyword(Keyword::Defer),
                        "class" => TokenType::Keyword(Keyword::Class),
                        "struct" => TokenType::Keyword(Keyword::Struct),
                        _ => TokenType::Variable,
                    },
                    value: raw,
                    position: pos,
                    ..Default::default()
                });
                return Ok(Some(&MainLexer));
            }
        }
        Ok(None)
    }
}

struct NumberLexer;
impl Lexer for NumberLexer {
    fn lex(
        &self,
        iter: &mut Buffer,
        tokens: &mut Vec<Token>,
    ) -> Result<Option<&'static dyn Lexer>, ZyxtError> {
        let mut raw = "".to_string();
        let pos = iter.peek().unwrap().1;
        let mut dotted = false;
        while let Some((char, _)) = iter.peek() {
            if NUMERIC.is_match(char) {
                raw.push_str(char);
                iter.next().unwrap();
            } else if char == "." && !dotted {
                dotted = true;
                raw.push_str(char);
                iter.next().unwrap();
            } else {
                tokens.push(Token {
                    // TODO auto-get categories and whitespace
                    type_: TokenType::LiteralNumber,
                    value: raw,
                    position: pos,
                    ..Default::default()
                });
                return Ok(Some(&MainLexer));
            }
        }
        Ok(None)
    }
}

struct LineCommentLexer;
impl Lexer for LineCommentLexer {
    #[allow(clippy::while_let_on_iterator)]
    fn lex(
        &self,
        iter: &mut Buffer,
        tokens: &mut Vec<Token>,
    ) -> Result<Option<&'static dyn Lexer>, ZyxtError> {
        let mut raw = "".to_string();
        while let Some((char, _)) = iter.next() {
            raw.push_str(char);
            if char == "\n" {
                tokens.last_mut().unwrap().value.push_str(&raw);
                return Ok(Some(&MainLexer));
            }
        }
        Ok(None)
    }
}

struct BlockCommentLexer;
impl Lexer for BlockCommentLexer {
    fn lex(
        &self,
        iter: &mut Buffer,
        tokens: &mut Vec<Token>,
    ) -> Result<Option<&'static dyn Lexer>, ZyxtError> {
        let mut raw = "".to_string();
        while let Some((char, _)) = iter.next() {
            raw.push_str(char);
            if char == "*" {
                raw.push_str(char);
                let (char, _) = iter.next().unwrap();
                if char == "/" {
                    tokens.last_mut().unwrap().value.push_str(&raw);
                    return Ok(Some(&MainLexer));
                } else {
                    raw.push_str(char);
                }
            } else if char == "/" {
                raw.push_str(char);
                let (char, _) = iter.next().unwrap();
                if char == "*" {
                    tokens.last_mut().unwrap().value.push_str(&raw.clone());
                    raw = "".to_string();
                    BlockCommentLexer.lex(iter, tokens)?.unwrap();
                } else {
                    raw.push_str(char);
                }
            } else {
                raw.push_str(char);
            }
        }
        Ok(None)
    }
}

struct WhitespaceLexer;
impl Lexer for WhitespaceLexer {
    #[allow(clippy::while_let_on_iterator)]
    fn lex(
        &self,
        iter: &mut Buffer,
        tokens: &mut Vec<Token>,
    ) -> Result<Option<&'static dyn Lexer>, ZyxtError> {
        let mut raw = "".to_string();
        let pos = if let Some((_, pos)) = iter.peek() {
            pos
        } else {
            return Ok(None);
        };
        while let Some((char, _)) = iter.peek() {
            if WHITESPACE.is_match(char) {
                raw.push_str(char);
                iter.next().unwrap();
            } else {
                tokens.push(Token {
                    // TODO auto-get categories and whitespace
                    type_: TokenType::Whitespace,
                    value: raw,
                    position: pos,
                    ..Default::default()
                });
                return Ok(Some(&MainLexer));
            }
        }
        Ok(None)
    }
}

struct MainLexer;
impl Lexer for MainLexer {
    fn lex(
        &self,
        iter: &mut Buffer,
        tokens: &mut Vec<Token>,
    ) -> Result<Option<&'static dyn Lexer>, ZyxtError> {
        while let Some((char, pos)) = iter.to_owned().peek() {
            if char == "\"" {
                return Ok(Some(&TextLiteralLexer));
            } else if ALPHABETIC.is_match(char) {
                return Ok(Some(&WordLexer));
            } else if WHITESPACE.is_match(char) {
                return Ok(Some(&WhitespaceLexer));
            } else if NUMERIC.is_match(char) {
                return Ok(Some(&NumberLexer));
            } else {
                let mut char = iter.next().unwrap().0.to_string();
                tokens.push(Token {
                    type_: match char.as_str() {
                        "+" => match iter.peek().as_mut() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::AssignmentOpr(OprType::Plus)
                            }
                            Some(("+", _)) => {
                                iter.next().unwrap();
                                char.push('+');
                                TokenType::UnaryOpr(OprType::Increment, Side::Right)
                            }
                            Some(("-", _)) => {
                                iter.next().unwrap();
                                char.push('-');
                                TokenType::NormalOpr(OprType::PlusMinus)
                            }
                            _ => TokenType::NormalOpr(OprType::Plus),
                        },
                        "-" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::AssignmentOpr(OprType::Minus)
                            }
                            Some(("-", _)) => {
                                iter.next().unwrap();
                                char.push('-');
                                TokenType::UnaryOpr(OprType::Decrement, Side::Right)
                            }
                            Some(("+", _)) => {
                                iter.next().unwrap();
                                char.push('+');
                                TokenType::NormalOpr(OprType::MinusPlus)
                            }
                            _ => TokenType::NormalOpr(OprType::Minus),
                        },
                        "*" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::AssignmentOpr(OprType::AstMult)
                            }
                            Some(("/", _)) => todo!(),
                            _ => TokenType::NormalOpr(OprType::AstMult),
                        },
                        "/" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::AssignmentOpr(OprType::FractDiv)
                            }
                            Some(("*", _)) => {
                                iter.next().unwrap();
                                tokens.push(Token {
                                    type_: TokenType::Comment,
                                    value: "/*".to_string(),
                                    position: pos,
                                    ..Default::default()
                                });
                                return Ok(Some(&BlockCommentLexer));
                            }
                            Some(("/", _)) => {
                                iter.next().unwrap();
                                tokens.push(Token {
                                    type_: TokenType::Comment,
                                    value: "//".to_string(),
                                    position: pos,
                                    ..Default::default()
                                });
                                return Ok(Some(&LineCommentLexer));
                            }
                            _ => TokenType::NormalOpr(OprType::FractDiv),
                        },
                        "^" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::AssignmentOpr(OprType::Power)
                            }
                            _ => TokenType::NormalOpr(OprType::Power),
                        },
                        "%" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::AssignmentOpr(OprType::Modulo)
                            }
                            _ => TokenType::NormalOpr(OprType::Modulo),
                        },
                        "~" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                TokenType::AssignmentOpr(OprType::Concat)
                            }
                            _ => TokenType::NormalOpr(OprType::Concat),
                        },
                        "@" => TokenType::NormalOpr(OprType::TypeCast),
                        "=" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::NormalOpr(OprType::Eq)
                            }
                            _ => TokenType::AssignmentOpr(OprType::Null),
                        },
                        "!" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::NormalOpr(OprType::Noteq)
                            }
                            _ => TokenType::UnaryOpr(OprType::Not, Side::Left),
                        },
                        ">" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::NormalOpr(OprType::Gteq)
                            }
                            Some(("<", _)) => {
                                iter.next().unwrap();
                                char.push('<');
                                TokenType::NormalOpr(OprType::Swap)
                            } // TODO insertion
                            _ => TokenType::NormalOpr(OprType::Gt),
                        },
                        "<" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::NormalOpr(OprType::Lteq)
                            }
                            _ => TokenType::NormalOpr(OprType::Lt),
                        },
                        "&" => match iter.peek() {
                            Some(("&", _)) => {
                                iter.next().unwrap();
                                char.push('&');
                                TokenType::AssignmentOpr(OprType::And)
                            } // TODO pointer
                            _ => TokenType::UnaryOpr(OprType::Ref, Side::Left),
                        },
                        "|" => match iter.peek() {
                            Some(("|", _)) => {
                                iter.next().unwrap();
                                char.push('|');
                                TokenType::AssignmentOpr(OprType::Or)
                            } // TODO |>
                            _ => TokenType::Bar,
                        },
                        "." => TokenType::DotOpr,
                        ":" => match iter.peek() {
                            Some(("=", _)) => {
                                iter.next().unwrap();
                                char.push('=');
                                TokenType::DeclarationOpr
                            }
                            _ => TokenType::Colon,
                        },
                        ";" => TokenType::StatementEnd,
                        "," => TokenType::Comma,
                        "(" => TokenType::OpenParen,
                        "[" => TokenType::OpenSquareParen,
                        "{" => TokenType::OpenCurlyParen,
                        ")" => TokenType::CloseParen,
                        "]" => TokenType::CloseSquareParen,
                        "}" => TokenType::CloseCurlyParen,
                        _ => {
                            return Err(ZyxtError::error_2_1_1(char.to_owned())
                                .with_pos_and_raw(&pos, &char.to_string()))
                        }
                    },
                    value: char,
                    position: pos.to_owned(),
                    ..Default::default()
                });
            }
        }
        Ok(None)
    }
}

pub fn lex(preinput: String, filename: &str) -> Result<Vec<Token>, ZyxtError> {
    if preinput.trim().is_empty() {
        return Ok(vec![]);
    };
    let input = preinput + "\n";

    let pos = Position {
        filename: filename.to_string(),
        ..Default::default()
    };
    let mut iter = Buffer::new(&input, pos);
    let mut tokens = vec![];
    let mut lexer: &'static dyn Lexer = &MainLexer;
    while let Some(new_lexer) = lexer.lex(&mut iter, &mut tokens)? {
        lexer = new_lexer;
    }
    tokens = clean_whitespaces(tokens);
    Ok(tokens)
}
