use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result};
use enum_as_inner::EnumAsInner;
use crate::lexer::Position;
use crate::syntax::token::{Flag, OprType};
use crate::Token;
use crate::typechecker::{bin_op_return_type, un_op_return_type};

#[derive(Clone, PartialEq, EnumAsInner)]
pub enum Element {
    Comment {
        position: Position,
        content: String,
    },
    Call {
        position: Position,
        called: Box<Element>,
        args: Vec<Element>,
        kwargs: Box<HashMap<String, Element>>,
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
            Element::Call {position, called, args, kwargs} =>
                format!("Call[position={}, called={}, args=[{}], kwargs={{{}}}", position, **called,
                        args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(","),
                        kwargs.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join(",")),
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
            Element::Variable { position, .. } |
            Element::Literal { position, .. } |
            Element::Comment { position, .. } |
            Element::Call { position, .. } |
            Element::UnaryOpr { position, .. } |
            Element::BinaryOpr { position, .. } |
            Element::DeclarationStmt { position, .. } => position
        }
    }
    pub fn get_name(&self) -> String {
        if let Element::Variable {name: type1, ..} = self {return type1.clone()} else {panic!("not variable")}
    }
    pub fn get_type(&self) -> Element {
        match self {
            Element::Literal {type_, ..} => (**type_).clone(),
            _ => Element::Variable {
                position: self.get_pos().clone(),
                name: match self {
                    Element::BinaryOpr {type_, operand1, operand2, position} => {
                        let type1 = operand1.get_type().get_name();
                        let type2 = operand2.get_type().get_name();
                        bin_op_return_type(type_, type1, type2, position)
                    }, // TODO Element::UnaryOpr, Element::Call etc etc etc
                    Element::UnaryOpr {type_, operand, position} => {
                        let opnd_type = operand.get_type().get_name();
                        un_op_return_type(type_, opnd_type, position)
                    },
                    Element::Call {..} => "#null".to_string(),
                    _ => "".to_string()
                },
                parent: Box::new(Element::NullElement)
            }
        }
    }
 }