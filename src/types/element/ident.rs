use smol_str::SmolStr;

use crate::{
    types::{
        element::{Element, ElementData, ElementVariant},
        position::PosRaw,
    },
    InterpreterData, Print, Type, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Ident {
    pub name: SmolStr,
    pub parent: Option<Element>,
}

impl ElementData for Ident {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Ident(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        true
    }
    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        typelist.get_val(&self.name, pos_raw)
    }

    fn desugared(&self, _pos_raw: &PosRaw, out: &mut impl Print) -> ZResult<ElementVariant> {
        let mut new_self = self.to_owned();
        new_self.parent = new_self.parent.map(|a| a.desugared(out)).transpose()?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        i_data.get_val(&self.name, &Default::default()) // TODO
    }
}
