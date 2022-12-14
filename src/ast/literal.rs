use crate::{
    ast::{Element, ElementData},
    types::position::{GetSpan, Span},
    InterpreterData, Print, Type, Value, ZResult,
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

impl ElementData for Literal {
    fn as_variant(&self) -> Element {
        Element::Literal(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn process<O: Print>(
        &mut self,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        Ok(self.content.get_type_obj().as_type_element())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        Ok(if let Value::PreType(v) = &self.content {
            Value::Type(v.as_type_value(i_data)?)
        } else {
            self.content.to_owned()
        })
    }
}
