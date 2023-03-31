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
        Self {
            value: "".into(),
            ty: None,
            span: Span::default(),
            whitespace: "".into(),
        }
    }
}
impl Token {
    #[must_use]
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
    #[must_use]
    pub const fn order(&self) -> usize {
        match self {
            Self::UnPlus | Self::UnMinus | Self::Not | Self::Ref | Self::Deref => 1,
            Self::TypeCast => 2,
            Self::Pow => 3,
            Self::Mul | Self::Div | Self::Mod => 6,
            Self::Add | Self::Sub | Self::AddSub | Self::SubAdd => 8,
            Self::Gt
            | Self::Lt
            | Self::Ge
            | Self::Le
            | Self::Eq
            | Self::Ne
            | Self::Istype
            | Self::Isnttype
            | Self::Is
            | Self::Isnt
            | Self::Iseq
            | Self::Isnteq => 10,
            Self::And => 14,
            Self::Or => 16,
            Self::Concat => 18,
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
pub enum AccessType {
    Field,
    Method,
    Namespace,
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
    DotOpr(AccessType), // .
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
    #[must_use]
    pub fn categories(&self) -> Vec<TokenCategory> {
        match self {
            Self::Ident => vec![TokenCategory::ValueStart, TokenCategory::ValueEnd],
            Self::LiteralNumber => vec![
                TokenCategory::Literal,
                TokenCategory::ValueStart,
                TokenCategory::ValueEnd,
            ],
            Self::OpenSquareParen
            | Self::OpenCurlyParen
            | Self::OpenParen
            | Self::CloseSquareParen
            | Self::CloseCurlyParen
            | Self::CloseParen => vec![
                TokenCategory::Parenthesis,
                TokenCategory::OpenParen,
                TokenCategory::ValueStart,
            ],
            Self::DotOpr(..) => vec![TokenCategory::Operator],
            Self::StatementEnd => vec![
                TokenCategory::LiteralStringStart,
                TokenCategory::LiteralStringEnd,
            ],
            Self::AssignmentOpr(..) => vec![TokenCategory::Operator],
            Self::UnaryOpr(OprType::Not | OprType::Ref | OprType::Deref, ..) => {
                vec![TokenCategory::Operator, TokenCategory::ValueStart]
            }
            Self::BinaryOpr(..) | Self::DeclarationOpr => vec![TokenCategory::Operator],
            Self::Bar => vec![
                TokenCategory::Literal,
                TokenCategory::ValueStart,
                TokenCategory::ValueEnd,
            ],
            Self::Comment => vec![
                TokenCategory::Literal,
                TokenCategory::ValueStart,
                TokenCategory::ValueEnd,
            ],
            Self::Keyword(..) | Self::Flag(..) => vec![TokenCategory::ValueStart],
            Self::LiteralMisc => vec![
                TokenCategory::Literal,
                TokenCategory::ValueStart,
                TokenCategory::ValueEnd,
            ],
            Self::Comma => vec![],
            _ => vec![],
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
