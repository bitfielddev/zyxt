use crate::{
    types::element::{block::Block, Element, ElementData, ElementVariant, PosRaw},
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Defer {
    content: Element<Block>,
}

impl ElementData for Defer {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Defer(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        Ok(self.content.data.block_type(typelist, false)?.0)
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        Ok(Defer {
            content: Element {
                pos_raw: self.content.pos_raw.to_owned(),
                data: self.content.desugared(out)?.as_block().unwrap(),
            },
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        i_data.add_defer(self.content.to_owned());
        Ok(Value::Unit)
    }
}
