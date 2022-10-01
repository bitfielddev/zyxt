use smol_str::SmolStr;

use crate::{
    types::{
        element::{ident::Ident, Element, ElementData, ElementVariants, PosRaw},
        token::OprType,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Set {
    variable: Element<Ident>, // variable
    content: Element,
}

impl ElementData for Set {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Set(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        if !self.variable.is_pattern() {
            return Err(ZyxtError::error_2_2(self.variable.to_owned()).with_element(&self.variable));
        }
        let content_type = self.content.process(self.typelist)?;
        let var_type = self.typelist.get_val(&self.variable.data.name, pos_raw)?;
        if content_type != var_type {
            Err(
                ZyxtError::error_4_3(self.variable.data.name, var_type, content_type)
                    .with_pos_raw(pos_raw),
            )
        } else {
            Ok(var_type)
        }
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
