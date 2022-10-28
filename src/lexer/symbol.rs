use crate::{
    lexer::{
        buffer::Buffer,
        comments::{lex_block_comment, lex_line_comment},
    },
    types::{
        position::PosRaw,
        token::{OprType, Token, TokenType},
    },
    ZyxtError,
};

pub fn lex_symbol(iter: &mut Buffer, tokens: &mut Vec<Token>) -> Result<(), ZyxtError> {
    let (char, pos) = iter.next().unwrap();
    let pos = pos.to_owned();
    let mut char = char.to_string();
    tokens.push(Token {
        ty: Some(match char.chars().next().unwrap() {
            '+' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Plus))
                }
                Some(('-', _)) => {
                    iter.next().unwrap();
                    char.push('-');
                    TokenType::BinaryOpr(OprType::PlusMinus)
                }
                _ => TokenType::BinaryOpr(OprType::Plus),
            },
            '-' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Minus))
                }
                Some(('+', _)) => {
                    iter.next().unwrap();
                    char.push('+');
                    TokenType::BinaryOpr(OprType::MinusPlus)
                }
                _ => TokenType::BinaryOpr(OprType::Minus),
            },
            '*' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::AstMult))
                }
                Some(('/', _)) => todo!(),
                _ => TokenType::BinaryOpr(OprType::AstMult),
            },
            '/' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::FractDiv))
                }
                Some(('*', _)) => {
                    iter.next().unwrap();
                    tokens.push(Token {
                        ty: Some(TokenType::Comment),
                        value: "/*".into(),
                        position: pos,
                        ..Default::default()
                    });
                    lex_block_comment(iter, tokens)?;
                    return Ok(());
                }
                Some(('/', _)) => {
                    iter.next().unwrap();
                    tokens.push(Token {
                        ty: Some(TokenType::Comment),
                        value: "//".into(),
                        position: pos,
                        ..Default::default()
                    });
                    lex_line_comment(iter, tokens)?;
                    return Ok(());
                }
                _ => TokenType::BinaryOpr(OprType::FractDiv),
            },
            '^' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Power))
                }
                _ => TokenType::BinaryOpr(OprType::Power),
            },
            '%' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Modulo))
                }
                _ => TokenType::BinaryOpr(OprType::Modulo),
            },
            '~' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    TokenType::AssignmentOpr(Some(OprType::Concat))
                }
                _ => TokenType::BinaryOpr(OprType::Concat),
            },
            '@' => TokenType::BinaryOpr(OprType::TypeCast),
            '=' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::BinaryOpr(OprType::Eq)
                }
                _ => TokenType::AssignmentOpr(None),
            },
            '!' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::BinaryOpr(OprType::Noteq)
                }
                _ => TokenType::UnaryOpr(OprType::Not),
            },
            '>' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::BinaryOpr(OprType::Gteq)
                }
                Some(('<', _)) => {
                    iter.next().unwrap();
                    char.push('<');
                    TokenType::BinaryOpr(OprType::Swap)
                } // TODO insertion
                _ => TokenType::BinaryOpr(OprType::Gt),
            },
            '<' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::BinaryOpr(OprType::Lteq)
                }
                _ => TokenType::BinaryOpr(OprType::Lt),
            },
            '&' => match iter.peek() {
                Some(('&', _)) => {
                    iter.next().unwrap();
                    char.push('&');
                    TokenType::BinaryOpr(OprType::And)
                } // TODO pointer
                _ => TokenType::UnaryOpr(OprType::Ref),
            },
            '|' => match iter.peek() {
                Some(('|', _)) => {
                    iter.next().unwrap();
                    char.push('|');
                    TokenType::BinaryOpr(OprType::Or)
                } // TODO |>
                _ => TokenType::Bar,
            },
            '.' => TokenType::DotOpr,
            ':' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::DeclarationOpr
                }
                _ => TokenType::Colon,
            },
            ';' => TokenType::StatementEnd,
            ',' => TokenType::Comma,
            '(' => TokenType::OpenParen,
            '[' => TokenType::OpenSquareParen,
            '{' => TokenType::OpenCurlyParen,
            ')' => TokenType::CloseParen,
            ']' => TokenType::CloseSquareParen,
            '}' => TokenType::CloseCurlyParen,
            _ => {
                return Err(
                    ZyxtError::error_2_1_1(char.to_owned()).with_pos_raw(&PosRaw {
                        position: pos,
                        raw: char.into(),
                    }),
                )
            }
        }),
        value: char.into(),
        position: pos,
        ..Default::default()
    });
    Ok(())
}
