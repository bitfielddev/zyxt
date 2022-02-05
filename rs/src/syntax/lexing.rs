use std::fmt::{Display, Formatter, Result};
use regex::Regex;
use crate::lexer::Position;
use crate::syntax::parsing::{Flag, OprType};

#[derive(Clone, PartialEq)]
pub struct Token {
    pub value: String,
    pub type_: TokenType,
    pub position: Position,
    pub categories: &'static [TokenCategory]
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Token[value=\"{}\", type={:?}, position={}, categories={:?}]",
               self.value, self.type_, self.position, self.categories)
    }
}
impl Default for Token {
    fn default() -> Self {
        Token {
            value: "".to_string(),
            type_: TokenType::Null,
            position: Position{..Default::default()},
            categories: &[]
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum UnarySide { Left, Right }
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    CommentStart, // //
    CommentEnd, // \n
    MultilineCommentStart, // /*
    MultilineCommentEnd, // */
    Flag(Flag), // hoi, pub, priv, prot, const
    UnaryOpr(OprType, UnarySide), // \~, ++, ! etc
    AssignmentOpr(OprType), // =, += etc
    NormalOpr(OprType), // +, -, /f, rt, \&, ==, >, is, &&, ||, ^^, .., ><, istype, isnttype etc
    DotOpr, // .
    DeclarationStmt, // :=
    LiteralMisc, // true, null, etc
    LiteralNumber, // 3, 24, -34.5 etc
    LiteralString, // "abc" etc
    StatementEnd, // ;
    OpenParen, // (
    CloseParen, // )
    OpenSquareParen, // [
    CloseSquareParen, // ]
    OpenCurlyParen, // {
    CloseCurlyParen, // }
    OpenAngleBracket, // <
    CloseAngleBracket, // >
    Comma, // ,
    Colon, // :
    Apostrophe, // '
    Quote, // "
    Bar, // |
    Comment,
    Variable,
    Whitespace,
    Null
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenCategory {
    Operator,
    Literal,
    Parenthesis,
    OpenParen,
    CloseParen,
    LiteralStringStart, //  marks the start of a literal string
    LiteralStringEnd // marks the end of a literal string
}

pub enum Pattern<'a> {
    Vartokens(TokenType),
    Value(TokenType, &'a str),
    Token(TokenType)
}
pub struct SingularTokenEntry<'a> {
    pub value: char,
    pub re: Option<Regex>,
    pub type_: TokenType,
    pub categories: &'a [TokenCategory],
    pub pair: Option<TokenType>
}
pub struct CompoundTokenEntry<'a> {
    pub value: &'a str,
    pub type_: TokenType,
    pub combination: &'a [Pattern<'a>],
    pub categories: &'a [TokenCategory],
    pub pair: Option<TokenType>,
    pub literal_type: Option<TokenType>
}
impl Default for SingularTokenEntry<'static> {
    fn default() -> SingularTokenEntry<'static> {
        SingularTokenEntry {
            value: ' ',
            re: None,
            type_: TokenType::Null,
            categories: &[],
            pair: None
        }
    }
}
impl Default for CompoundTokenEntry<'static> {
    fn default() -> CompoundTokenEntry<'static> {
        CompoundTokenEntry {
            value: "",
            type_: TokenType::Null,
            combination: &[],
            categories: &[],
            pair: None,
            literal_type: None
        }
    }
}

