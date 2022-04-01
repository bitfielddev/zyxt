use std::fmt::{Display, Formatter, Result};
use crate::objects::position::Position;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub value: String,
    pub type_: TokenType,
    pub position: Position,
    pub categories: &'static [TokenCategory],
    pub whitespace: String
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
impl Token {
    pub fn get_raw(&self) -> String {
        format!("{}{}", &self.whitespace, &self.value)
    }
}
pub fn get_order(opr: &OprType) -> u8 {
    match *opr {
        OprType::Null => 0,
        OprType::Increment |
        OprType::Decrement |
        OprType::PlusSign |
        OprType::MinusSign |
        OprType::Not |
        OprType::Ref |
        OprType::Deref => 1,
        OprType::TypeCast => 2,
        OprType::Power => 3,
        //OprType::Root |
        //OprType::Logarithm => 4,
        OprType::DotMult => 5,
        OprType::AstMult |
        OprType::FractDiv |
        OprType::FloorfractDiv |
        OprType::CeilfractDiv |
        OprType::RoundfractDiv |
        OprType::Modulo => 6,
        OprType::CrossMult |
        OprType::Div |
        OprType::FloorDiv |
        OprType::CeilDiv |
        OprType::RoundDiv => 7,
        OprType::Plus |
        OprType::Minus |
        OprType::PlusMinus |
        OprType::MinusPlus => 8,
        OprType::Gt |
        OprType::Lt |
        OprType::Gteq |
        OprType::Lteq |
        OprType::Eq |
        OprType::Noteq |
        OprType::Istype |
        OprType::Isnttype |
        OprType::Is |
        OprType::Isnt |
        OprType::Iseq |
        OprType::Isnteq => 10,
        OprType::And => 14,
        OprType::Xor => 15,
        OprType::Or => 16,
        OprType::Concat => 18,
        OprType::Swap => 19
    }
}
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub enum OprType {
    Increment,
    Decrement,
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
    Null
}
impl Display for OprType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Flag {Hoi, Pub, Priv, Prot, Const, Inst}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Keyword {If, Else, Elif, Do, While, For, Delete, Return, Proc, Fn, Pre, Defer, Class, Struct}
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Side { Left, Right }
#[derive(Copy, Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum TokenType {
    CommentStart, // //
    CommentEnd, // \n
    MultilineCommentStart, // /*
    MultilineCommentEnd, // */
    Flag(Flag), // hoi, pub, priv, prot, const
    UnaryOpr(OprType, Side), // \~, ++, ! etc
    AssignmentOpr(OprType), // =, += etc
    NormalOpr(OprType), // +, -, /f, rt, \&, ==, >, is, &&, ||, ^^, .., ><, istype, isnttype etc
    DotOpr, // .
    DeclarationOpr, // :=
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
    Keyword(Keyword), // if, while etc
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
    LiteralStringEnd, // marks the end of a literal string,
    ValueStart,
    ValueEnd
}