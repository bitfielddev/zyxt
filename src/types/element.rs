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

use std::{borrow::Cow, fmt::Debug};

use enum_as_inner::EnumAsInner;

use crate::types::{
    element::{
        binary_opr::BinaryOpr, block::Block, call::Call, class::Class, declare::Declare,
        defer::Defer, delete::Delete, ident::Ident, literal::Literal, preprocess::Preprocess,
        procedure::Procedure, r#if::If, r#return::Return, set::Set, unary_opr::UnaryOpr,
    },
    errors::ZResult,
    interpreter_data::InterpreterData,
    position::{GetSpan, Span},
    printer::Print,
    typeobj::Type,
    value::Value,
};

pub trait ElementData: Clone + PartialEq + Debug + GetSpan {
    fn as_variant(&self) -> Element;
    fn is_pattern(&self) -> bool {
        false
    }
    fn process<O: Print>(
        &mut self,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        Ok(Type::Any)
    }
    fn desugared(&self, _out: &mut impl Print) -> ZResult<Element> {
        Ok(self.as_variant())
    }
    fn interpret_expr<O: Print>(&self, _: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        unreachable!()
    }
}

macro_rules! for_all_variants {
    ($self:expr, $f:ident $(, $args:tt)*) => {
        match $self {
            Element::Call(v) => v.$f($($args,)*),
            Element::UnaryOpr(v) => v.$f($($args,)*),
            Element::BinaryOpr(v) => v.$f($($args,)*),
            Element::Declare(v) => v.$f($($args,)*),
            Element::Set(v) => v.$f($($args,)*),
            Element::Literal(v) => v.$f($($args,)*),
            Element::Ident(v) => v.$f($($args,)*),
            Element::If(v) => v.$f($($args,)*),
            Element::Block(v) => v.$f($($args,)*),
            Element::Delete(v) => v.$f($($args,)*),
            Element::Return(v) => v.$f($($args,)*),
            Element::Procedure(v) => v.$f($($args,)*),
            Element::Preprocess(v) => v.$f($($args,)*),
            Element::Defer(v) => v.$f($($args,)*),
            Element::Class(v) => v.$f($($args,)*),
        }
    }
}

#[derive(Clone, PartialEq, Debug, EnumAsInner)]
pub enum Element {
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
impl GetSpan for Element {
    fn span(&self) -> Option<Span> {
        for_all_variants!(&self, span)
    }
}
impl ElementData for Element {
    fn as_variant(&self) -> Element {
        self.to_owned()
    }
    fn is_pattern(&self) -> bool {
        for_all_variants!(&self, is_pattern)
    }
    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        for_all_variants!(self, process, typelist)
    }
    fn desugared(&self, out: &mut impl Print) -> ZResult<Element> {
        for_all_variants!(&self, desugared, out)
    }
    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        for_all_variants!(&self, interpret_expr, i_data)
    }
}
