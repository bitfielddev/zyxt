use std::sync::Arc;

use tracing::debug;

use crate::{
    ast::{Ast, AstData, Condition, Reconstruct},
    errors::ZError,
    types::{
        position::{GetSpan, Span},
        r#type::TypeCheckType,
    },
    InterpretSymTable, TypeCheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct If {
    pub conditions: Vec<Condition>,
}
impl GetSpan for If {
    fn span(&self) -> Option<Span> {
        self.conditions.span()
    }
}

impl AstData for If {
    fn as_variant(&self) -> Ast {
        Ast::If(self.to_owned())
    }

    fn is_pattern(&self) -> bool {
        false
    }
    fn type_check(&mut self, ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        debug!(span = ?self.span(), "Type-checking if expression");
        let mut first_ty: Option<TypeCheckType> = None;
        for ty in &mut self.conditions {
            let ty = ty.if_true.block_type(ty_symt, true)?;
            if let Some(first_ty) = &first_ty {
                if !Arc::ptr_eq(first_ty, &ty) {
                    return Err(ZError::t011(first_ty, &ty));
                }
            } else {
                first_ty = Some(ty)
            }
        }
        self.conditions[0].if_true.block_type(ty_symt, true)
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring if statement");
        Ok(Self {
            conditions: self
                .conditions
                .iter()
                .map(|a| {
                    let mut a = a.to_owned();
                    a.desugar()?;
                    Ok(a)
                })
                .collect::<Result<_, _>>()?,
        }
        .as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        for cond in &self.conditions {
            if cond.condition.is_none()
                || cond
                    .condition
                    .as_ref()
                    .map(|cond| cond.interpret_expr(val_symt))
                    .transpose()?
                    == Some(Value::Bool(true))
            {
                return cond.if_true.interpret_block(val_symt, false, true);
            }
        }
        Ok(Value::Unit)
    }
}

impl Reconstruct for If {
    fn reconstruct(&self) -> String {
        let mut s = String::new();
        for (i, c) in self.conditions.iter().enumerate() {
            s.push_str(if i == 0 {
                "if "
            } else if c.condition.is_none() {
                "else "
            } else {
                "elif "
            });
            if let Some(condition) = &c.condition {
                s.push_str(&condition.reconstruct());
            }
            s.push(' ');
            s.push_str(&c.if_true.reconstruct())
        }
        s
    }
}
