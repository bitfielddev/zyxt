use smol_str::SmolStr;

use crate::{
    types::element::{Element, ElementData, ElementVariants, PosRaw},
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Literal {
    pub content: Value,
}

impl ElementData for Literal {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Literal(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        _typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(self.data.content.get_type_obj().as_type_element())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
