use regex::Regex;
use crate::syntax::token::{Flag, OprType, TokenCategory, TokenType, Side, Keyword};

pub enum Pattern<'a> {
    Vartokens(TokenType),
    Value(TokenType, &'a str),
    Token(TokenType),
    Re(TokenType, &'a str)
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
    pub literal: bool
}
pub struct SideDependentTokenEntry<'a> {
    pub value: &'a str,
    pub type_: TokenType,
    pub side: Side,
    pub from: TokenType
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
            literal: false
        }
    }
}

pub fn singular_token_entries() -> Vec<SingularTokenEntry<'static>> {
    vec![
        SingularTokenEntry {
            re: Some(Regex::new(r"\d").unwrap()),
            type_: TokenType::LiteralNumber,
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        SingularTokenEntry {
            re: Some(Regex::new(r"\w").unwrap()),
            type_: TokenType::Variable,
            categories: &[TokenCategory::ValueStart, TokenCategory::ValueEnd],
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
            categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen, TokenCategory::ValueStart],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '[',
            type_: TokenType::OpenSquareParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen, TokenCategory::ValueStart],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '{',
            type_: TokenType::OpenCurlyParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::OpenParen, TokenCategory::ValueStart],
            ..Default::default()
        },
        SingularTokenEntry {
            value: ')',
            type_: TokenType::CloseParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen, TokenCategory::ValueEnd],
            ..Default::default()
        },
        SingularTokenEntry {
            value: ']',
            type_: TokenType::CloseSquareParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen, TokenCategory::ValueEnd],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '}',
            type_: TokenType::CloseCurlyParen,
            categories: &[TokenCategory::Parenthesis, TokenCategory::CloseParen, TokenCategory::ValueEnd],
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
            value: '\"',
            type_: TokenType::Quote,
            categories: &[TokenCategory::LiteralStringStart, TokenCategory::LiteralStringEnd],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '!',
            type_: TokenType::UnaryOpr(OprType::Not, Side::Left),
            categories: &[TokenCategory::Operator, TokenCategory::ValueStart],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '~',
            type_: TokenType::NormalOpr(OprType::Concat),
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '@',
            type_: TokenType::NormalOpr(OprType::TypeCast),
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
            type_: TokenType::UnaryOpr(OprType::Ref, Side::Left),
            categories: &[TokenCategory::Operator, TokenCategory::ValueStart],
            ..Default::default()
        },
        SingularTokenEntry {
            value: '\\',
            type_: TokenType::UnaryOpr(OprType::Deref, Side::Left),
            categories: &[TokenCategory::Operator, TokenCategory::ValueStart],
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
            type_: TokenType::LiteralNumber,
            combination: &[
                Pattern::Token(TokenType::LiteralNumber),
                Pattern::Token(TokenType::LiteralNumber)
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            type_: TokenType::LiteralNumber,
            combination: &[
                Pattern::Re(TokenType::LiteralNumber, r"^[^\.]*$"),
                Pattern::Token(TokenType::DotOpr),
                Pattern::Token(TokenType::LiteralNumber)
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            type_: TokenType::Variable,
            combination: &[
                Pattern::Token(TokenType::Variable),
                Pattern::Token(TokenType::LiteralNumber)
            ],
            categories: &[TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            type_: TokenType::Variable,
            combination: &[
                Pattern::Token(TokenType::Variable),
                Pattern::Token(TokenType::Variable)
            ],
            categories: &[TokenCategory::ValueStart, TokenCategory::ValueEnd],
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
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        },
        CompoundTokenEntry{
            value: ":=",
            type_: TokenType::DeclarationOpr,
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
            value: "~=",
            type_: TokenType::AssignmentOpr(OprType::Concat),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Concat)),
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
                Pattern::Token(TokenType::UnaryOpr(OprType::Not, Side::Left)),
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
            value: "&&",
            type_: TokenType::NormalOpr(OprType::And),
            combination: &[
                Pattern::Token(TokenType::UnaryOpr(OprType::Ref, Side::Left)),
                Pattern::Token(TokenType::UnaryOpr(OprType::Ref, Side::Left))
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
            type_: TokenType::UnaryOpr(OprType::Increment, Side::Right),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Plus)),
                Pattern::Token(TokenType::NormalOpr(OprType::Plus))
            ],
            categories: &[TokenCategory::Operator, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "--",
            type_: TokenType::UnaryOpr(OprType::Decrement, Side::Right),
            combination: &[
                Pattern::Token(TokenType::NormalOpr(OprType::Minus)),
                Pattern::Token(TokenType::NormalOpr(OprType::Minus))
            ],
            categories: &[TokenCategory::Operator, TokenCategory::ValueEnd],
            ..Default::default()
        },
    ]
}

