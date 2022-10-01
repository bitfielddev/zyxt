use smol_str::SmolStr;

use crate::{
    types::element::{block::Block, Element, ElementData, ElementVariants, PosRaw},
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Defer {
    content: Element<Block>,
}

impl ElementData for Defer {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Defer(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(self.content.data.block_type(typelist, false)?.0)
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
