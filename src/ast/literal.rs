use std::sync::Arc;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    types::position::{GetSpan, Span},
    InterpretSymTable, Type, TypecheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Literal {
    pub span: Option<Span>,
    pub content: Value,
}
impl GetSpan for Literal {
    fn span(&self) -> Option<Span> {
        self.span.span()
    }
}

impl AstData for Literal {
    fn as_variant(&self) -> Ast {
        Ast::Literal(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn typecheck(&mut self, ty_symt: &mut TypecheckSymTable) -> ZResult<Arc<Type>> {
        Ok(self.content.ty())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        Ok(self.content.to_owned())
    }
}
impl Reconstruct for Literal {
    fn reconstruct(&self) -> String {
        format!("{:?}", self.content)
    }
}
