mod binary_opr;
mod block;
mod call;
mod class;
mod comment;
mod declare;
mod defer;
mod delete;
mod ident;
mod r#if;
mod literal;
mod preprocess;
mod procedure;
mod r#return;
mod set;
mod unary_opr;

use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use enum_as_inner::EnumAsInner;
use smol_str::SmolStr;

use crate::{
    instructor::Process,
    types::{
        element::{
            binary_opr::BinaryOpr, block::Block, call::Call, class::Class, comment::Comment,
            declare::Declare, defer::Defer, delete::Delete, ident::Ident, literal::Literal,
            preprocess::Preprocess, procedure::Procedure, r#if::If, r#return::Return, set::Set,
            unary_opr::UnaryOpr,
        },
        errors::ZyxtError,
        interpreter_data::{FrameType, InterpreterData},
        position::Position,
        printer::Print,
        token::{Flag, OprType, Token},
        typeobj::{unit_t::UNIT_T, Type},
        value::Value,
    },
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PosRaw {
    pub position: Position,
    pub raw: SmolStr,
}

pub trait ElementData: Clone + PartialEq + Eq + Debug {
    fn as_variant(&self) -> ElementVariants;
    fn is_pattern(&self) -> bool {
        false
    }
    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(Type::Any)
    }
    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        _out: &mut impl Print,
    ) -> Result<ElementVariants, ZyxtError> {
        Ok(self.as_variant())
    }
    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        unreachable!()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Element<V: ElementData = ElementVariants> {
    pub pos_raw: PosRaw,
    pub data: Box<V>,
}
impl Element {
    fn as_variant(&self) -> Element {
        self.to_owned()
    }
    fn is_pattern(&self) -> bool {
        self.data.is_pattern()
    }
    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        self.data.process(typelist)
    }
    fn desugared(&self, out: &mut impl Print) -> Result<Element, ZyxtError> {
        Element {
            pos_raw: self.pos_raw.to_owned(),
            data: *self.data.desugared(&self.pos_raw, out)?,
        }
    }
    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        self.data.interpret_expr(i_data)
    }
}

macro_rules! for_all_variants {
    ($self:ident, $f:ident $(, $args:tt)*) => {
        match &$self {
            ElementVariants::Comment(v) => v.$f($($args,)*),
            ElementVariants::Call(v) => v.$f($($args,)*),
            ElementVariants::UnaryOpr(v) => v.$f($($args,)*),
            ElementVariants::BinaryOpr(v) => v.$f($($args,)*),
            ElementVariants::Declare(v) => v.$f($($args,)*),
            ElementVariants::Set(v) => v.$f($($args,)*),
            ElementVariants::Literal(v) => v.$f($($args,)*),
            ElementVariants::Ident(v) => v.$f($($args,)*),
            ElementVariants::If(v) => v.$f($($args,)*),
            ElementVariants::Block(v) => v.$f($($args,)*),
            ElementVariants::Delete(v) => v.$f($($args,)*),
            ElementVariants::Return(v) => v.$f($($args,)*),
            ElementVariants::Procedure(v) => v.$f($($args,)*),
            ElementVariants::Preprocess(v) => v.$f($($args,)*),
            ElementVariants::Defer(v) => v.$f($($args,)*),
            ElementVariants::Class(v) => v.$f($($args,)*),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, EnumAsInner)]
pub enum ElementVariants {
    Comment(Comment),
    Call(Call),
    UnaryOpr(UnaryOpr),
    BinaryOpr(BinaryOpr),
    Declare(Declare),
    Set(Set),
    Literal(Literal),
    Ident(Ident),
    If(If),
    Block(Block),
    Delete(Delete),
    Return(Return),
    Procedure(Procedure),
    Preprocess(Preprocess),
    Defer(Defer),
    Class(Class),
}
impl ElementData for ElementVariants {
    fn as_variant(&self) -> ElementVariants {
        self.to_owned()
    }
    fn is_pattern(&self) -> bool {
        for_all_variants!(self, is_pattern)
    }
    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        for_all_variants!(self, process, pos_raw, typelist)
    }
    fn desugared(
        &self,
        pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariants, ZyxtError> {
        for_all_variants!(self, desugared, pos_raw, out)
    }
    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        for_all_variants!(self, interpret_expr, i_data)
    }
}

pub trait VecElementRaw {
    fn get_raw(&self) -> String;
}
impl VecElementRaw for Vec<Element> {
    fn get_raw(&self) -> String {
        self.iter()
            .map(|e| e.pos_raw.raw)
            .collect::<Vec<String>>()
            .join("\n")
    }
}
