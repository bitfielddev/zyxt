use std::sync::Arc;

use itertools::Either;
use tracing::debug;

use crate::{
    ast::{argument::Argument, Ast, AstData, Block, Reconstruct},
    primitives::{generic_proc, PROC_T, UNIT_T},
    types::{
        position::{GetSpan, Span},
        sym_table::TypecheckFrameType,
        value::Proc,
    },
    InterpretSymTable, Type, TypecheckSymTable, Value, ZError, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Procedure {
    pub is_fn: bool,
    pub kwd_span: Option<Span>,
    pub args: Vec<Argument>,
    pub return_type: Option<Box<Ast>>,
    pub content: Block,
}
impl GetSpan for Procedure {
    fn span(&self) -> Option<Span> {
        self.kwd_span
            .merge_span(&self.args)
            .merge_span(&self.return_type)
            .merge_span(&self.content)
    }
}

impl AstData for Procedure {
    fn as_variant(&self) -> Ast {
        Ast::Procedure(self.to_owned())
    }

    fn typecheck(&mut self, ty_symt: &mut TypecheckSymTable) -> ZResult<Arc<Type>> {
        let sig_ret_ty = if let Some(ty) = &mut self.return_type {
            let Ast::Ident(i) = &**ty else {
                todo!()
            };
            Some(ty_symt.get_type(&i.name, ty.span())?)
        } else {
            None
        };
        ty_symt.add_frame(if self.is_fn {
            TypecheckFrameType::Function
        } else {
            TypecheckFrameType::Normal
        }(sig_ret_ty.map(|a| Arc::clone(&a))));
        for arg in &mut self.args {
            let ty = ty_symt.get_type_from_ident(&arg.ty, arg.ty.span())?;
            ty_symt.declare_val(&arg.name.name, ty);
        }
        let res = self.content.block_type(ty_symt, false)?;
        let (TypecheckFrameType::Function(ret_ty) | TypecheckFrameType::Normal(ret_ty)) = &ty_symt.0.front().unwrap().ty else {
            unreachable!()
        };
        let ret_ty = Arc::clone(if let Some(ret_ty) = ret_ty {
            if *ret_ty != res {
                todo!("error")
            }
            ret_ty
        } else {
            &res
        });
        ty_symt.pop_frame();
        Ok(generic_proc(vec![], ret_ty))
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring procedure statement");
        let mut new_self = self.to_owned();
        new_self.args = self
            .args
            .iter()
            .map(|a| {
                let mut a = a.to_owned();
                a.desugar()?;
                Ok(a)
            })
            .collect::<Result<Vec<_>, _>>()?;
        new_self.content = self
            .content
            .desugared()?
            .as_block()
            .unwrap_or_else(|| unreachable!())
            .to_owned();
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        Ok(Value::Proc(Proc::Defined {
            is_fn: self.is_fn,
            content: self.content.to_owned(),
        }))
    }
}
impl Reconstruct for Procedure {
    fn reconstruct(&self) -> String {
        format!("todo")
    }
}
