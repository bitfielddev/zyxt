use crate::{
    lexer::{
        buffer::Buffer,
        comments::{lex_block_comment, lex_line_comment},
    },
    types::{
        element::PosRaw,
        token::{OprType, Token, TokenType},
    },
    ZyxtError,
};

pub fn lex_symbol(iter: &mut Buffer, tokens: &mut Vec<Token>) -> Result<(), ZyxtError> {
    let (char, pos) = iter.next().unwrap();
    let mut char = char.to_string();
    tokens.push(Token {
        ty: Some(match &*char {
            "+" => match iter.peek().as_mut() {
                Some(("=", _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Plus))
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
                    TokenType::AssignmentOpr(Some(OprType::Minus))
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
                    TokenType::AssignmentOpr(Some(OprType::AstMult))
                }
                Some(("/", _)) => todo!(),
                _ => TokenType::NormalOpr(OprType::AstMult),
            },
            "/" => match iter.peek() {
                Some(("=", _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::FractDiv))
                }
                Some(("*", _)) => {
                    iter.next().unwrap();
                    tokens.push(Token {
                        ty: Some(TokenType::Comment),
                        value: "/*".into(),
                        position: pos.to_owned(),
                        ..Default::default()
                    });
                    lex_block_comment(iter, tokens)?;
                    return Ok(());
                }
                Some(("/", _)) => {
                    iter.next().unwrap();
                    tokens.push(Token {
                        ty: Some(TokenType::Comment),
                        value: "//".into(),
                        position: pos.to_owned(),
                        ..Default::default()
                    });
                    lex_line_comment(iter, tokens)?;
                    return Ok(());
                }
                _ => TokenType::NormalOpr(OprType::FractDiv),
            },
            "^" => match iter.peek() {
                Some(("=", _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Power))
                }
                _ => TokenType::NormalOpr(OprType::Power),
            },
            "%" => match iter.peek() {
                Some(("=", _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Modulo))
                }
                _ => TokenType::NormalOpr(OprType::Modulo),
            },
            "~" => match iter.peek() {
                Some(("=", _)) => {
                    iter.next().unwrap();
                    TokenType::AssignmentOpr(Some(OprType::Concat))
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
                _ => TokenType::AssignmentOpr(None),
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
                return Err(
                    ZyxtError::error_2_1_1(char.to_owned()).with_pos_raw(&PosRaw {
                        position: pos.to_owned(),
                        raw: char.into(),
                    }),
                )
            }
        }),
        value: char.into(),
        position: pos.to_owned(),
        ..Default::default()
    });
    Ok(())
}
