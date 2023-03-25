use std::sync::Arc;

use tracing::debug;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    types::position::{GetSpan, Span},
    InterpretSymTable, Type, TypecheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Defer {
    pub kwd_span: Span,
    pub content: Box<Ast>,
}
impl GetSpan for Defer {
    fn span(&self) -> Option<Span> {
        self.kwd_span.merge_span(&self.content)
    }
}

impl AstData for Defer {
    fn as_variant(&self) -> Ast {
        Ast::Defer(self.to_owned())
    }

    fn type_check(&mut self, ty_symt: &mut TypecheckSymTable) -> ZResult<Arc<Type>> {
        self.content.type_check(ty_symt)
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring defer statement");
        Ok(Self {
            kwd_span: self.kwd_span.to_owned(),
            content: self.content.desugared()?.as_variant().into(),
        }
        .as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        val_symt.add_defer(*self.content.to_owned());
        Ok(Value::Unit)
    }
}
impl Reconstruct for Defer {
    fn reconstruct(&self) -> String {
        format!("defer {}", self.content.reconstruct())
    }
}
