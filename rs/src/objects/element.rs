use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::objects::position::Position;
use crate::objects::token::{Flag, OprType};
use crate::Token;
use crate::errors::ZyxtError;
use crate::objects::variable::Variable;
use crate::objects::typeobj::TypeObj;
use crate::objects::varstack::Varstack;

#[derive(Clone, PartialEq, Debug)]
pub struct Condition {
    pub condition: Element,
    pub if_true: Vec<Element>
}
#[derive(Clone, PartialEq, Debug)]
pub struct Argument {
    pub name: String,
    pub type_: TypeObj,
    pub default: Option<Element>
}

#[derive(Clone, PartialEq, Debug)]
pub enum Element {
    Comment {
        position: Position,
        raw: String,
        content: String,
    },
    Call {
        position: Position,
        raw: String,
        called: Box<Element>,
        args: Vec<Element>,
        kwargs: Box<HashMap<String, Element>>,
    },
    UnaryOpr {
        position: Position,
        raw: String,
        type_: OprType,
        operand: Box<Element>
    },
    BinaryOpr {
        position: Position,
        raw: String,
        type_: OprType,
        operand1: Box<Element>,
        operand2: Box<Element>
    },
    Declare {
        position: Position,
        raw: String,
        variable: Box<Element>, // variable
        content: Box<Element>,
        flags: Vec<Flag>,
        type_: TypeObj, // variable
    },
    Set {
        position: Position,
        raw: String,
        variable: Box<Element>, // variable
        content: Box<Element>
    },
    Literal {
        position: Position,
        raw: String,
        type_: TypeObj,
        content: String
    },
    Variable {
        position: Position,
        raw: String,
        name: String,
        parent: Box<Element>
    },
    If {
        position: Position,
        raw: String,
        conditions: Vec<Condition>
    },
    Block {
        position: Position,
        raw: String,
        content: Vec<Element>
    },
    Delete {
        position: Position,
        raw: String,
        names: Vec<String>,
    },
    Return {
        position: Position,
        raw: String,
        value: Box<Element>
    },
    Procedure {
        position: Position,
        raw: String,
        is_fn: bool,
        args: Vec<Argument>,
        return_type: TypeObj,
        content: Vec<Element>
    },
    NullElement,
    Token(Token)
}
impl Display for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Condition[condition={:#?}, if_true=[{}]]", self.condition,
               self.if_true.iter().map(|ele| format!("{:#?}", ele)).collect::<Vec<String>>().join(","))
    }
}
impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Argument[name={}, type={}, default={}]", self.name, self.type_,
               if self.default == None {"None".to_string()} else {format!("{:#?}", self.default.clone().unwrap())})
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
            Element::Block { position, .. } |
            Element::Delete { position, .. } |
            Element::Return { position, .. } |
            Element::Procedure { position, .. } => position
        }
    }
    pub fn get_raw(&self) -> &String {
        match self {
            Element::NullElement => &"".to_string(),
            Element::Token(t) => &t.get_raw(),
            Element::Variable { raw, .. } |
            Element::Literal { raw, .. } |
            Element::Comment { raw, .. } |
            Element::Call { raw, .. } |
            Element::UnaryOpr { raw, .. } |
            Element::BinaryOpr { raw, .. } |
            Element::Declare { raw, .. } |
            Element::Set { raw, .. } |
            Element::If { raw, .. } |
            Element::Block { raw, .. } |
            Element::Delete { raw, .. } |
            Element::Return { raw, .. } |
            Element::Procedure { raw, .. } => raw
        }
    }
    pub fn get_name(&self) -> String {
        if let Element::Variable {name: type1, ..} = self {return type1.clone()} else {panic!("not variable")}
    }
    pub fn bin_op_return_type(type_: &OprType, type1: TypeObj, type2: TypeObj,
                              typelist: &mut Varstack<TypeObj>, position: &Position) -> Result<TypeObj, ZyxtError> {
        if type_ == &OprType::TypeCast {
            return Ok(type2)
        }
        if let Some(v) = Variable::default(type1.clone(), typelist)?
            .bin_opr(type_, Variable::default(type2.clone(), typelist)?) {
            return Ok(v.get_type_obj())
        } else {
            Err(ZyxtError::from_pos(position).error_4_0_0(type_.to_string(), type1.to_string(), type2.to_string()))
        }
    }
    pub fn un_op_return_type(type_: &OprType, opnd_type: TypeObj,
                             typelist: &mut Varstack<TypeObj>, position: &Position) -> Result<TypeObj, ZyxtError> {
        if let Some(v) = Variable::default(opnd_type.clone(), typelist)?.un_opr(type_) {
            return Ok(v.get_type_obj())
        } else{
            Err(ZyxtError::from_pos(position).error_4_0_1(type_.to_string(), opnd_type.to_string()))
        }
    }
    pub fn get_block_type(content: &mut Vec<Element>, typelist: &mut Varstack<TypeObj>, add_set: bool) -> Result<TypeObj, ZyxtError> {
        let mut last = TypeObj::null();
        if add_set {typelist.add_set();}
        for ele in content.iter_mut() {
            last = ele.get_type(typelist)?;
        }
        if add_set {typelist.pop_set();}
        Ok(last)
    }
    pub fn get_type(&mut self, typelist: &mut Varstack<TypeObj>) -> Result<TypeObj, ZyxtError> {
        match self {
            Element::Literal {type_, ..} => Ok(type_.clone()),
            Element::Variable {name, position, ..} =>
                typelist.get_val(name, position),
            Element::Block {content, ..} => Element::get_block_type(content, typelist, true),
            Element::Call {called, args, ..} => {
                // TODO arg type checking
                for arg in args.iter_mut() {arg.get_type(typelist)?;}
                match *called.clone() {
                    Element::Procedure {return_type, ..} => Ok(return_type),
                    _ => Ok(TypeObj::null()) // TODO call return
                }
            }
            Element::Declare {position, variable, content,
                flags, type_, raw} => {
                let content_type = content.get_type(typelist)?;
                if *type_ == TypeObj::null() {
                    typelist.declare_val(&variable.get_name(), &content_type);
                    *self = Element::Declare {
                        type_: content_type.clone(),
                        content: content.clone(),
                        variable: variable.clone(),
                        position: position.clone(),
                        raw: raw.clone(),
                        flags: flags.clone()
                    };
                } else {
                    typelist.declare_val(&variable.get_name(), &type_);
                    if content_type != *type_ {
                        let new_content = Element::BinaryOpr {
                            position: position.clone(),
                            raw: raw.clone(),
                            type_: OprType::TypeCast,
                            operand1: content.clone(),
                            operand2: Box::new(type_.as_element())
                        };
                        *self = Element::Declare {
                            type_: type_.clone(),
                            content: Box::new(new_content),
                            variable: variable.clone(),
                            position: position.clone(),
                            raw: raw.clone(),
                            flags: flags.clone()
                        };
                    }
                };
                Ok(content_type)
            },
            Element::If {conditions, ..} => Element::get_block_type(&mut conditions[0].if_true, typelist, true), // TODO consider all returns
            Element::BinaryOpr {type_, operand1, operand2, position, ..} => {
                let type1 = operand1.get_type(typelist)?;
                let type2 = operand2.get_type(typelist)?;
                Element::bin_op_return_type(type_, type1, type2, typelist, position)
            },
            Element::UnaryOpr {type_, operand, position, ..} => {
                let opnd_type = operand.get_type(typelist)?;
                Element::un_op_return_type(type_, opnd_type, typelist, position)
            },
            Element::Procedure {is_fn, return_type, content, args, ..} => {
                typelist.add_set();
                for arg in args {
                    typelist.declare_val(&arg.name, &arg.type_);
                }
                let res =  Element::get_block_type(content, typelist, false)?;
                if return_type == &TypeObj::null() {*return_type = res;}
                Ok(TypeObj::Prim {
                    name: if *is_fn {"fn"} else {"proc"}.to_string(),
                    type_args: vec![TypeObj::null(), return_type.clone()]
                })
            }, // TODO angle bracket thingy when it is implemented
            _ => Ok(TypeObj::null())
        }
    }
}