use crate::{
    lexer::{
        buffer::Buffer,
        comments::{lex_block_comment, lex_line_comment},
    },
    types::{
        position::Span,
        token::{AccessType, OprType, Token, TokenType},
    },
    ZError, ZResult,
};

#[tracing::instrument(skip_all)]
pub fn lex_symbol(iter: &mut Buffer, tokens: &mut Vec<Token>) -> ZResult<()> {
    let (char, pos) = iter.next().unwrap();
    let pos = pos.to_owned();
    let mut char = char.to_string();
    tokens.push(Token {
        ty: Some(match char.chars().next().unwrap() {
            '+' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Add))
                }
                Some(('-', _)) => {
                    iter.next().unwrap();
                    char.push('-');
                    TokenType::BinaryOpr(OprType::AddSub)
                }
                _ => TokenType::BinaryOpr(OprType::Add),
            },
            '-' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Sub))
                }
                Some(('+', _)) => {
                    iter.next().unwrap();
                    char.push('+');
                    TokenType::BinaryOpr(OprType::SubAdd)
                }
                _ => TokenType::BinaryOpr(OprType::Sub),
            },
            '*' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Mul))
                }
                Some(('/', _)) => todo!(),
                _ => TokenType::BinaryOpr(OprType::Mul),
            },
            '/' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Div))
                }
                Some(('*', _)) => {
                    iter.next().unwrap();
                    tokens.push(Token {
                        ty: Some(TokenType::Comment),
                        value: "/*".into(),
                        span: Span::new(pos, "/*"),
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
                        span: Span::new(pos, "//"),
                        ..Default::default()
                    });
                    lex_line_comment(iter, tokens)?;
                    return Ok(());
                }
                _ => TokenType::BinaryOpr(OprType::Div),
            },
            '^' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Pow))
                }
                _ => TokenType::BinaryOpr(OprType::Pow),
            },
            '%' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::AssignmentOpr(Some(OprType::Mod))
                }
                _ => TokenType::BinaryOpr(OprType::Mod),
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
                    TokenType::BinaryOpr(OprType::Ne)
                }
                _ => TokenType::UnaryOpr(OprType::Not),
            },
            '>' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::BinaryOpr(OprType::Ge)
                } // TODO insertion
                _ => TokenType::BinaryOpr(OprType::Gt),
            },
            '<' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::BinaryOpr(OprType::Le)
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
            '.' => TokenType::DotOpr(AccessType::Field),
            ':' => match iter.peek() {
                Some(('=', _)) => {
                    iter.next().unwrap();
                    char.push('=');
                    TokenType::DeclarationOpr
                }
                Some(('.', _)) => {
                    iter.next().unwrap();
                    char.push('.');
                    TokenType::DotOpr(AccessType::Method)
                }
                Some((':', _)) => {
                    iter.next().unwrap();
                    char.push(':');
                    TokenType::DotOpr(AccessType::Namespace)
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
            _ => return Err(ZError::l001().with_span(Span::new(pos, &char))),
        }),
        value: (&char).into(),
        span: Span::new(pos, &char),
        ..Default::default()
    });
    Ok(())
}
