use smol_str::SmolStr;

use crate::{
    types::{
        element::{
            binary_opr::BinaryOpr, ident::Ident, Element, ElementData, ElementVariants, PosRaw,
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
    pub type_: Option<Element>,
}

impl ElementData for Declare {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Declare(self.to_owned())
    }

    fn process<O: Print>(
        &mut self,
        pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        if !self.data.variable.is_pattern() {
            return Err(ZyxtError::error_2_2(self.data.variable).with_element(&self.data.variable));
        }
        let content_type = self.data.content.process(typelist)?;
        let type_ = if let ElementVariants::Literal(literal) = self.data.type_.data.as_ref() {
            if let Value::Type(t) = &literal.content {
                t
            } else {
                todo!()
            }
        } else {
            todo!()
        }
        .as_type_element();
        if type_ == Type::Any {
            typelist.declare_val(&self.data.variable.data.name, &content_type);
        } else {
            typelist.declare_val(&self.data.variable.data.name, &type_);
            if content_type != type_ {
                let mut new_content = BinaryOpr {
                    type_: OprType::TypeCast,
                    operand1: self.data.content.to_owned(),
                    operand2: type_.as_literal(),
                }
                .as_variant();
                new_content.process(typelist)?;
                *self = Declare {
                    type_: self.data.type_.to_owned(),
                    content: Element {
                        pos_raw: self.pos_raw.to_owned(),
                        data: Box::new(new_content),
                    },
                    variable: self.data.variable.to_owned(),
                    flags: self.data.flags.to_owned(),
                };
            }
        };
        Ok(content_type)
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariants, ZyxtError> {
        todo!()
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
