use crate::{
    types::{
        element::{block::Block, Element, ElementData, ElementVariant},
        position::PosRaw,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Defer {
    pub content: Element<Block>,
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
                data: Box::new(
                    self.content
                        .desugared(out)?
                        .data
                        .as_block()
                        .unwrap()
                        .to_owned(),
                ),
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
