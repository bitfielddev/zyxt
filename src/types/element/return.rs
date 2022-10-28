use crate::{
    types::{
        element::{Element, ElementData, ElementVariant},
        position::PosRaw,
        typeobj::unit_t::UNIT_T,
    },
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Return {
    pub value: Element,
}

impl ElementData for Return {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Return(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        Ok(UNIT_T.as_type().as_type_element())
    }

    fn desugared(&self, _pos_raw: &PosRaw, out: &mut impl Print) -> ZResult<ElementVariant> {
        Ok(Self {
            value: self.value.desugared(out)?,
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        Ok(Value::Return(Box::new(self.value.interpret_expr(i_data)?)))
    }
}
