mod argument;
mod binary_opr;
mod block;
mod call;
mod class;
mod comment;
mod condition;
mod declare;
mod defer;
mod delete;
mod ident;
mod r#if;
mod literal;
mod member;
mod preprocess;
mod procedure;
mod r#return;
mod set;
mod unary_opr;

use std::{fmt::Debug, sync::Arc};

pub use argument::Argument;
pub use binary_opr::BinaryOpr;
pub use block::Block;
pub use call::Call;
pub use class::Class;
pub use comment::Comment;
pub use condition::Condition;
pub use declare::Declare;
pub use defer::Defer;
pub use delete::Delete;
use enum_as_inner::EnumAsInner;
pub use ident::Ident;
use itertools::Itertools;
pub use literal::Literal;
pub use preprocess::Preprocess;
pub use procedure::Procedure;
pub use r#if::If;
pub use r#return::Return;
pub use set::Set;
pub use unary_opr::UnaryOpr;

pub use crate::ast::member::Member;
use crate::{
    errors::ZResult,
    primitives::ANY_T,
    types::{
        position::{GetSpan, Span},
        r#type::{Type, TypeCheckType},
        sym_table::{InterpretSymTable, TypeCheckSymTable},
        value::Value,
    },
};

pub trait AstData: Clone + PartialEq + Debug + GetSpan {
    fn as_variant(&self) -> Ast;
    fn is_pattern(&self) -> bool {
        false
    }
    fn type_check(&mut self, _ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        Ok(Arc::clone(&ANY_T).into())
    }
    fn desugared(&self) -> ZResult<Ast> {
        Ok(self.as_variant())
    }
    fn interpret_expr(&self, _: &mut InterpretSymTable) -> ZResult<Value> {
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
            Ast::Member(v) => v.$f($($args,)*),
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
    Member(Member),
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
    fn type_check(&mut self, ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        for_all_variants!(self, type_check, ty_symt)
    }
    fn desugared(&self) -> ZResult<Ast> {
        for_all_variants!(&self, desugared)
    }
    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        for_all_variants!(&self, interpret_expr, val_symt)
    }
}
impl Ast {
    pub fn desugar(&mut self) -> ZResult<()> {
        *self = self.desugared()?;
        Ok(())
    }
}

pub trait Reconstruct {
    fn reconstruct(&self) -> String;
}

impl Reconstruct for Ast {
    fn reconstruct(&self) -> String {
        for_all_variants!(&self, reconstruct)
    }
}

impl Reconstruct for Vec<Ast> {
    fn reconstruct(&self) -> String {
        self.iter().map(Reconstruct::reconstruct).join(" ; ")
    }
}
