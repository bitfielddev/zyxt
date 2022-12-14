use crate::{
    ast::{Ast, AstData, Call, Ident},
    types::{
        position::{GetSpan, Span},
        token::OprType,
    },
    ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct UnaryOpr {
    pub ty: OprType,
    pub opr_span: Option<Span>,
    pub operand: Box<Ast>,
}
impl GetSpan for UnaryOpr {
    fn span(&self) -> Option<Span> {
        self.opr_span.merge_span(&self.operand)
    }
}

impl AstData for UnaryOpr {
    fn as_variant(&self) -> Ast {
        Ast::UnaryOpr(self.to_owned())
    }

    fn desugared(&self) -> ZResult<Ast> {
        Ok(Call {
            called: Ident {
                name: match self.ty {
                    OprType::Not => "_not",
                    OprType::UnPlus => "_un_plus",
                    OprType::UnMinus => "_un_minus",
                    _ => panic!(),
                }
                .into(),
                name_span: None,
                dot_span: None,
                parent: Some(self.operand.desugared()?.into()),
            }
            .as_variant()
            .into(),
            paren_spans: None,
            args: vec![],
            kwargs: Default::default(),
        }
        .as_variant())
    }
}
