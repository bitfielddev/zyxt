use tracing::trace;

use crate::{
    lexer::{buffer::Buffer, ALPHANUMERIC},
    types::{
        position::Span,
        token::{Flag, Keyword, Token, TokenType},
    },
    ZResult,
};

pub fn lex_word(iter: &mut Buffer, tokens: &mut Vec<Token>) -> ZResult<()> {
    let mut raw = "".to_string();
    let _init_pos = iter.peek().unwrap().1;
    while let Some((char, pos)) = iter.peek() {
        trace!(?char, ?pos);
        if ALPHANUMERIC.is_match(&char.to_string()) {
            raw.push(char);
            iter.next().unwrap();
        } else {
            tokens.push(Token {
                ty: Some(match raw.as_str() {
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
                    "const" => TokenType::Flag(Flag::Const),
                    "hoi" => TokenType::Flag(Flag::Hoi),
                    "pub" => TokenType::Flag(Flag::Pub),
                    "inst" => TokenType::Flag(Flag::Inst),
                    "priv" => TokenType::Flag(Flag::Priv),
                    "prot" => TokenType::Flag(Flag::Prot),
                    _ => TokenType::Ident,
                }),
                value: (&raw).into(),
                span: Span::new(pos, &raw),
                ..Default::default()
            });
            return Ok(());
        }
    }
    Ok(())
}
