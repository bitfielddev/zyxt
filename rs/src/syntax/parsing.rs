use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use crate::lexer::Position;
use crate::Token;

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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Flag {Hoi, Pub, Priv, Prot, Const}

#[derive(Clone, PartialEq)]
pub enum Element {
    Comment {
        position: Position,
        content: String,
    },
    Call {
        position: Position,
        called: Box<Element>,
        args: Vec<Element>,
        //kwargs
    },
    UnaryOpr {
        position: Position,
        type_: OprType,
        operand: Box<Element>
    },
    BinaryOpr {
        position: Position,
        type_: OprType,
        operand1: Box<Element>,
        operand2: Box<Element>
    },
    DeclarationStmt {
        position: Position,
        variable: Box<Element>, // variable
        content: Box<Element>,
        flags: Vec<Flag>,
        type_: Box<Element>, // variable
    },
    Literal {
        position: Position,
        type_: Box<Element>, // variable
        content: String
    },
    Variable {
        position: Position,
        name: String,
        parent: Box<Element>
    },
    NullElement,
    Token(Token)
}
impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Element::NullElement => "NullElement".to_string(),
            Element::Token(token) => format!("Element::{}", token),
            Element::Variable {position, name, parent} =>
                format!("Variable[position={}, name={}, parent={}]", position, name, **parent),
            Element::Literal {position, type_, content} =>
                format!("Literal[position={}, type={}, content={}]", position, **type_, content),
            Element::Comment {position, content} =>
                format!("Comment[position={}, content={}]", position, content),
            Element::Call {position, called, args} =>
                format!("Call[position={}, called={}, args=[{}]]", position, **called, args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(",")),
            Element::UnaryOpr {position, type_, operand} =>
                format!("UnaryOpr[position={}, type={:?}, operand={}]", position, type_, **operand),
            Element::BinaryOpr {position, type_, operand1, operand2} =>
                format!("BinaryOpr[position={}, type={:?}, operand1={}, operand2={}]", position, type_, **operand1, **operand2),
            Element::DeclarationStmt {position, variable, content, flags, type_} => {
                format!("DeclarationStmt[position={}, variable={}, content={}, flags={}, type={}]", position, **variable, **content, flags.iter().map(|arg| format!("{:?}", arg)).collect::<Vec<String>>().join(","), **type_)
            }
        })
    }
}
impl Element {
    pub fn get_pos(&self) -> &Position {
        match self {
            Element::NullElement => panic!("null element"),
            Element::Token(..) => panic!("token"),
            Element::Variable { position, .. } => position,
            Element::Literal { position, .. } => position,
            Element::Comment { position, .. } => position,
            Element::Call { position, .. } => position,
            Element::UnaryOpr { position, .. } => position,
            Element::BinaryOpr { position, .. } => position,
            Element::DeclarationStmt { position, .. } => position
        }
    }
}