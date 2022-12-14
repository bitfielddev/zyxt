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

use std::fmt::Debug;

use enum_as_inner::EnumAsInner;

use crate::{
    ast::{
        binary_opr::BinaryOpr, block::Block, call::Call, class::Class, declare::Declare,
        defer::Defer, delete::Delete, ident::Ident, literal::Literal, preprocess::Preprocess,
        procedure::Procedure, r#if::If, r#return::Return, set::Set, unary_opr::UnaryOpr,
    },
    types::{
        errors::ZResult,
        interpreter_data::SymTable,
        position::{GetSpan, Span},
        typeobj::Type,
        value::Value,
    },
};

pub trait AstData: Clone + PartialEq + Debug + GetSpan {
    fn as_variant(&self) -> Ast;
    fn is_pattern(&self) -> bool {
        false
    }
    fn process(&mut self, _: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        Ok(Type::Any)
    }
    fn desugared(&self) -> ZResult<Ast> {
        Ok(self.as_variant())
    }
    fn interpret_expr(&self, _: &mut SymTable<Value>) -> ZResult<Value> {
        unreachable!()
    }
}

macro_rules! for_all_variants {
    ($self:expr, $f:ident $(, $args:tt)*) => {
        match $self {
            Ast::Call(v) => v.$f($($args,)*),
            Ast::UnaryOpr(v) => v.$f($($args,)*),
            Ast::BinaryOpr(v) => v.$f($($args,)*),
            Ast::Declare(v) => v.$f($($args,)*),
            Ast::Set(v) => v.$f($($args,)*),
            Ast::Literal(v) => v.$f($($args,)*),
            Ast::Ident(v) => v.$f($($args,)*),
            Ast::If(v) => v.$f($($args,)*),
            Ast::Block(v) => v.$f($($args,)*),
            Ast::Delete(v) => v.$f($($args,)*),
            Ast::Return(v) => v.$f($($args,)*),
            Ast::Procedure(v) => v.$f($($args,)*),
            Ast::Preprocess(v) => v.$f($($args,)*),
            Ast::Defer(v) => v.$f($($args,)*),
            Ast::Class(v) => v.$f($($args,)*),
        }
    }
}

#[derive(Clone, PartialEq, Debug, EnumAsInner)]
pub enum Ast {
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
impl GetSpan for Ast {
    fn span(&self) -> Option<Span> {
        for_all_variants!(&self, span)
    }
}
impl AstData for Ast {
    fn as_variant(&self) -> Ast {
        self.to_owned()
    }
    fn is_pattern(&self) -> bool {
        for_all_variants!(&self, is_pattern)
    }
    fn process(&mut self, typelist: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        for_all_variants!(self, process, typelist)
    }
    fn desugared(&self) -> ZResult<Ast> {
        for_all_variants!(&self, desugared)
    }
    fn interpret_expr(&self, i_data: &mut SymTable<Value>) -> ZResult<Value> {
        for_all_variants!(&self, interpret_expr, i_data)
    }
}
