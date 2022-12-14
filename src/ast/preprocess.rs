use crate::{
    ast::{Element, ElementData},
    types::position::{GetSpan, Span},
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Preprocess {
    pub kwd_span: Span,
    pub content: Box<Element>,
}
impl GetSpan for Preprocess {
    fn span(&self) -> Option<Span> {
        self.kwd_span.merge_span(&self.content)
    }
}

impl ElementData for Preprocess {
    fn as_variant(&self) -> Element {
        Element::Preprocess(self.to_owned())
    }

    fn desugared(&self, out: &mut impl Print) -> ZResult<Element> {
        let mut pre_instructions = self.content.desugared(out)?;
        let mut pre_typelist = InterpreterData::<Type<Element>, _>::new(out);
        pre_instructions.process(&mut pre_typelist)?;
        let mut i_data = InterpreterData::<Value, _>::new(out);
        let pre_value = pre_instructions.interpret_expr(&mut i_data)?;
        Ok(pre_value.as_element())
    }
}
