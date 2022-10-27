use std::fmt::{Display, Formatter, Result};

use smol_str::SmolStr;

use crate::types::position::Position;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    pub value: SmolStr,
    pub ty: Option<TokenType>,
    pub position: Position,
    pub whitespace: SmolStr,
}
impl Default for Token {
    fn default() -> Self {
        Token {
            value: "".into(),
            ty: None,
            position: Position {
                ..Default::default()
            },
            whitespace: "".into(),
        }
    }
}
impl Token {
    pub fn get_raw(&self) -> String {
        format!("{}{}", &self.whitespace, &self.value)
    }
}
pub fn get_order(opr: &OprType) -> u8 {
    match *opr {
        OprType::Increment
        | OprType::Decrement
        | OprType::PlusSign
        | OprType::MinusSign
        | OprType::Not
        | OprType::Ref
        | OprType::Deref => 1,
        OprType::TypeCast => 2,
        OprType::Power => 3,
        //OprType::Root |
        //OprType::Logarithm => 4,
        OprType::DotMult => 5,
        OprType::AstMult
        | OprType::FractDiv
        | OprType::FloorfractDiv
        | OprType::CeilfractDiv
        | OprType::RoundfractDiv
        | OprType::Modulo => 6,
        OprType::CrossMult
        | OprType::Div
        | OprType::FloorDiv
        | OprType::CeilDiv
        | OprType::RoundDiv => 7,
        OprType::Plus | OprType::Minus | OprType::PlusMinus | OprType::MinusPlus => 8,
        OprType::Gt
        | OprType::Lt
        | OprType::Gteq
        | OprType::Lteq
        | OprType::Eq
        | OprType::Noteq
        | OprType::Istype
        | OprType::Isnttype
        | OprType::Is
        | OprType::Isnt
        | OprType::Iseq
        | OprType::Isnteq => 10,
        OprType::And => 14,
        OprType::Xor => 15,
        OprType::Or => 16,
        OprType::Concat => 18,
        OprType::Swap => 19,
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum OprType {
    PlusSign,
    MinusSign,
    Not,
    Power,
    DotMult,
    AstMult,
    CrossMult,
    Div,
    FloorDiv,
    CeilDiv,
    RoundDiv,
    FractDiv,
    FloorfractDiv,
    CeilfractDiv,
    RoundfractDiv,
    Modulo,
    Plus,
    Minus,
    PlusMinus,
    MinusPlus,
    And,
    Or,
    Xor,
    Gt,
    Lt,
    Gteq,
    Lteq,
    Eq,
    Noteq,
    Istype,
    Isnttype,
    Is,
    Isnt,
    Iseq,
    Isnteq,
    Concat,
    Swap,
    Ref,
    Deref,
    TypeCast,
}
impl Display for OprType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}
impl OprType {
    pub fn side(&self) -> Side {
        match self {
            OprType::Not
            | OprType::Ref
            | OprType::Deref
            | OprType::PlusSign
            | OprType::MinusSign => Side::Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Flag {
    Hoi,
    Pub,
    Priv,
    Prot,
    Const,
    Inst,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Keyword {
    If,
    Else,
    Elif,
    Do,
    While,
    For,
    Delete,
    Return,
    Proc,
    Fn,
    Pre,
    Defer,
    Class,
    Struct,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum TokenType {
    CommentStart,                   // //
    CommentEnd,                     // \n
    MultilineCommentStart,          // /*
    MultilineCommentEnd,            // */
    Flag(Flag),                     // hoi, pub, priv, prot, const
    UnaryOpr(OprType),              // \~, ++, ! etc
    AssignmentOpr(Option<OprType>), // =, += etc
    BinaryOpr(OprType), // +, -, /f, rt, \&, ==, >, is, &&, ||, ^^, .., ><, istype, isnttype etc
    DotOpr,             // .
    DeclarationOpr,     // :=
    LiteralMisc,        // true, null, etc
    LiteralNumber,      // 3, 24, -34.5 etc
    LiteralString,      // "abc" etc
    StatementEnd,       // ;
    OpenParen,          // (
    CloseParen,         // )
    OpenSquareParen,    // [
    CloseSquareParen,   // ]
    OpenCurlyParen,     // {
    CloseCurlyParen,    // }
    OpenAngleBracket,   // <
    CloseAngleBracket,  // >
    Comma,              // ,
    Colon,              // :
    Apostrophe,         // '
    Quote,              // "
    Bar,                // |
    Keyword(Keyword),   // if, while etc
    Comment,
    Ident,
    Whitespace,
}
impl TokenType {
    pub fn categories(&self) -> Vec<TokenCategory> {
        match self {
            TokenType::Ident => vec![TokenCategory::ValueStart, TokenCategory::ValueEnd],
            TokenType::LiteralNumber => vec![
                TokenCategory::Literal,
                TokenCategory::ValueStart,
                TokenCategory::ValueEnd,
            ],
            TokenType::OpenSquareParen
            | TokenType::OpenCurlyParen
            | TokenType::OpenParen
            | TokenType::CloseSquareParen
            | TokenType::CloseCurlyParen
            | TokenType::CloseParen => vec![
                TokenCategory::Parenthesis,
                TokenCategory::OpenParen,
                TokenCategory::ValueStart,
            ],
            TokenType::DotOpr => vec![TokenCategory::Operator],
            TokenType::StatementEnd => vec![
                TokenCategory::LiteralStringStart,
                TokenCategory::LiteralStringEnd,
            ],
            TokenType::AssignmentOpr(..) => vec![TokenCategory::Operator],
            TokenType::UnaryOpr(OprType::Not, ..)
            | TokenType::UnaryOpr(OprType::Ref, ..)
            | TokenType::UnaryOpr(OprType::Deref, ..) => {
                vec![TokenCategory::Operator, TokenCategory::ValueStart]
            }
            TokenType::UnaryOpr(OprType::Increment, ..)
            | TokenType::UnaryOpr(OprType::Decrement, ..) => {
                vec![TokenCategory::Operator, TokenCategory::ValueEnd]
            }
            TokenType::BinaryOpr(..) | TokenType::DeclarationOpr => vec![TokenCategory::Operator],
            TokenType::Bar => vec![
                TokenCategory::Literal,
                TokenCategory::ValueStart,
                TokenCategory::ValueEnd,
            ],
            TokenType::Comment => vec![
                TokenCategory::Literal,
                TokenCategory::ValueStart,
                TokenCategory::ValueEnd,
            ],
            TokenType::Keyword(..) | TokenType::Flag(..) => vec![TokenCategory::ValueStart],
            TokenType::LiteralMisc => vec![
                TokenCategory::Literal,
                TokenCategory::ValueStart,
                TokenCategory::ValueEnd,
            ],
            TokenType::Comma => vec![],
            _ => todo!("{:?}", self),
        }
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenCategory {
    Operator,
    Literal,
    Parenthesis,
    OpenParen,
    CloseParen,
    LiteralStringStart, //  marks the start of a literal string
    LiteralStringEnd,   // marks the end of a literal string
    ValueStart,
    ValueEnd,
}
