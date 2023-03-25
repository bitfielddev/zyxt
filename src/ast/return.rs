use std::sync::Arc;

use tracing::debug;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    primitives::UNIT_T,
    types::{
        position::{GetSpan, Span},
        r#type::TypeCheckType,
    },
    InterpretSymTable, Type, TypeCheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Return {
    pub kwd_span: Option<Span>,
    pub value: Box<Ast>,
}
impl GetSpan for Return {
    fn span(&self) -> Option<Span> {
        self.kwd_span.merge_span(&self.value)
    }
}

impl AstData for Return {
    fn as_variant(&self) -> Ast {
        Ast::Return(self.to_owned())
    }

    fn type_check(&mut self, _ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        Ok(Arc::clone(&UNIT_T).into())
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring return statement");
        Ok(Self {
            kwd_span: self.kwd_span.to_owned(),
            value: self.value.desugared()?.into(),
        }
        .as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        Ok(Value::Return(Box::new(
            self.value.interpret_expr(val_symt)?,
        )))
    }
}

impl Reconstruct for Return {
    fn reconstruct(&self) -> String {
        format!("ret {}", self.value.reconstruct())
    }
}
