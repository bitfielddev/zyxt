use std::collections::HashMap;

use tracing::debug;

use crate::{
    ast::{Ast, AstData, Call, Member, Reconstruct},
    types::{
        position::{GetSpan, Span},
        token::{AccessType, OprType},
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
        debug!(span = ?self.span(), "Desugaring unary operator");
        Ok(Call {
            called: Member {
                ty: AccessType::Method,
                name: match self.ty {
                    OprType::Not => "_not",
                    OprType::UnPlus => "_un_plus",
                    OprType::UnMinus => "_un_minus",
                    _ => panic!(),
                }
                .into(),
                name_span: None,
                dot_span: None,
                parent: self.operand.desugared()?.into(),
            }
            .desugared()?
            .into(),
            paren_spans: None,
            args: vec![],
            kwargs: HashMap::default(),
        }
        .as_variant())
    }
}

impl Reconstruct for UnaryOpr {
    fn reconstruct(&self) -> String {
        format!("({}) {}", self.ty, self.operand.reconstruct())
    }
}
