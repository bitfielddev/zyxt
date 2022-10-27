use crate::{
    types::{
        element::{Element, ElementData, ElementVariant},
        position::PosRaw,
        typeobj::unit_t::UNIT_T,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
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
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(UNIT_T.to_type().to_type_element())
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        Ok(Self {
            value: self.value.desugared(out)?,
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        Ok(Value::Return(Box::new(self.value.interpret_expr(i_data)?)))
    }
}
