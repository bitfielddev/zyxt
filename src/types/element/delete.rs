use crate::{
    types::{
        element::{ident::Ident, Element, ElementData, ElementVariants, PosRaw},
        typeobj::unit_t::UNIT_T,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Delete {
    names: Vec<Element<Ident>>,
}

impl ElementData for Delete {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Delete(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(UNIT_T.as_type().as_type_element())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
