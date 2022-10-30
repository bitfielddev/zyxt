use crate::{
    types::{
        element::{Element, ElementData, ElementVariant},
        position::PosRaw,
    },
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Preprocess {
    pub content: Element,
}

impl ElementData for Preprocess {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Preprocess(self.to_owned())
    }

    fn desugared(&self, _pos_raw: &PosRaw, out: &mut impl Print) -> ZResult<ElementVariant> {
        let mut pre_instructions = self.content.data.desugared(&Default::default(), out)?;
        let mut pre_typelist = InterpreterData::<Type<Element>, _>::new(out);
        pre_instructions.process(&Default::default(), &mut pre_typelist)?;
        let mut i_data = InterpreterData::<Value, _>::new(out);
        let pre_value = pre_instructions.interpret_expr(&mut i_data)?;
        Ok(*pre_value.as_element().data)
    }
}
