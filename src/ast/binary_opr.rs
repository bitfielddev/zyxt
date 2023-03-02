use std::collections::HashMap;

use tracing::debug;

use crate::{
    ast::{Ast, AstData, Call, Member, Reconstruct},
    primitives::BOOL_T,
    types::{
        position::{GetSpan, Span},
        token::{AccessType, OprType},
    },
    SymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct BinaryOpr {
    pub ty: OprType,
    pub opr_span: Option<Span>,
    pub operand1: Box<Ast>,
    pub operand2: Box<Ast>,
}
impl GetSpan for BinaryOpr {
    fn span(&self) -> Option<Span> {
        self.operand1
            .merge_span(&self.opr_span)
            .merge_span(&self.operand2)
    }
}

impl AstData for BinaryOpr {
    fn as_variant(&self) -> Ast {
        Ast::BinaryOpr(self.to_owned())
    }

    fn desugared(&self) -> ZResult<Ast> {
        Ok(match self.ty {
            OprType::And | OprType::Or => {
                debug!(span = ?self.span(), "Desugaring && / || operator");
                let mut new_self = self.to_owned();
                for operand in [&mut new_self.operand1, &mut new_self.operand2] {
                    *operand = Self {
                        ty: OprType::TypeCast,
                        opr_span: self.opr_span.to_owned(),
                        operand1: operand.desugared()?.into(),
                        operand2: BOOL_T
                            .as_type_element()
                            .get_instance()
                            .as_literal()
                            .as_variant()
                            .into(),
                    }
                    .desugared()?
                    .into();
                }
                new_self.as_variant()
            }
            OprType::TypeCast => {
                debug!(span = ?self.span(), "Desugaring @ operator");
                let mut new_self = self.to_owned();
                new_self.operand1.desugar()?;
                new_self.operand2.desugar()?;
                new_self.as_variant()
            }
            _ => {
                debug!(span = ?self.span(), "Desugaring miscellaneous binary operator");
                Call {
                    called: Member {
                        ty: AccessType::Method,
                        name: match self.ty {
                            OprType::Add => "_add",
                            OprType::Sub => "_sub",
                            OprType::Mul => "_mul",
                            OprType::Div => "_div",
                            OprType::Mod => "_rem",
                            OprType::Eq => "_eq",
                            OprType::Ne => "_ne",
                            OprType::Lt => "_lt",
                            OprType::Le => "_le",
                            OprType::Gt => "_gt",
                            OprType::Ge => "_ge",
                            OprType::Concat => "_concat",
                            _ => unimplemented!("{:#?}", self.ty),
                        }
                        .into(),
                        name_span: None,
                        dot_span: None,
                        parent: self.operand1.desugared()?.into(),
                    }
                    .desugared()?
                    .into(),
                    paren_spans: None,
                    args: vec![self.operand2.desugared()?],
                    kwargs: HashMap::default(),
                }
                .desugared()?
            }
        })
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        match self.ty {
            OprType::And => {
                if let Value::Bool(b) = self.operand1.interpret_expr(val_symt)? {
                    if b {
                        if let Value::Bool(b) = self.operand2.interpret_expr(val_symt)? {
                            Ok(Value::Bool(b))
                        } else {
                            panic!()
                        }
                    } else {
                        Ok(Value::Bool(false))
                    }
                } else {
                    panic!()
                }
            }
            OprType::Or => {
                if let Value::Bool(b) = self.operand1.interpret_expr(val_symt)? {
                    if b {
                        Ok(Value::Bool(true))
                    } else if let Value::Bool(b) = self.operand2.interpret_expr(val_symt)? {
                        Ok(Value::Bool(b))
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            _opr => panic!("{_opr:?}"),
        }
    }
}

impl Reconstruct for BinaryOpr {
    fn reconstruct(&self) -> String {
        format!(
            "{} <{}> {}",
            self.operand1.reconstruct(),
            self.ty,
            self.operand2.reconstruct()
        )
    }
}