pub fn compound_token_entries_2() -> Vec<CompoundTokenEntry<'static>> {
    vec![
        CompoundTokenEntry{
            type_: TokenType::Comment,
            combination: &[
                Pattern::Token(TokenType::CommentStart),
                Pattern::Vartokens(TokenType::Null),
                Pattern::Value(TokenType::Whitespace, "\n")
            ],
            pair: Some(TokenType::CommentStart),
            literal: true,
            ..Default::default()
        },
        CompoundTokenEntry{
            type_: TokenType::Comment,
            combination: &[
                Pattern::Token(TokenType::MultilineCommentStart),
                Pattern::Vartokens(TokenType::Null),
                Pattern::Token(TokenType::MultilineCommentEnd)
            ],
            pair: Some(TokenType::MultilineCommentStart),
            literal: true,
            ..Default::default()
        },
        CompoundTokenEntry{
            type_: TokenType::LiteralString,
            combination: &[
                Pattern::Token(TokenType::Quote),
                Pattern::Vartokens(TokenType::Null),
                Pattern::Token(TokenType::Quote)
            ],
            pair: Some(TokenType::Quote),
            literal: true,
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "if",
            type_: TokenType::Keyword(Keyword::If),
            combination: &[
                Pattern::Value(TokenType::Variable, "if"),
            ],
            categories: &[TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "else",
            type_: TokenType::Keyword(Keyword::Else),
            combination: &[
                Pattern::Value(TokenType::Variable, "else"),
            ],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "elif",
            type_: TokenType::Keyword(Keyword::Elif),
            combination: &[
                Pattern::Value(TokenType::Variable, "elif"),
            ],
            ..Default::default()
        },

        CompoundTokenEntry{
            value: "do",
            type_: TokenType::Keyword(Keyword::Do),
            combination: &[
                Pattern::Value(TokenType::Variable, "do"),
            ],
            ..Default::default()
        },

        CompoundTokenEntry{
            value: "while",
            type_: TokenType::Keyword(Keyword::While),
            combination: &[
                Pattern::Value(TokenType::Variable, "while"),
            ],
            ..Default::default()
        },

        CompoundTokenEntry{
            value: "for",
            type_: TokenType::Keyword(Keyword::For),
            combination: &[
                Pattern::Value(TokenType::Variable, "for"),
            ],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "del",
            type_: TokenType::Keyword(Keyword::Delete),
            combination: &[
                Pattern::Value(TokenType::Variable, "del"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "ret",
            type_: TokenType::Keyword(Keyword::Return),
            combination: &[
                Pattern::Value(TokenType::Variable, "ret"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "proc",
            type_: TokenType::Keyword(Keyword::Proc),
            combination: &[
                Pattern::Value(TokenType::Variable, "proc"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "fn",
            type_: TokenType::Keyword(Keyword::Fn),
            combination: &[
                Pattern::Value(TokenType::Variable, "fn"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "!is",
            type_: TokenType::NormalOpr(OprType::Isnt),
            combination: &[
                Pattern::Token(TokenType::UnaryOpr(OprType::Not, Side::Left)),
                Pattern::Value(TokenType::Variable, "is"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "is",
            type_: TokenType::NormalOpr(OprType::Is),
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
            categories: &[TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "pub",
            type_: TokenType::Flag(Flag::Pub),
            combination: &[
                Pattern::Value(TokenType::Variable, "pub"),
            ],
            categories: &[TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "priv",
            type_: TokenType::Flag(Flag::Priv),
            combination: &[
                Pattern::Value(TokenType::Variable, "priv"),
            ],
            categories: &[TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "const",
            type_: TokenType::Flag(Flag::Const),
            combination: &[
                Pattern::Value(TokenType::Variable, "const"),
            ],
            categories: &[TokenCategory::ValueStart],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "true",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "true"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "false",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "false"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "null",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "null"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "inf",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "inf"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "undef",
            type_: TokenType::LiteralMisc,
            combination: &[
                Pattern::Value(TokenType::Variable, "undef"),
            ],
            categories: &[TokenCategory::Literal, TokenCategory::ValueStart, TokenCategory::ValueEnd],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "!istype",
            type_: TokenType::NormalOpr(OprType::Isnttype),
            combination: &[
                Pattern::Token(TokenType::AssignmentOpr(OprType::Is)),
                Pattern::Value(TokenType::Variable, "type"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
        CompoundTokenEntry{
            value: "istype",
            type_: TokenType::NormalOpr(OprType::Istype),
            combination: &[
                Pattern::Token(TokenType::AssignmentOpr(OprType::Iseq)),
                Pattern::Value(TokenType::Variable, "type"),
            ],
            categories: &[TokenCategory::Operator],
            ..Default::default()
        },
    ]
}

pub fn side_dependent_token_entries() -> Vec<SideDependentTokenEntry<'static>> {
    vec![
        SideDependentTokenEntry {
            value: "-",
            type_: TokenType::UnaryOpr(OprType::MinusSign, Side::Left),
            side: Side::Left,
            from: TokenType::NormalOpr(OprType::Minus)
        },
        SideDependentTokenEntry {
            value: "+",
            type_: TokenType::UnaryOpr(OprType::PlusSign, Side::Left),
            side: Side::Left,
            from: TokenType::NormalOpr(OprType::Plus)
        }
    ]
}