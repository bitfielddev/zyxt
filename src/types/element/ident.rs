use smol_str::SmolStr;

use crate::{
    types::element::{Element, ElementData, ElementVariants, PosRaw},
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Ident {
    pub name: SmolStr,
    pub parent: Option<Element>,
}

impl ElementData for Ident {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Ident(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        true
    }
    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        typelist.get_val(&self.name, pos_raw)
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
