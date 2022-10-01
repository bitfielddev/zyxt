use crate::{
    types::{
        element::{call::Call, ident::Ident, Element, ElementData, ElementVariants, PosRaw},
        token::OprType,
        typeobj::bool_t::BOOL_T,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BinaryOpr {
    pub type_: OprType,
    pub operand1: Element,
    pub operand2: Element,
}

impl ElementData for BinaryOpr {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::BinaryOpr(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn desugared(
        &self,
        pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariants, ZyxtError> {
        Ok(match self.type_ {
            OprType::And | OprType::Or => {
                let mut new_self = self.to_owned();
                for operand in [&mut new_self.operand1, &mut new_self.operand2] {
                    *operand = Element {
                        pos_raw: pos_raw.to_owned(),
                        data: Box::new(
                            BinaryOpr {
                                type_: OprType::Typecast,
                                operand1: operand.desugared(out)?,
                                operand2: Element {
                                    pos_raw: pos_raw.to_owned(),
                                    data: Box::new(
                                        BOOL_T
                                            .as_type_element()
                                            .get_instance()
                                            .as_literal()
                                            .data
                                            .as_variant(),
                                    ),
                                },
                            }
                            .as_variant(),
                        ),
                    }
                    .desugared(out)?;
                }
                new_self.as_variant()
            }
            _ => ElementVariants::Call(Call {
                called: Element {
                    pos_raw: pos_raw.to_owned(),
                    data: Box::new(
                        Ident {
                            name: match self.type_ {
                                OprType::Plus => "_add",
                                OprType::Minus => "_sub",
                                OprType::AstMult => "_mul",
                                OprType::FractDiv => "_div",
                                OprType::Modulo => "_rem",
                                OprType::Eq => "_eq",
                                OprType::Noteq => "_ne",
                                OprType::Lt => "_lt",
                                OprType::Lteq => "_le",
                                OprType::Gt => "_gt",
                                OprType::Gteq => "_ge",
                                OprType::Concat => "_concat",
                                OprType::TypeCast => "_typecast",
                                _ => unimplemented!("{:#?}", self.type_),
                            }
                            .into(),
                            parent: Some(self.operand1.desugared(out)?),
                        }
                        .as_variant(),
                    ),
                },
                args: vec![self.operand2.desugared(out)?],
                kwargs: Default::default(),
            })
            .desugared(pos_raw, out),
        })
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
