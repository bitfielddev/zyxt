use std::{collections::HashMap, sync::Arc};

use tracing::debug;

use crate::{
    ast::{Ast, AstData, Call, Literal, Member, Reconstruct},
    primitives::{BOOL_T, BOOL_T_VAL, TYPE_T},
    types::{
        position::{GetSpan, Span},
        r#type::{Type, TypeCheckType},
        sym_table::TypeCheckSymTable,
        token::{AccessType, OprType},
        value::Proc,
    },
    InterpretSymTable, Value, ZResult,
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

    fn type_check(&mut self, ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        debug!(span = ?self.span(), "Type-checking binary operator");
        let ty1 = self.operand1.type_check(ty_symt)?;
        let ty2 = self.operand2.type_check(ty_symt)?;
        match self.ty {
            OprType::And | OprType::Or => Ok(Arc::clone(&BOOL_T).into()),
            OprType::TypeCast => Ok({
                let ty2 = ty2.as_const()?;
                if Arc::ptr_eq(ty2, &TYPE_T) {
                    TypeCheckType::Const(Arc::clone(&ty1))
                } else {
                    Arc::clone(ty2).into()
                }
            }),
            _ => unreachable!(),
        }
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
                        operand2: Box::new(Value::Type(Arc::clone(&BOOL_T_VAL)).as_ast()),
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

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        let operand1 = self.operand1.interpret_expr(val_symt)?;
        let operand2 = self.operand2.interpret_expr(val_symt)?;
        match self.ty {
            OprType::And => {
                if let Value::Bool(b) = operand1 {
                    if b {
                        if let Value::Bool(b) = operand2 {
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
                if let Value::Bool(b) = operand1 {
                    if b {
                        Ok(Value::Bool(true))
                    } else if let Value::Bool(b) = operand2 {
                        Ok(Value::Bool(b))
                    } else {
                        panic!()
                    }
                } else {
                    panic!()
                }
            }
            OprType::TypeCast => {
                let opr1_ty = Arc::clone(&operand1.value_ty());
                let namespace = opr1_ty.namespace();
                let Some(Value::Proc(proc)) = namespace
                    .get("_typecast") else {
                    todo!()
                };
                proc.call(vec![operand1, operand2], val_symt)
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
