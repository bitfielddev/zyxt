use crate::{
    ast::{Ast, AstData, Reconstruct},
    types::position::{GetSpan, Span},
    SymTable, Type, Value, ZResult,
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
    fn process(&mut self, _ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        Ok(self.content.get_type_obj().as_type_element())
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        Ok(if let Value::PreType(v) = &self.content {
            Value::Type(v.as_type_value(val_symt)?)
        } else {
            self.content.to_owned()
        })
    }
}
impl Reconstruct for Literal {
    fn reconstruct(&self) -> String {
        format!("{:?}", self.content)
    }
}
