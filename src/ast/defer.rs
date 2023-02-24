use crate::{
    ast::{Ast, AstData, Reconstruct},
    types::position::{GetSpan, Span},
    SymTable, Type, Value, ZResult,
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

    fn process(&mut self, ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        self.content.process(ty_symt)
    }

    fn desugared(&self) -> ZResult<Ast> {
        Ok(Self {
            kwd_span: self.kwd_span.to_owned(),
            content: self.content.desugared()?.as_variant().into(),
        }
        .as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        val_symt.add_defer(*self.content.to_owned());
        Ok(Value::Unit)
    }
}
impl Reconstruct for Defer {
    fn reconstruct(&self) -> String {
        format!("defer {}", self.content.reconstruct())
    }
}
