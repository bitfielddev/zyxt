use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

use smol_str::SmolStr;

use crate::{
    instructor::Process,
    types::{
        errors::ZyxtError,
        interpreter_data::InterpreterData,
        position::Position,
        printer::Print,
        token::{Flag, OprType, Token},
        typeobj::{unit_t::UNIT_T, Type},
        value::Value,
    },
};

#[derive(Clone, PartialEq, Debug)]
pub struct Condition {
    pub condition: Element,
    pub if_true: Vec<Element>,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Argument {
    pub name: SmolStr,
    pub type_: Element,
    pub default: Option<Element>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Element {
    Comment {
        position: Position,
        raw: String,
        content: SmolStr,
    },
    Call {
        position: Position,
        raw: String,
        called: Box<Element>,
        args: Vec<Element>,
        kwargs: HashMap<SmolStr, Element>,
    },
    UnaryOpr {
        position: Position,
        raw: String,
        type_: OprType,
        operand: Box<Element>,
    },
    BinaryOpr {
        position: Position,
        raw: String,
        type_: OprType,
        operand1: Box<Element>,
        operand2: Box<Element>,
    },
    Declare {
        position: Position,
        raw: String,
        variable: Box<Element>, // variable
        content: Box<Element>,
        flags: Vec<Flag>,
        type_: Box<Element>, // variable
    },
    Set {
        position: Position,
        raw: String,
        variable: Box<Element>, // variable
        content: Box<Element>,
    },
    Literal {
        position: Position,
        raw: String,
        content: Value,
    },
    Ident {
        position: Position,
        raw: String,
        name: SmolStr,
        parent: Box<Element>,
    },
    If {
        position: Position,
        raw: String,
        conditions: Vec<Condition>,
    },
    Block {
        position: Position,
        raw: String,
        content: Vec<Element>,
    },
    Delete {
        position: Position,
        raw: String,
        names: Vec<SmolStr>,
    },
    Return {
        position: Position,
        raw: String,
        value: Box<Element>,
    },
    Procedure {
        position: Position,
        raw: String,
        is_fn: bool,
        args: Vec<Argument>,
        return_type: Box<Element>,
        content: Vec<Element>,
    },
    Preprocess {
        position: Position,
        raw: String,
        content: Vec<Element>,
    },
    Defer {
        position: Position,
        raw: String,
        content: Vec<Element>,
    },
    Class {
        position: Position,
        raw: String,
        is_struct: bool,
        implementations: HashMap<SmolStr, Element>,
        inst_fields: HashMap<SmolStr, (Element, Option<Box<Element>>)>,
        content: Vec<Element>,
        args: Option<Vec<Argument>>,
    },
    NullElement,
    Token(Token),
}
impl Display for Argument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.name,
            if self.type_.get_name() != "_any" {
                format!(": {}", self.type_.get_name())
            } else {
                "".to_string()
            },
            if let Some(r) = &self.default {
                format!(": {}", r.get_raw().trim())
            } else {
                "".to_string()
            }
        )
    }
}
pub trait VecElementRaw {
    fn get_raw(&self) -> String;
}
impl VecElementRaw for Vec<Element> {
    fn get_raw(&self) -> String {
        self.iter()
            .map(|e| e.get_raw())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Element {
    pub fn get_pos(&self) -> &Position {
        match self {
            Element::NullElement => panic!("null element"),
            Element::Token(Token { position, .. })
            | Element::Ident { position, .. }
            | Element::Literal { position, .. }
            | Element::Comment { position, .. }
            | Element::Call { position, .. }
            | Element::UnaryOpr { position, .. }
            | Element::BinaryOpr { position, .. }
            | Element::Declare { position, .. }
            | Element::Set { position, .. }
            | Element::If { position, .. }
            | Element::Block { position, .. }
            | Element::Delete { position, .. }
            | Element::Return { position, .. }
            | Element::Procedure { position, .. }
            | Element::Preprocess { position, .. }
            | Element::Defer { position, .. }
            | Element::Class { position, .. } => position,
        }
    }
    pub fn get_raw(&self) -> String {
        match self {
            Element::NullElement => "".to_string(),
            Element::Token(t) => t.get_raw(),
            Element::Ident { raw, .. }
            | Element::Literal { raw, .. }
            | Element::Comment { raw, .. }
            | Element::Call { raw, .. }
            | Element::UnaryOpr { raw, .. }
            | Element::BinaryOpr { raw, .. }
            | Element::Declare { raw, .. }
            | Element::Set { raw, .. }
            | Element::If { raw, .. }
            | Element::Block { raw, .. }
            | Element::Delete { raw, .. }
            | Element::Return { raw, .. }
            | Element::Procedure { raw, .. }
            | Element::Preprocess { raw, .. }
            | Element::Defer { raw, .. }
            | Element::Class { raw, .. } => raw.to_owned(),
        }
    }
    pub fn get_raw_mut(&mut self) -> Option<&mut String> {
        match self {
            Element::NullElement | Element::Token(_) => None,
            Element::Ident { raw, .. }
            | Element::Literal { raw, .. }
            | Element::Comment { raw, .. }
            | Element::Call { raw, .. }
            | Element::UnaryOpr { raw, .. }
            | Element::BinaryOpr { raw, .. }
            | Element::Declare { raw, .. }
            | Element::Set { raw, .. }
            | Element::If { raw, .. }
            | Element::Block { raw, .. }
            | Element::Delete { raw, .. }
            | Element::Return { raw, .. }
            | Element::Procedure { raw, .. }
            | Element::Preprocess { raw, .. }
            | Element::Defer { raw, .. }
            | Element::Class { raw, .. } => Some(raw),
        }
    }
    pub fn get_name(&self) -> SmolStr {
        if let Element::Ident { name: type1, .. } = self {
            type1.to_owned()
        } else {
            panic!("not variable")
        }
    }
    pub fn block_type<O: Print>(
        content: &mut [Element],
        typelist: &mut InterpreterData<Type<Element>, O>,
        add_set: bool,
    ) -> Result<(Type<Element>, Option<Type<Element>>), ZyxtError> {
        let mut last = UNIT_T.as_type_element();
        let mut return_type = None;
        if add_set {
            typelist.add_frame(None);
        }
        for ele in content.iter_mut() {
            last = ele.process(typelist)?;
            if let Type::Return(value) = last.to_owned() {
                if return_type.to_owned().is_none() {
                    return_type = Some(*value);
                } else if last != return_type.to_owned().unwrap() {
                    return Err(ZyxtError::error_4_t(last, return_type.unwrap())
                        .with_pos_and_raw(ele.get_pos(), &ele.get_raw()));
                }
            }
        }
        if let Some(return_type) = return_type.to_owned() {
            if last != return_type {
                let last_ele = content.last().unwrap();
                return Err(ZyxtError::error_4_t(last, return_type)
                    .with_pos_and_raw(last_ele.get_pos(), &last_ele.get_raw()));
            }
        }
        if add_set {
            typelist.pop_frame();
        }
        Ok((last, if add_set { None } else { return_type }))
    }
    pub fn is_pattern(&self) -> bool {
        matches!(self, Element::Ident { .. })
    }
}
