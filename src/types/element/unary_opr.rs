use crate::{
    types::{
        element::{call::Call, ident::Ident, Element, ElementData, ElementVariant},
        position::PosRaw,
        token::OprType,
    },
    Print, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct UnaryOpr {
    pub ty: OprType,
    pub operand: Element,
}

impl ElementData for UnaryOpr {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::UnaryOpr(self.to_owned())
    }

    fn desugared(&self, pos_raw: &PosRaw, out: &mut impl Print) -> ZResult<ElementVariant> {
        Ok(Call {
            called: Element {
                pos_raw: pos_raw.to_owned(),
                data: Box::new(
                    Ident {
                        name: match self.ty {
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
}
