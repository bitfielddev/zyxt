use crate::{
    types::{
        element::{
            binary_opr::BinaryOpr, ident::Ident, Element, ElementData, ElementVariant, PosRaw,
        },
        token::{Flag, OprType},
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Declare {
    pub variable: Element<Ident>,
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
    ) -> Result<Type<Element>, ZyxtError> {
        if !self.variable.is_pattern() {
            return Err(ZyxtError::error_2_2(self.variable.to_owned()).with_element(&self.variable));
        }
        let content_type = self.content.process(typelist)?;
        let ty = if let ElementVariant::Literal(literal) = self.ty.data.as_ref() {
            if let Value::Type(t) = &literal.content {
                t
            } else {
                todo!()
            }
        } else {
            todo!()
        }
        .as_type_element();
        if ty == Type::Any {
            typelist.declare_val(&self.variable.data.name, &content_type);
        } else {
            typelist.declare_val(&self.variable.data.name, &ty);
            if content_type != ty {
                let mut new_content = BinaryOpr {
                    ty: OprType::TypeCast,
                    operand1: self.content.to_owned(),
                    operand2: ty.as_literal(),
                }
                .as_variant();
                new_content.process(typelist)?;
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
        };
        Ok(content_type)
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        let mut new_self = self.to_owned();
        new_self.content = self.content.desugared(out)?;
        new_self.ty = self.ty.map(|a| a.desugared(out)).transpose()?;
        Ok(new_self.as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        let var = self.content.interpret_expr(i_data);
        i_data.declare_val(&self.variable.data.name, &var.to_owned()?);
        var
    }
}