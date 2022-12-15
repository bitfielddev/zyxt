use smol_str::SmolStr;

use crate::{
    ast::{Ast, AstData},
    types::position::{GetSpan, Span},
    SymTable, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Ident {
    pub name: SmolStr,
    pub name_span: Option<Span>,
    pub dot_span: Option<Span>,
    pub parent: Option<Box<Ast>>,
}
impl GetSpan for Ident {
    fn span(&self) -> Option<Span> {
        self.parent
            .merge_span(&self.dot_span)
            .merge_span(&self.name_span)
    }
}

impl AstData for Ident {
    fn as_variant(&self) -> Ast {
        Ast::Ident(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        true
    }
    fn process(&mut self, ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        ty_symt.get_val(&self.name, &self.name_span)
    }

    fn desugared(&self) -> ZResult<Ast> {
        let mut new_self = self.to_owned();
        new_self.parent = new_self
            .parent
            .map(|a| a.desugared())
            .transpose()?
            .map(Into::into);
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        val_symt.get_val(&self.name, &self.name_span)
    }
}
