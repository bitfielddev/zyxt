use crate::{
    types::{
        element::{binary_opr::BinaryOpr, Element, ElementData},
        position::{GetSpan, Span},
        token::{Flag, OprType},
    },
    InterpreterData, Print, Type, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Declare {
    pub variable: Box<Element>,
    pub content: Box<Element>,
    pub flags: Vec<(Flag, Span)>,
    pub ty: Option<Box<Element>>,
    pub eq_span: Option<Span>,
}
impl GetSpan for Declare {
    fn span(&self) -> Option<Span> {
        self.variable
            .merge_span(self.flags.iter().map(|a| &a.1).collect::<Vec<_>>())
            .merge_span(&self.ty)
            .merge_span(&self.content)
            .merge_span(&self.eq_span)
    }
}

impl ElementData for Declare {
    fn as_variant(&self) -> Element {
        Element::Declare(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        if !self.variable.is_pattern() {
            return Err(ZError::error_2_2(*self.variable.to_owned()).with_span(&*self.variable));
        }
        let content_type = self.content.process(typelist)?;
        let ty = self.ty.as_ref().map(|ty| {
            if let Element::Literal(literal) = &**ty {
                if let Value::Type(t) = &literal.content {
                    t.as_type_element()
                } else {
                    todo!()
                }
            } else {
                todo!()
            }
        });
        let name = if let Element::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        if let Some(ty) = ty {
            typelist.declare_val(name, &ty);
            if content_type != ty {
                let mut new_content = BinaryOpr {
                    ty: OprType::TypeCast,
                    opr_span: None,
                    operand1: self.content.to_owned(),
                    operand2: ty.as_literal().into(),
                }
                .as_variant();
                new_content.process(typelist)?;
                *self = Declare {
                    ty: self.ty.to_owned(),
                    content: new_content.into(),
                    variable: self.variable.to_owned(),
                    flags: self.flags.to_owned(),
                    eq_span: self.eq_span.to_owned(),
                };
            }
        } else {
            typelist.declare_val(name, &content_type);
        }
        Ok(content_type)
    }

    fn desugared(&self, out: &mut impl Print) -> ZResult<Element> {
        let mut new_self = self.to_owned();
        new_self.content = self.content.desugared(out)?.into();
        new_self.ty = self
            .ty
            .as_ref()
            .map(|a| a.desugared(out))
            .transpose()?
            .map(|a| a.into());
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        let name = if let Element::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        let var = self.content.interpret_expr(i_data);
        i_data.declare_val(name, &var.to_owned()?);
        var
    }
}
