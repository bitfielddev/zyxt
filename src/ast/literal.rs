use crate::{
    ast::{Ast, AstData},
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
    fn process(&mut self, _typelist: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        Ok(self.content.get_type_obj().as_type_element())
    }

    fn interpret_expr(&self, i_data: &mut SymTable<Value>) -> ZResult<Value> {
        Ok(if let Value::PreType(v) = &self.content {
            Value::Type(v.as_type_value(i_data)?)
        } else {
            self.content.to_owned()
        })
    }
}
