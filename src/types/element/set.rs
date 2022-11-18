use crate::{
    types::{
        element::{Element, ElementData},
        position::{GetSpan, Span},
    },
    InterpreterData, Print, Type, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Set {
    pub variable: Box<Element>,
    pub eq_span: Option<Span>,
    pub content: Box<Element>,
}
impl GetSpan for Set {
    fn span(&self) -> Option<Span> {
        self.variable
            .merge_span(&self.eq_span)
            .merge_span(&self.content)
    }
}

impl ElementData for Set {
    fn as_variant(&self) -> Element {
        Element::Set(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        if !self.variable.is_pattern() {
            return Err(ZError::error_2_2(*self.variable.to_owned()).with_span(&*self.variable));
        }
        let content_type = self.content.process(typelist)?;
        let name = if let Element::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        let var_type = typelist.get_val(name, &self.variable.span().unwrap())?;
        if content_type != var_type {
            Err(ZError::error_4_3(name, var_type, content_type)) // TODO span
        } else {
            Ok(var_type)
        }
    }

    fn desugared(&self, out: &mut impl Print) -> ZResult<Element> {
        let mut new_self = self.to_owned();
        new_self.content = self.content.desugared(out)?.into();
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        let var = self.content.interpret_expr(i_data);
        let name = if let Element::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        i_data.set_val(name, &var.to_owned()?, &Default::default())?; // TODO
        var
    }
}
