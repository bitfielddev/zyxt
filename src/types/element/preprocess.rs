use crate::{
    types::{
        element::{block::Block, Element, ElementData, ElementVariant},
        position::PosRaw,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Preprocess {
    pub content: Element<Block>,
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
        let mut pre_instructions: &Block = self
            .content
            .data
            .desugared(&Default::default(), out)?
            .as_block()
            .unwrap();
        pre_instructions.process(&Default::default(), &mut pre_typelist)?;
        let mut i_data = InterpreterData::<Value, _>::new(out);
        let pre_value = pre_instructions.interpret_block(&mut i_data, true, false)?;
        Ok(*pre_value.as_element().data)
    }
}
