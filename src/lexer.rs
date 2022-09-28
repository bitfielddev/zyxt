use lazy_static::lazy_static;
use regex::Regex;
use smol_str::SmolStr;
use unicode_segmentation::UnicodeSegmentation;

use crate::types::{
    errors::ZyxtError,
    position::Position,
    token::{Keyword, OprType, Token, TokenType},
};

lazy_static! {
    static ref ALPHANUMERIC: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    static ref NUMERIC: Regex = Regex::new(r"^[0-9]+$").unwrap();
    static ref WHITESPACE: Regex = Regex::new(r"^\s+$").unwrap();
    static ref ALPHABETIC: Regex = Regex::new(r"^[a-zA-Z_]+$").unwrap();
}

fn clean_whitespaces(input: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = vec![];
    let mut whitespace_stack: SmolStr = "".into();

    for mut t in input {
        if t.type_ != TokenType::Whitespace {
            t.whitespace = whitespace_stack.to_owned();
            whitespace_stack = "".into();
            out.push(t);
        } else {
            whitespace_stack = format!("{whitespace_stack}{}", t.value).into();
        }
    }
    out
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
                    type_: TokenType::LiteralString,
                    value: raw.into(),
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
                        _ => TokenType::Ident,
                    },
                    value: raw.into(),
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
                    type_: TokenType::LiteralNumber,
                    value: raw.into(),
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
                tokens.last_mut().unwrap().value =
                    format!("{}{raw}", tokens.last().unwrap().value).into();
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
                    tokens.last_mut().unwrap().value =
                        format!("{}{raw}", tokens.last().unwrap().value).into();
                    return Ok(Some(&MainLexer));
                } else {
                    raw.push_str(char);
                }
            } else if char == "/" {
                raw.push_str(char);
                let (char, _) = iter.next().unwrap();
                if char == "*" {
                    tokens.last_mut().unwrap().value =
                        format!("{}{raw}", tokens.last().unwrap().value).into();
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
                    type_: TokenType::Whitespace,
                    value: raw.into(),
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
                                TokenType::UnaryOpr(OprType::Increment)
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
                                TokenType::UnaryOpr(OprType::Decrement)
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
                                    value: "/*".into(),
                                    position: pos,
                                    ..Default::default()
                                });
                                return Ok(Some(&BlockCommentLexer));
                            }
                            Some(("/", _)) => {
                                iter.next().unwrap();
                                tokens.push(Token {
                                    type_: TokenType::Comment,
                                    value: "//".into(),
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
                            _ => TokenType::UnaryOpr(OprType::Not),
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
                                TokenType::NormalOpr(OprType::And)
                            } // TODO pointer
                            _ => TokenType::UnaryOpr(OprType::Ref),
                        },
                        "|" => match iter.peek() {
                            Some(("|", _)) => {
                                iter.next().unwrap();
                                char.push('|');
                                TokenType::NormalOpr(OprType::Or)
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
                    value: char.into(),
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
