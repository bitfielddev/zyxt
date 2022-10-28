use crate::{
    types::{
        element::{binary_opr::BinaryOpr, Element, ElementData, ElementVariant},
        position::PosRaw,
        token::{Flag, OprType},
    },
    InterpreterData, Print, Type, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Declare {
    pub variable: Element,
    pub content: Element,
    pub flags: Vec<Flag>,
    pub ty: Option<Element>,
}

impl ElementData for Declare {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Declare(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> ZResult<Type<Element>> {
        if !self.variable.is_pattern() {
            return Err(ZError::error_2_2(self.variable.to_owned()).with_element(&self.variable));
        }
        let content_type = self.content.process(typelist)?;
        let ty = self.ty.as_ref().map(|ty| {
            if let ElementVariant::Literal(literal) = &*ty.data {
                if let Value::Type(t) = &literal.content {
                    t.as_type_element()
                } else {
                    todo!()
                }
            } else {
                todo!()
            }
        });
        let name = if let ElementVariant::Ident(ident) = &*self.variable.data {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        if let Some(ty) = ty {
            typelist.declare_val(name, &ty);
            if content_type != ty {
                let mut new_content = BinaryOpr {
                    ty: OprType::TypeCast,
                    operand1: self.content.to_owned(),
                    operand2: ty.as_literal(),
                }
                .as_variant();
                new_content.process(pos_raw, typelist)?;
                *self = Declare {
                    ty: self.ty.to_owned(),
                    content: Element {
                        pos_raw: pos_raw.to_owned(),
                        data: Box::new(new_content),
                    },
                    variable: self.variable.to_owned(),
                    flags: self.flags.to_owned(),
                };
            }
        } else {
            typelist.declare_val(name, &content_type);
        }
        Ok(content_type)
    }

    fn desugared(&self, _pos_raw: &PosRaw, out: &mut impl Print) -> ZResult<ElementVariant> {
        let mut new_self = self.to_owned();
        new_self.content = self.content.desugared(out)?;
        new_self.ty = self.ty.as_ref().map(|a| a.desugared(out)).transpose()?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(&self, i_data: &mut InterpreterData<Value, O>) -> ZResult<Value> {
        let name = if let ElementVariant::Ident(ident) = &*self.variable.data {
            &ident.name
        } else {
            unimplemented!() // TODO
        };
        let var = self.content.interpret_expr(i_data);
        i_data.declare_val(name, &var.to_owned()?);
        var
    }
}
