use smol_str::SmolStr;

use crate::{
    types::{
        element::{ident::Ident, Element, ElementData, ElementVariants, PosRaw},
        typeobj::unit_t::UNIT_T,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Return {
    value: Element,
}

impl ElementData for Return {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Return(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(UNIT_T.to_type().to_type_element())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
