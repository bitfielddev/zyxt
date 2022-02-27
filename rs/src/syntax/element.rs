use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result, write};
use crate::lexer::Position;
use crate::syntax::token::{Flag, OprType};
use crate::{errors, Token};
use crate::interpreter::Variable;

#[derive(Clone, PartialEq)]
pub struct Condition {
    condition: Element,
    if_true: Vec<Element>
}

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
    Declare {
        position: Position,
        variable: Box<Element>, // variable
        content: Box<Element>,
        flags: Vec<Flag>,
        type_: Box<Element>, // variable
    },
    Set {
        position: Position,
        variable: Box<Element>, // variable
        content: Box<Element>
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
    If {
        position: Position,
        conditions: Vec<Condition>
    },
    Block {
        position: Position,
        content: Vec<Element>
    },
    NullElement,
    Token(Token)
}
impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Condition[condition={}, if_true=[{}]]", self.condition,
               self.if_true.iter().map(|ele| ele.to_string()).collect::<Vec<String>>().join(","))
    }
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
                format!("Call[position={}, called={}, args=[{}], kwargs={{{}}}]", position, **called,
                        args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(","),
                        kwargs.iter().map(|(k, v)| format!("{}: {}", k, v)).collect::<Vec<String>>().join(",")),
            Element::UnaryOpr {position, type_, operand} =>
                format!("UnaryOpr[position={}, type={:?}, operand={}]", position, type_, **operand),
            Element::BinaryOpr {position, type_, operand1, operand2} =>
                format!("BinaryOpr[position={}, type={:?}, operand1={}, operand2={}]", position, type_, **operand1, **operand2),
            Element::Declare {position, variable, content, flags, type_} => {
                format!("Declare[position={}, variable={}, content={}, flags={}, type={}]", position, **variable, **content,
                        flags.iter().map(|arg| format!("{:?}", arg)).collect::<Vec<String>>().join(","), **type_)
            },
            Element::Set {position, variable, content} => {
                format!("Set[position={}, variable={}, content={}]", position, **variable, **content)
            },
            Element::If {position, conditions} => {
                format!("If[position={}, conditions=[{}]]", position,
                        conditions.iter().map(|cond| cond.to_string()).collect::<Vec<String>>().join(","))
            },
            Element::Block {position, content} => {
                format!("If[position={}, conditions=[{}]]", position,
                        content.iter().map(|ele| ele.to_string()).collect::<Vec<String>>().join(","))
            }
        })
    }
}
impl Element {
    pub fn get_pos(&self) -> &Position {
        match self {
            Element::NullElement => panic!("null element"),
            Element::Token(Token{ position, .. }) |
            Element::Variable { position, .. } |
            Element::Literal { position, .. } |
            Element::Comment { position, .. } |
            Element::Call { position, .. } |
            Element::UnaryOpr { position, .. } |
            Element::BinaryOpr { position, .. } |
            Element::Declare { position, .. } |
            Element::Set { position, .. } |
            Element::If { position, .. } |
            Element::Block { position, .. } => position
        }
    }
    pub fn get_name(&self) -> String {
        if let Element::Variable {name: type1, ..} = self {return type1.clone()} else {panic!("not variable")}
    }
    pub fn bin_op_return_type(type_: &OprType, type1: String, type2: String, position: &Position) -> String {
        if type_ == &OprType::TypeCast {
            return type2
        }
        if let Some(v) = Variable::default(type1.clone())
            .bin_opr(type_, Variable::default(type2.clone())) {
            return v.get_type_name()
        } else {
            errors::error_pos(position);
            errors::error_4_0_0(type_.to_string(), type1, type2)
        }
    }
    pub fn un_op_return_type(type_: &OprType, opnd_type: String, position: &Position) -> String {
        if let Some(v) = Variable::default(opnd_type.clone()).un_opr(type_) {
            return v.get_type_name()
        } else{
            errors::error_pos(position);
            errors::error_4_0_1(type_.to_string(), opnd_type)
        }
    }
    pub fn get_type(&self, typelist: &HashMap<String, Element>) -> Element {
        match self {
            Element::Literal {type_, ..} => (**type_).clone(),
            Element::Variable {name, position, ..} => typelist.get(name).unwrap_or_else(|| {
                errors::error_pos(position);
                errors::error_3_0(name.clone())
            }).clone(),
            _ => Element::Variable {
                position: self.get_pos().clone(),
                name: match self {
                    Element::BinaryOpr {type_, operand1, operand2, position} => {
                        let type1 = operand1.get_type(typelist).get_name();
                        let type2 = operand2.get_type(typelist).get_name();
                        Element::bin_op_return_type(type_, type1, type2, position)
                    },
                    Element::UnaryOpr {type_, operand, position} => {
                        let opnd_type = operand.get_type(typelist).get_name();
                        Element::un_op_return_type(type_, opnd_type, position)
                    },
                    Element::Call {..} => "#null".to_string(),
                    _ => "".to_string()
                },
                parent: Box::new(Element::NullElement)
            }
        }
    }
 }