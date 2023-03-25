use std::sync::Arc;

use smol_str::SmolStr;
use tracing::debug;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    errors::ToZResult,
    types::{
        position::{GetSpan, Span},
        r#type::TypeCheckType,
        token::AccessType,
    },
    InterpretSymTable, TypeCheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Member {
    pub ty: AccessType,
    pub name: SmolStr,
    pub parent: Box<Ast>,
    pub name_span: Option<Span>,
    pub dot_span: Option<Span>,
}
impl GetSpan for Member {
    fn span(&self) -> Option<Span> {
        self.parent
            .merge_span(&self.dot_span)
            .merge_span(&self.name_span)
    }
}

impl AstData for Member {
    fn as_variant(&self) -> Ast {
        Ast::Member(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        true
    }

    fn type_check(&mut self, ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        debug!(span = ?self.span(), "Type-checking member access");
        let parent_type = self.parent.type_check(ty_symt)?;
        let res = match self.ty {
            AccessType::Method => unreachable!(),
            AccessType::Namespace => parent_type
                .as_const()?
                .namespace()
                .get(&self.name)
                .ok_or_else(|| todo!())
                .map(|a| Arc::clone(a))?,
            AccessType::Field => parent_type
                .fields()
                .get(&self.name)
                .ok_or_else(|| todo!())
                .map(Arc::clone)?,
        };
        Ok(res.into())
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring member access");
        let mut new_self = self.to_owned();
        new_self.parent.desugar()?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        let parent = self.parent.interpret_expr(val_symt)?;
        match self.ty {
            AccessType::Method => unreachable!(),
            AccessType::Field => todo!(),
            AccessType::Namespace => Ok(parent
                .as_type()
                .z()?
                .namespace()
                .get(&self.name)
                .z()?
                .to_owned()),
        }
    }
}

impl Reconstruct for Member {
    fn reconstruct(&self) -> String {
        format!(
            "{} {} {}",
            self.parent.reconstruct(),
            match self.ty {
                AccessType::Field => ".",
                AccessType::Method => ":.",
                AccessType::Namespace => "::",
            },
            self.name
        )
    }
}
