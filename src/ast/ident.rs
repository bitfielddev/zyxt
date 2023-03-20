use smol_str::SmolStr;

use crate::{
    ast::{Ast, AstData, Reconstruct},
    types::position::{GetSpan, Span},
    SymTable, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ident {
    pub name: SmolStr,
    pub name_span: Option<Span>,
}
impl GetSpan for Ident {
    fn span(&self) -> Option<Span> {
        self.name_span.to_owned()
    }
}

impl AstData for Ident {
    fn as_variant(&self) -> Ast {
        Ast::Ident(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        true
    }
    fn typecheck(&mut self, ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        ty_symt.get_val(&self.name, &self.name_span)
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        val_symt.get_val(&self.name, &self.name_span)
    }
}

impl Reconstruct for Ident {
    fn reconstruct(&self) -> String {
        self.name.to_owned().into()
    }
}

impl Ident {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            name_span: None,
        }
    }
}
