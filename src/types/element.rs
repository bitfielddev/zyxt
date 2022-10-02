pub mod binary_opr;
pub mod block;
pub mod call;
pub mod class;
pub mod comment;
pub mod declare;
pub mod defer;
pub mod delete;
pub mod ident;
pub mod r#if;
pub mod literal;
pub mod preprocess;
pub mod procedure;
pub mod r#return;
pub mod set;
pub mod unary_opr;

use std::fmt::{Debug, Display, Formatter};

use enum_as_inner::EnumAsInner;
use smol_str::SmolStr;

use crate::types::{
    element::{
        binary_opr::BinaryOpr, block::Block, call::Call, class::Class, comment::Comment,
        declare::Declare, defer::Defer, delete::Delete, ident::Ident, literal::Literal,
        preprocess::Preprocess, procedure::Procedure, r#if::If, r#return::Return, set::Set,
        unary_opr::UnaryOpr,
    },
    errors::ZyxtError,
    interpreter_data::InterpreterData,
    position::Position,
    printer::Print,
    token::{OprType, Token},
    typeobj::{unit_t::UNIT_T, Type},
    value::Value,
};

#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PosRaw {
    pub position: Position,
    pub raw: SmolStr,
}

pub trait ElementData: Clone + PartialEq + Eq + Debug {
    fn as_variant(&self) -> ElementVariant;
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
    ) -> Result<ElementVariant, ZyxtError> {
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
pub struct Element<V: ElementData = ElementVariant> {
    pub pos_raw: PosRaw,
    pub data: Box<V>,
}
impl<V: ElementData> Element<V> {
    pub fn as_variant(&self) -> Element {
        self.to_owned()
    }
    pub fn is_pattern(&self) -> bool {
        self.data.is_pattern()
    }
    pub fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        self.data.process(typelist)
    }
    pub fn desugared(&self, out: &mut impl Print) -> Result<Element, ZyxtError> {
        Element {
            pos_raw: self.pos_raw.to_owned(),
            data: *self.data.desugared(&self.pos_raw, out)?,
        }
    }
    pub fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        self.data.interpret_expr(i_data)
    }
}

macro_rules! for_all_variants {
    ($self:ident, $f:ident $(, $args:tt)*) => {
        match &$self {
            ElementVariant::Comment(v) => v.$f($($args,)*),
            ElementVariant::Call(v) => v.$f($($args,)*),
            ElementVariant::UnaryOpr(v) => v.$f($($args,)*),
            ElementVariant::BinaryOpr(v) => v.$f($($args,)*),
            ElementVariant::Declare(v) => v.$f($($args,)*),
            ElementVariant::Set(v) => v.$f($($args,)*),
            ElementVariant::Literal(v) => v.$f($($args,)*),
            ElementVariant::Ident(v) => v.$f($($args,)*),
            ElementVariant::If(v) => v.$f($($args,)*),
            ElementVariant::Block(v) => v.$f($($args,)*),
            ElementVariant::Delete(v) => v.$f($($args,)*),
            ElementVariant::Return(v) => v.$f($($args,)*),
            ElementVariant::Procedure(v) => v.$f($($args,)*),
            ElementVariant::Preprocess(v) => v.$f($($args,)*),
            ElementVariant::Defer(v) => v.$f($($args,)*),
            ElementVariant::Class(v) => v.$f($($args,)*),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, EnumAsInner)]
pub enum ElementVariant {
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
impl ElementData for ElementVariant {
    fn as_variant(&self) -> ElementVariant {
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
    ) -> Result<ElementVariant, ZyxtError> {
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
