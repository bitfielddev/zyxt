use smol_str::SmolStr;

use crate::{
    types::{
        element::{call::Call, ident::Ident, Element, ElementData, ElementVariants, PosRaw},
        token::OprType,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UnaryOpr {
    type_: OprType,
    operand: Element,
}

impl ElementData for UnaryOpr {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::UnaryOpr(self.to_owned())
    }

    fn desugared(
        &self,
        pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariants, ZyxtError> {
        Ok(Call {
            called: Element {
                pos_raw: pos_raw.to_owned(),
                data: Box::new(
                    Ident {
                        name: match self.type_ {
                            OprType::Not => "_not",
                            OprType::PlusSign => "_un_plus",
                            OprType::MinusSign => "_un_minus",
                            _ => panic!(),
                        }
                        .into(),
                        parent: Some(self.operand.desugared(out)?),
                    }
                    .as_variant(),
                ),
            },
            args: vec![],
            kwargs: Default::default(),
        }
        .as_variant())
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        todo!()
    }
}
