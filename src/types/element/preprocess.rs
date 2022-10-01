use crate::{
    gen_instructions,
    interpreter::interpret_block,
    types::element::{block::Block, Element, ElementData, ElementVariant, PosRaw},
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Preprocess {
    content: Block,
}

impl ElementData for Preprocess {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Preprocess(self.to_owned())
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        let mut pre_typelist = InterpreterData::<Type<Element>, _>::new(out);
        let pre_instructions =
            gen_instructions(self.content.content.to_owned(), &mut pre_typelist)?;
        let mut i_data = InterpreterData::<Value, _>::new(out);
        let pre_value = interpret_block(&pre_instructions, &mut i_data, true, false)?;
        Ok(*pre_value.as_element().data)
    }
}
