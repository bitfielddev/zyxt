use crate::{
    types::{
        element::{ident::Ident, Element, ElementData},
        position::{GetSpan, Span},
        typeobj::unit_t::UNIT_T,
    },
    InterpreterData, Print, Type, Value, ZResult,
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

impl ElementData for Delete {
    fn as_variant(&self) -> Element {
        Element::Delete(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        Ok(UNIT_T.get_instance().as_type_element())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        for name in &self.names {
            i_data.delete_val(&name.name, &Span::default())?; // TODO
        }
        Ok(Value::Unit)
    }
}
