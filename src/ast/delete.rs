use crate::{
    ast::{ident::Ident, Ast, AstData},
    primitives::UNIT_T,
    types::position::{GetSpan, Span},
    InterpreterData, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Delete {
    pub kwd_span: Option<Span>,
    pub names: Vec<Ident>,
}
impl GetSpan for Delete {
    fn span(&self) -> Option<Span> {
        self.names.merge_span(&self.kwd_span)
    }
}

impl AstData for Delete {
    fn as_variant(&self) -> Ast {
        Ast::Delete(self.to_owned())
    }

    fn process(&mut self, _typelist: &mut InterpreterData<Type<Ast>>) -> ZResult<Type<Ast>> {
        Ok(UNIT_T.get_instance().as_type_element())
    }

    fn interpret_expr(&self, i_data: &mut InterpreterData<Value>) -> ZResult<Value> {
        for name in &self.names {
            i_data.delete_val(&name.name, &Span::default())?; // TODO
        }
        Ok(Value::Unit)
    }
}
