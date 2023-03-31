use std::sync::Arc;

use tracing::debug;

use crate::{
    ast::{Ast, AstData, BinaryOpr, Reconstruct},
    errors::ToZResult,
    types::{
        position::{GetSpan, Span},
        r#type::{Type, TypeCheckType},
        token::{Flag, OprType},
    },
    InterpretSymTable, TypeCheckSymTable, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Declare {
    pub variable: Box<Ast>,
    pub content: Box<Ast>,
    pub flags: Vec<(Flag, Span)>,
    pub ty: Option<Box<Ast>>,
    pub eq_span: Option<Span>,
}
impl GetSpan for Declare {
    fn span(&self) -> Option<Span> {
        self.variable
            .merge_span(self.flags.iter().map(|a| &a.1).collect::<Vec<_>>())
            .merge_span(&self.ty)
            .merge_span(&self.content)
            .merge_span(&self.eq_span)
    }
}

impl AstData for Declare {
    fn as_variant(&self) -> Ast {
        Ast::Declare(self.to_owned())
    }

    fn type_check(&mut self, ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        debug!(span = ?self.span(), "Type-checking declaration");
        if !self.variable.is_pattern() {
            return Err(ZError::t006().with_span(&self.variable));
        }
        let mut content_type = self.content.type_check(ty_symt)?;
        let ty = self
            .ty
            .as_ref()
            .map(|ty| {
                let Ast::Ident(i) = &**ty else {
                return Err(ZError::t008().with_span(&self.ty))
            };
                ty_symt.get_type(&i.name, ty.span())
            })
            .transpose()?;
        let name = if let Ast::Ident(ident) = &*self.variable {
            ident.name.to_owned()
        } else {
            return Err(ZError::t008().with_span(&self.variable));
        };
        if let Some(ty) = ty {
            if *content_type != ty {
                let mut new_content = BinaryOpr {
                    ty: OprType::TypeCast,
                    opr_span: None,
                    operand1: self.content.to_owned(),
                    operand2: self.ty.to_owned().unwrap_or_else(|| unreachable!()),
                }
                .as_variant();
                new_content.type_check(ty_symt)?;
                *self = Self {
                    ty: self.ty.to_owned(),
                    content: new_content.into(),
                    variable: self.variable.to_owned(),
                    flags: self.flags.to_owned(),
                    eq_span: self.eq_span.to_owned(),
                };
            }
        }
        if let Ok(ty) = content_type.as_const_mut() {
            ty.update_name(self.variable.as_ident().z()?.to_owned())?;
        }
        ty_symt.declare_val(&name, content_type.to_owned())?;
        Ok(content_type)
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring declaration");
        let mut new_self = self.to_owned();
        new_self.content.desugar()?;
        new_self.variable.desugar()?;
        new_self.ty = self
            .ty
            .as_ref()
            .map(|a| a.desugared())
            .transpose()?
            .map(Into::into);
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        let name = if let Ast::Ident(ident) = &*self.variable {
            &ident.name
        } else {
            unreachable!()
        };
        let var = self.content.interpret_expr(val_symt)?;
        val_symt.declare_val(name, var.to_owned());
        Ok(var)
    }
}
impl Reconstruct for Declare {
    fn reconstruct(&self) -> String {
        if let Some(ty) = &self.ty {
            format!(
                "{}: {} := {}",
                self.variable.reconstruct(),
                ty.reconstruct(),
                self.content.reconstruct()
            )
        } else {
            format!(
                "{} := {}",
                self.variable.reconstruct(),
                self.content.reconstruct()
            )
        }
    }
}
