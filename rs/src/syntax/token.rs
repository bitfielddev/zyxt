use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use crate::lexer::Position;

#[derive(Clone, PartialEq)]
pub struct Token {
    pub value: String,
    pub type_: TokenType,
    pub position: Position,
    pub categories: &'static [TokenCategory],
    pub whitespace: String
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
            categories: &[],
            whitespace: "".to_string()
        }
    }
}

pub fn get_order(opr: &OprType) -> u8 {
    let map = HashMap::from([
        (OprType::Null, 0u8),
        (OprType::Increment, 2u8),
        (OprType::Decrement, 2u8),
        (OprType::PlusSign, 2u8),
        (OprType::MinusSign, 2u8),
        (OprType::Not, 2u8),
        (OprType::BitComplement, 2u8),
        (OprType::Logarithm, 4u8),
        (OprType::Root, 4u8),
        (OprType::Power, 3u8),
        (OprType::DotMult, 5u8),
        (OprType::AstMult, 6u8),
        (OprType::CrossMult, 7u8),
        (OprType::Div, 7u8),
        (OprType::FloorDiv, 7u8),
        (OprType::CeilDiv, 7u8),
        (OprType::RoundDiv, 7u8),
        (OprType::FractDiv, 6u8),
        (OprType::FloorfractDiv, 6u8),
        (OprType::CeilfractDiv, 6u8),
        (OprType::RoundfractDiv, 6u8),
        (OprType::Modulo, 6u8),
        (OprType::Plus, 8u8),
        (OprType::Minus, 8u8),
        (OprType::PlusMinus, 8u8),
        (OprType::MinusPlus, 8u8),
        (OprType::BitLshift, 9u8),
        (OprType::BitRshift, 9u8),
        (OprType::Bit0Rshift, 9u8),
        (OprType::And, 14u8),
        (OprType::Or, 16u8),
        (OprType::Xor, 15u8),
        (OprType::Gt, 10u8),
        (OprType::Lt, 10u8),
        (OprType::Gteq, 10u8),
        (OprType::Lteq, 10u8),
        (OprType::Eq, 10u8),
        (OprType::Noteq, 10u8),
        (OprType::Istype, 10u8),
        (OprType::Isnttype, 10u8),
        (OprType::Is, 10u8),
        (OprType::Isnt, 10u8),
        (OprType::Iseq, 10u8),
        (OprType::Isnteq, 10u8),
        (OprType::BitAnd, 11u8),
        (OprType::BitOr, 13u8),
        (OprType::BitXor, 12u8),
        (OprType::Concat, 17u8),
        (OprType::Swap, 19u8),
    ]);
    *map.get(opr).unwrap()
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum OprType {
    Increment,
    Decrement,
    PlusSign,
    MinusSign,
    Not,
    BitComplement,
    Logarithm,
    Root,
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
    BitLshift,
    BitRshift,
    Bit0Rshift,
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
    BitAnd,
    BitOr,
    BitXor,
    Concat,
    Swap,
    Ref,
    Deref,
    Null
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flag {Hoi, Pub, Priv, Prot, Const}
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