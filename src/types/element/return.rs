use crate::{
    types::{
        element::{Element, ElementData},
        position::{GetSpan, Span},
        typeobj::unit_t::UNIT_T,
    },
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Return {
    pub kwd_span: Option<Span>,
    pub value: Box<Element>,
}
impl GetSpan for Return {
    fn span(&self) -> Option<Span> {
        self.kwd_span.merge_span(&self.value)
    }
}

impl ElementData for Return {
    fn as_variant(&self) -> Element {
        Element::Return(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        Ok(UNIT_T.as_type().as_type_element())
    }

    fn desugared(&self, out: &mut impl Print) -> ZResult<Element> {
        Ok(Self {
            kwd_span: self.kwd_span.to_owned(),
            value: self.value.desugared(out)?.into(),
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        Ok(Value::Return(Box::new(self.value.interpret_expr(i_data)?)))
    }
}
