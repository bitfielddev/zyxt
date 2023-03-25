use std::sync::Arc;

use tracing::debug;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    types::position::{GetSpan, Span},
    InterpretSymTable, Type, TypecheckSymTable, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Set {
    pub variable: Box<Ast>,
    pub eq_span: Option<Span>,
    pub content: Box<Ast>,
}
impl GetSpan for Set {
    fn span(&self) -> Option<Span> {
        self.variable
            .merge_span(&self.eq_span)
            .merge_span(&self.content)
    }
}

impl AstData for Set {
    fn as_variant(&self) -> Ast {
        Ast::Set(self.to_owned())
    }

    fn type_check(&mut self, ty_symt: &mut TypecheckSymTable) -> ZResult<Arc<Type>> {
        if !self.variable.is_pattern() {
            return Err(ZError::t006().with_span(&*self.variable));
        }
        let content_type = self.content.type_check(ty_symt)?;
        let name = if let Ast::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        let var_type = ty_symt.get_val(
            name,
            &self.variable.span().unwrap_or_else(|| unreachable!()),
        )?;
        if content_type == var_type {
            Ok(var_type)
        } else {
            Err(ZError::t010(&var_type, &content_type).with_span(&*self)) // TODO span
        }
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring set statement");
        let mut new_self = self.to_owned();
        new_self.content.desugar()?;
        new_self.variable.desugar()?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        let var = self.content.interpret_expr(val_symt)?;
        let name = if let Ast::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        val_symt.set_val(name, var.to_owned(), self)?;
        Ok(var)
    }
}

impl Reconstruct for Set {
    fn reconstruct(&self) -> String {
        format!(
            "{} = {}",
            self.variable.reconstruct(),
            self.content.reconstruct()
        )
    }
}
