use std::fmt::{Display, Formatter, Result};
use crate::lexer::StateTracker;
use crate::syntax::lexing::TokenType::LiteralNumber;
use crate::syntax::parsing::{Flag, OprType};

#[derive(Clone, PartialEq)]
pub struct Token {
    pub value: String,
    pub type_: TokenType,
    pub line: u32,
    pub column: u32,
    pub categories: &'static [TokenCategory]
}
impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Token[value=\"{}\", type={:?}, line={}, column={}, categories={:?}]",
               self.value, self.type_, self.line, self.column, self.categories)
    }
}
impl Default for Token {
    fn default() -> Self {
        Token {
            value: "".to_string(),
            type_: TokenType::Null,
            line: 0,
            column: 0,
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
    Comment,
    Variable,
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

pub struct Greedy(TokenType);
pub struct SingularTokenEntry<'a> {
        value: &'a str,
        type_: TokenType,
        categories: &'a [TokenCategory]
}
pub struct CompoundTokenEntry<'a> {
        value: &'a str,
        type_: TokenType,
        combination: &'a [TokenType],
        categories: &'a [TokenCategory]
}
impl Default for SingularTokenEntry<'static> {
    fn default() -> SingularTokenEntry<'static> {
        SingularTokenEntry {
            value: "",
            type_: TokenType::Null,
            categories: &[]
        }
    }
}


const SINGULAR_TOKEN_ENTRIES: Vec<SingularTokenEntry<'static>> = vec![
    SingularTokenEntry{
        value: r"\w",
        type_: TokenType::Variable,
        categories: &[]
    },
    SingularTokenEntry{
        value: r"\d",
        type_: TokenType::LiteralNumber,
        categories: &[]
    },
    SingularTokenEntry{
        value: r"\(",
        type_: TokenType::OpenParen,
        categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
    },
    SingularTokenEntry{
        value: r"\[",
        type_: TokenType::OpenParen,
        categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
    },
    SingularTokenEntry{
        value: r"{",
        type_: TokenType::OpenParen,
        categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen],
    },
    SingularTokenEntry{
        value: r"\)",
        type_: TokenType::CloseParen,
        categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen]
    },
    SingularTokenEntry{
        value: r"\]",
        type_: TokenType::CloseParen,
        categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen]
    },
    SingularTokenEntry{
        value: r"}",
        type_: TokenType::CloseParen,
        categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen]
    },
    SingularTokenEntry{
        value: r"\.",
        type_: TokenType::DotOpr,
        categories: &[TokenCategory::Operator],
    },
    SingularTokenEntry{
        value: r";",
        type_: TokenType::StatementEnd,
        categories: &[]
    },
    SingularTokenEntry{
        value: r",",
        type_: TokenType::Comma,
        ..Default::default()
    },
    SingularTokenEntry{
        value: r":",
        type_: TokenType::Colon,
        ..Default::default()
    },
    SingularTokenEntry{
        value: r"!",
        type_: TokenType::UnaryOpr(OprType::Not, UnarySide::Left),
        categories: &[TokenCategory::Operator],
    },
    SingularTokenEntry{
        value: r"\+",
        type_: TokenType::NormalOpr(OprType::Plus),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"-",
        type_: TokenType::NormalOpr(OprType::Minus),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"±",
        type_: TokenType::NormalOpr(OprType::PlusMinus),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"∓",
        type_: TokenType::NormalOpr(OprType::MinusPlus),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"·",
        type_: TokenType::NormalOpr(OprType::DotMult),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"\*",
        type_: TokenType::NormalOpr(OprType::AstMult),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"×",
        type_: TokenType::NormalOpr(OprType::CrossMult),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"/",
        type_: TokenType::NormalOpr(OprType::FractDiv),
        categories: &[TokenCategory::Operator],
    },
    SingularTokenEntry{
        value: r"÷",
        type_: TokenType::NormalOpr(OprType::Div),
        categories: &[TokenCategory::Operator],
    },
    SingularTokenEntry{
        value: r"^",
        type_: TokenType::NormalOpr(OprType::Power),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"%",
        type_: TokenType::NormalOpr(OprType::Modulo),
        categories: &[TokenCategory::Operator],
    },
    SingularTokenEntry{
        value: r"=",
        type_: TokenType::AssignmentOpr(OprType::Null),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r">",
        type_: TokenType::NormalOpr(OprType::Gt),
        categories: &[TokenCategory::Operator]
    },
    SingularTokenEntry{
        value: r"<",
        type_: TokenType::NormalOpr(OprType::Lt),
        categories: &[TokenCategory::Operator]
    },
];

const COMPOUND_TOKEN_ENTRIES: Vec<CompoundTokenEntry<'static>> = vec![

];