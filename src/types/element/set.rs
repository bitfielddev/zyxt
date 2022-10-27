use crate::{
    types::{
        element::{Element, ElementData, ElementVariant},
        position::PosRaw,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Set {
    pub variable: Element,
    pub content: Element,
}

impl ElementData for Set {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Set(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        if !self.variable.is_pattern() {
            return Err(ZyxtError::error_2_2(self.variable.to_owned()).with_element(&self.variable));
        }
        let content_type = self.content.process(typelist)?;
        let name = if let ElementVariant::Ident(ident) = &*self.variable.data {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        let var_type = typelist.get_val(name, pos_raw)?;
        if content_type != var_type {
            Err(ZyxtError::error_4_3(name, var_type, content_type).with_pos_raw(pos_raw))
        } else {
            Ok(var_type)
        }
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        let mut new_self = self.to_owned();
        new_self.content = self.content.desugared(out)?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        let var = self.content.interpret_expr(i_data);
        let name = if let ElementVariant::Ident(ident) = &*self.variable.data {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        i_data.set_val(name, &var.to_owned()?, &Default::default())?; // TODO
        var
    }
}
