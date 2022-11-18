use std::fmt::{Display, Formatter, Result};

use smol_str::SmolStr;

use crate::types::position::{GetSpan, Span};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    pub value: SmolStr,
    pub ty: Option<TokenType>,
    pub span: Span,
    pub whitespace: SmolStr,
}
impl GetSpan for Token {
    fn span(&self) -> Option<Span> {
        Some(self.span.to_owned())
    }
}
impl Default for Token {
    fn default() -> Self {
        Token {
            value: "".into(),
            ty: None,
            span: Span::default(),
            whitespace: "".into(),
        }
    }
}
impl Token {
    pub fn get_raw(&self) -> String {
        format!("{}{}", &self.whitespace, &self.value)
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum OprType {
    UnPlus,
    UnMinus,
    Not,
    Pow,
    Mul,
    Div,
    Mod,
    Add,
    Sub,
    AddSub,
    SubAdd,
    And,
    Or,
    Gt,
    Lt,
    Ge,
    Le,
    Eq,
    Ne,
    Istype,
    Isnttype,
    Is,
    Isnt,
    Iseq,
    Isnteq,
    Concat,
    Ref,
    Deref,
    TypeCast,
}
impl Display for OprType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{self:?}")
    }
}
impl OprType {
    pub fn order(&self) -> usize {
        match self {
            OprType::UnPlus | OprType::UnMinus | OprType::Not | OprType::Ref | OprType::Deref => 1,
            OprType::TypeCast => 2,
            OprType::Pow => 3,
            OprType::Mul | OprType::Div | OprType::Mod => 6,
            OprType::Add | OprType::Sub | OprType::AddSub | OprType::SubAdd => 8,
            OprType::Gt
            | OprType::Lt
            | OprType::Ge
            | OprType::Le
            | OprType::Eq
            | OprType::Ne
            | OprType::Istype
            | OprType::Isnttype
            | OprType::Is
            | OprType::Isnt
            | OprType::Iseq
            | OprType::Isnteq => 10,
            OprType::And => 14,
            OprType::Or => 16,
            OprType::Concat => 18,
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
