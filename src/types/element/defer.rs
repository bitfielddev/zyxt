use crate::{
    types::{
        element::{Element, ElementData, ElementVariant},
        position::PosRaw,
    },
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Defer {
    pub content: Element,
}

impl ElementData for Defer {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Defer(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        self.content.data.process(pos_raw, typelist)
    }

    fn desugared(&self, _pos_raw: &PosRaw, out: &mut impl Print) -> ZResult<ElementVariant> {
        Ok(Defer {
            content: self.content.desugared(out)?.as_variant(),
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        i_data.add_defer(self.content.to_owned());
        Ok(Value::Unit)
    }
}
