use std::{collections::HashMap, sync::Arc};

use itertools::{Either, Itertools};
use smol_str::SmolStr;
use tracing::debug;

use crate::{
    ast::{Ast, AstData, BinaryOpr, Ident, Literal, Member, Reconstruct},
    primitives::{ANY_T, PROC_T, UNIT_T},
    types::{
        position::{GetSpan, Span},
        r#type::TypeCheckType,
        token::{AccessType, OprType},
    },
    InterpretSymTable, Type, TypeCheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Call {
    pub called: Box<Ast>,
    pub paren_spans: Option<(Span, Span)>,
    pub args: Vec<Ast>,
    pub kwargs: HashMap<SmolStr, Ast>,
}
impl GetSpan for Call {
    fn span(&self) -> Option<Span> {
        let start_paren = self.paren_spans.as_ref().map(|a| &a.0);
        let end_paren = self.paren_spans.as_ref().map(|a| &a.1);
        self.called
            .merge_span(start_paren)
            .merge_span(&self.args)
            .merge_span(end_paren)
    }
}

impl AstData for Call {
    fn as_variant(&self) -> Ast {
        Ast::Call(self.to_owned())
    }
    fn type_check(&mut self, ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        debug!(span = ?self.span(), "Type-checking function call");
        if let Ast::Member(Member { name, parent, .. }) = &*self.called {
            if let Ast::Ident(Ident {
                name: parent_name, ..
            }) = &**parent
            {
                if &**name == "out" && &**parent_name == "ter" {
                    self.args
                        .iter_mut()
                        .map(|a| a.type_check(ty_symt))
                        .collect::<ZResult<Vec<_>>>()?;
                    return Ok(Arc::clone(&UNIT_T).into());
                }
            }
        }
        let called_type = self.called.type_check(ty_symt)?;
        let arg_tys = self
            .args
            .iter_mut()
            .map(|a| a.type_check(ty_symt))
            .collect::<ZResult<Vec<_>>>()?;
        let extract_proc = |ty: &Type| {
            if let Type::Generic { type_args, base } = ty {
                if !Arc::ptr_eq(base, &PROC_T) {
                    None
                } else if let Some((_, sig_arg_tys)) = type_args.iter().find(|(k, _)| *k == "A") {
                    if let Some((_, ret_ty)) = type_args.iter().find(|(k, _)| *k == "R") {
                        let Either::Right(Either::Left(sig_arg_tys)) = sig_arg_tys else {
                        unreachable!()
                    };
                        let Either::Right(Either::Right(ret_ty)) = ret_ty else {
                        unreachable!()
                    };
                        Some((sig_arg_tys.to_owned(), Arc::clone(ret_ty)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        let (sig_arg_tys, ret_ty) = if let Some(res) = extract_proc(&called_type) {
            res
        } else {
            let mut ty = called_type;
            let mut out = None;

            while let Some(f) = ty.namespace().get("_call").cloned() {
                if let Some(res) = extract_proc(&f) {
                    out = Some(res);
                    break;
                }
                ty = Arc::clone(&f).into();
            }
            if let Some(res) = out {
                res
            } else {
                todo!()
            }
        };
        if arg_tys.len() != sig_arg_tys.len() {
            todo!()
        }
        for (arg_ty, sig_arg_ty) in arg_tys.iter().zip(&sig_arg_tys) {
            if !Arc::ptr_eq(arg_ty, sig_arg_ty) && !Arc::ptr_eq(sig_arg_ty, &ANY_T) {
                todo!("{arg_ty:?}, {sig_arg_ty:?}")
            }
        }
        Ok(ret_ty.into())
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring function call");
        let mut called = self.called.desugared()?;
        let mut args = self
            .args
            .iter()
            .map(AstData::desugared)
            .collect::<ZResult<Vec<_>>>()?;
        if let Ast::Member(Member {
            ty: AccessType::Method,
            name,
            parent,
            ..
        }) = called
        {
            called = Ast::Member(Member {
                ty: AccessType::Namespace,
                name,
                parent: Box::new(
                    Ast::BinaryOpr(BinaryOpr {
                        ty: OprType::TypeCast,
                        opr_span: None,
                        operand1: parent.to_owned(),
                        operand2: Box::from(Ast::Ident(Ident::new("type"))),
                    })
                    .desugared()?,
                ),
                name_span: None,
                dot_span: None,
            });
            args.reverse();
            args.push(*parent);
            args.reverse();
        }
        Ok(Ast::Call(Self {
            called: Box::new(called),
            paren_spans: self.paren_spans.to_owned(),
            args,
            kwargs: self
                .kwargs
                .iter()
                .map(|(k, v)| Ok((k.to_owned(), v.desugared()?)))
                .collect::<ZResult<_>>()?,
        }))
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        if let Ast::Member(Member { name, parent, .. }) = &*self.called {
            if let Ast::Ident(Ident {
                name: parent_name, ..
            }) = &**parent
            {
                if &**name == "out" && &**parent_name == "ter" {
                    let s = self
                        .args
                        .iter()
                        .map(|arg| arg.interpret_expr(val_symt))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{s}");
                    return Ok(Value::Unit);
                }
            }
        }
        let Value::Proc(proc) = self.called.interpret_expr(val_symt)? else {
            todo!()
        };
        proc.call(
            self.args
                .iter()
                .map(|a| a.interpret_expr(val_symt))
                .collect::<ZResult<Vec<_>>>()?,
            val_symt,
        )
    }
}

impl Reconstruct for Call {
    fn reconstruct(&self) -> String {
        format!(
            "{} ( {} )",
            self.called.reconstruct(),
            self.args.iter().map(Reconstruct::reconstruct).join(" , ")
        )
    }
}
