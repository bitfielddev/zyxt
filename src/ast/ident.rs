use smol_str::SmolStr;

use crate::{
    ast::{Element, ElementData},
    types::position::{GetSpan, Span},
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Ident {
    pub name: SmolStr,
    pub name_span: Option<Span>,
    pub dot_span: Option<Span>,
    pub parent: Option<Box<Element>>,
}
impl GetSpan for Ident {
    fn span(&self) -> Option<Span> {
        self.parent
            .merge_span(&self.dot_span)
            .merge_span(&self.name_span)
    }
}

impl ElementData for Ident {
    fn as_variant(&self) -> Element {
        Element::Ident(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        true
    }
    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        typelist.get_val(&self.name, &self.name_span)
    } // TODO change sig of get_val

    fn desugared(&self, out: &mut impl Print) -> ZResult<Element> {
        let mut new_self = self.to_owned();
        new_self.parent = new_self
            .parent
            .map(|a| a.desugared(out))
            .transpose()?
            .map(|a| a.into());
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        i_data.get_val(&self.name, &self.name_span) // TODO
    }
}