pub fn singular_token_entries() -> Vec<SingularTokenEntry<'static>> {
    vec![
        SingularTokenEntry {
            re: Some(Regex::new(r"\w").unwrap()),
            type_: TokenType::Variable,
            ..Default::default()
        },
        SingularTokenEntry {
            re: Some(Regex::new(r"\d").unwrap()),
            type_: TokenType::LiteralNumber,
            ..Default::default()
        },
        SingularTokenEntry {
            re: Some(Regex::new(r"\s").unwrap()),
            type_: TokenType::Whitespace,
            ..Default::default()
        },
        SingularTokenEntry {
            value: '(',
            type_: TokenType::OpenParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '[',
            type_: TokenType::OpenParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '{',
            type_: TokenType::OpenParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
            ..Default::default()
        },
        SingularTokenEntry {
            value: ')',
            type_: TokenType::CloseParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen],
            ..Default::default()
        },
        SingularTokenEntry {
            value: ']',
            type_: TokenType::CloseParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '}',
            type_: TokenType::CloseParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '.',
            type_: TokenType::DotOpr,
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: ';',
            type_: TokenType::StatementEnd,
            ..Default::default()
        },
        SingularTokenEntry {
            value: ',',
            type_: TokenType::Comma,
            ..Default::default()
        },
        SingularTokenEntry {
            value: ':',
            type_: TokenType::Colon,
            ..Default::default()
        },
        SingularTokenEntry {
            value: '!',
            type_: TokenType::UnaryOpr(OprType::Not, UnarySide::Left),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '~',
            type_: TokenType::NormalOpr(OprType::Concat),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '+',
            type_: TokenType::NormalOpr(OprType::Plus),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '-',
            type_: TokenType::NormalOpr(OprType::Minus),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '±',
            type_: TokenType::NormalOpr(OprType::PlusMinus),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '∓',
            type_: TokenType::NormalOpr(OprType::MinusPlus),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '·',
            type_: TokenType::NormalOpr(OprType::DotMult),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '*',
            type_: TokenType::NormalOpr(OprType::AstMult),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '×',
            type_: TokenType::NormalOpr(OprType::CrossMult),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '/',
            type_: TokenType::NormalOpr(OprType::FractDiv),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '÷',
            type_: TokenType::NormalOpr(OprType::Div),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '^',
            type_: TokenType::NormalOpr(OprType::Power),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '%',
            type_: TokenType::NormalOpr(OprType::Modulo),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '=',
            type_: TokenType::AssignmentOpr(OprType::Null),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '>',
            type_: TokenType::NormalOpr(OprType::Gt),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '<',
            type_: TokenType::NormalOpr(OprType::Lt),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '&',
            type_: TokenType::UnaryOpr(OprType::Ref, UnarySide::Left),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '\\',
            type_: TokenType::UnaryOpr(OprType::Deref, UnarySide::Left),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '|',
            type_: TokenType::Bar,
            ..Default::default()
        },
    ]
}

pub fn compound_token_entries_1() -> Vec<CompoundTokenEntry<'static>> {
    vec![
        CompoundTokenEntry{
            type_: TokenType::Variable,
            combination: &[
                Pattern::Vartokens(TokenType::Variable),
                Pattern::Vartokens(TokenType::LiteralNumber)
            ],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "//",
            type_: TokenType::CommentStart,
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::FractDiv)),
                Pattern::Token(TokenType::NormalOpr(OprType::FractDiv))
            ],
            categories: &[TokenCategory::LiteralStringStart],
            pair: Some(TokenType::CommentEnd),
            literal_type: Some(TokenType::Comment)
        },
        CompoundTokenEntry{
            value: "\n",
            type_: TokenType::CommentEnd,
            combination: &[
                Pattern::Value(TokenType::Whitespace, "\n")
            ],
            categories: &[TokenCategory::LiteralStringEnd],
            pair: Some(TokenType::CommentStart),
            literal_type: Some(TokenType::Comment)
        },
        CompoundTokenEntry{
            value: "/*",
            type_: TokenType::MultilineCommentStart,
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::FractDiv)),
                Pattern::Token(TokenType::NormalOpr(OprType::AstMult))
            ],
            categories: &[TokenCategory::LiteralStringStart],
            pair: Some(TokenType::MultilineCommentEnd),
            literal_type: Some(TokenType::Comment)
        },
        CompoundTokenEntry{
            value: "*/",
            type_: TokenType::MultilineCommentEnd,
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::AstMult)),
                Pattern::Token(TokenType::NormalOpr(OprType::FractDiv))
            ],
            categories: &[TokenCategory::LiteralStringEnd],
            pair: Some(TokenType::MultilineCommentStart),
            literal_type: Some(TokenType::Comment)
        },
        CompoundTokenEntry{
            value: ":=",
            type_: TokenType::DeclarationStmt,
            combination: &[
                Pattern::Token(TokenType::Colon),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "+-",
            type_: TokenType::NormalOpr(OprType::PlusMinus),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Plus)),
                Pattern::Token(TokenType::NormalOpr(OprType::Minus)),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "-+",
            type_: TokenType::NormalOpr(OprType::PlusMinus),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Minus)),
                Pattern::Token(TokenType::NormalOpr(OprType::Plus)),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "/f",
            type_: TokenType::NormalOpr(OprType::FloorfractDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::FractDiv)),
                Pattern::Value(TokenType::Variable, "f")
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "/c",
            type_: TokenType::NormalOpr(OprType::CeilfractDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::FractDiv)),
                Pattern::Value(TokenType::Variable, "c")
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "/~",
            type_: TokenType::NormalOpr(OprType::RoundfractDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::FractDiv)),
                Pattern::Token(TokenType::NormalOpr(OprType::Concat)),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "÷f",
            type_: TokenType::NormalOpr(OprType::FloorDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Div)),
                Pattern::Value(TokenType::Variable, "f")
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "÷c",
            type_: TokenType::NormalOpr(OprType::CeilDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Div)),
                Pattern::Value(TokenType::Variable, "c")
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "÷~",
            type_: TokenType::NormalOpr(OprType::RoundDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Div)),
                Pattern::Token(TokenType::NormalOpr(OprType::Concat)),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "+=",
            type_: TokenType::AssignmentOpr(OprType::Plus),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Plus)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "-=",
            type_: TokenType::AssignmentOpr(OprType::Minus),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Minus)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "*=",
            type_: TokenType::AssignmentOpr(OprType::AstMult),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::AstMult)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "/=",
            type_: TokenType::AssignmentOpr(OprType::FractDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::FractDiv)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "/f=",
            type_: TokenType::AssignmentOpr(OprType::FloorfractDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::FloorfractDiv)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "/c=",
            type_: TokenType::AssignmentOpr(OprType::FloorfractDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::CeilfractDiv)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "/~=",
            type_: TokenType::AssignmentOpr(OprType::FloorfractDiv),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::RoundfractDiv)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "%=",
            type_: TokenType::AssignmentOpr(OprType::Modulo),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Modulo)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "==",
            type_: TokenType::AssignmentOpr(OprType::Eq),
            combination: &[
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "<=",
            type_: TokenType::AssignmentOpr(OprType::Lteq),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Lt)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: ">=",
            type_: TokenType::AssignmentOpr(OprType::Gteq),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Gt)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "!=",
            type_: TokenType::AssignmentOpr(OprType::Noteq),
            combination: &[
                Pattern::Token(TokenType::UnaryOpr(OprType::Not, UnarySide::Left)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "===",
            type_: TokenType::AssignmentOpr(OprType::Iseq),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Eq)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "!==",
            type_: TokenType::AssignmentOpr(OprType::Isnteq),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Noteq)),
                Pattern::Token(TokenType::AssignmentOpr(OprType::Null))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "!is",
            type_: TokenType::AssignmentOpr(OprType::Isnteq),
            combination: &[
                Pattern::Token(TokenType::UnaryOpr(OprType::Not, UnarySide::Left)),
                Pattern::Value(TokenType::Variable, "is"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "is",
            type_: TokenType::AssignmentOpr(OprType::Isnteq),
            combination: &[
                Pattern::Value(TokenType::Variable, "is"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "&&",
            type_: TokenType::NormalOpr(OprType::And),
            combination: &[
                Pattern::Token(TokenType::UnaryOpr(OprType::Ref, UnarySide::Left)),
                Pattern::Token(TokenType::UnaryOpr(OprType::Ref, UnarySide::Left))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "||",
            type_: TokenType::NormalOpr(OprType::And),
            combination: &[
                Pattern::Token(TokenType::Bar),
                Pattern::Token(TokenType::Bar)
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "^^",
            type_: TokenType::NormalOpr(OprType::Xor),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Power)),
                Pattern::Token(TokenType::NormalOpr(OprType::Power))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "!istype",
            type_: TokenType::AssignmentOpr(OprType::Isnttype),
            combination: &[
                Pattern::Token(TokenType::UnaryOpr(OprType::Not, UnarySide::Left)),
                Pattern::Value(TokenType::Variable, "istype"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "istype",
            type_: TokenType::AssignmentOpr(OprType::Istype),
            combination: &[
                Pattern::Value(TokenType::Variable, "istype"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "><",
            type_: TokenType::NormalOpr(OprType::Swap),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Gt)),
                Pattern::Token(TokenType::NormalOpr(OprType::Lt))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "++",
            type_: TokenType::UnaryOpr(OprType::Increment, UnarySide::Right),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Plus)),
                Pattern::Token(TokenType::NormalOpr(OprType::Plus))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "--",
            type_: TokenType::UnaryOpr(OprType::Decrement, UnarySide::Right),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Minus)),
                Pattern::Token(TokenType::NormalOpr(OprType::Minus))
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "is",
            type_: TokenType::AssignmentOpr(OprType::Isnteq),
            combination: &[
                Pattern::Value(TokenType::Variable, "is"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "hoi",
            type_: TokenType::Flag(Flag::Hoi),
            combination: &[
                Pattern::Value(TokenType::Variable, "hoi"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "pub",
            type_: TokenType::Flag(Flag::Hoi),
            combination: &[
                Pattern::Value(TokenType::Variable, "pub"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "priv",
            type_: TokenType::Flag(Flag::Hoi),
            combination: &[
                Pattern::Value(TokenType::Variable, "priv"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "const",
            type_: TokenType::Flag(Flag::Hoi),
            combination: &[
                Pattern::Value(TokenType::Variable, "const"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "true",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "true"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "false",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "false"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "null",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "null"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "inf",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "inf"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "undef",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "undef"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
    ]
}