use std::{collections::HashMap, sync::Arc};

use itertools::Itertools;
use smol_str::SmolStr;
use tracing::debug;

use crate::{
    ast::{argument::Argument, Ast, AstData, Block, Reconstruct},
    errors::{ToZResult, ZError},
    types::{
        position::{GetSpan, Span},
        r#type::{TypeCheckType, ValueType},
        sym_table::TypeCheckFrameType,
        token::Flag,
    },
    InterpretSymTable, Type, TypeCheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Class {
    Raw {
        is_struct: bool,
        kwd_span: Option<Span>,
        content: Option<Block>,
        args: Option<Vec<Argument>>,
    },
    TypeChecked {
        is_struct: bool,
        span: Option<Span>,
        reconstruction: String,
        namespace: HashMap<SmolStr, Ast>,
        fields: HashMap<SmolStr, Arc<Type>>,
    },
}

impl GetSpan for Class {
    fn span(&self) -> Option<Span> {
        match self {
            Self::Raw {
                kwd_span,
                content,
                args,
                ..
            } => kwd_span.merge_span(content).merge_span(args),
            Self::TypeChecked { span, .. } => span.to_owned(),
        }
    }
}

impl AstData for Class {
    fn as_variant(&self) -> Ast {
        Ast::Class(self.to_owned())
    }

    fn type_check(&mut self, ty_symt: &mut TypeCheckSymTable) -> ZResult<TypeCheckType> {
        debug!(span = ?self.span(), "Type-checking class declaration");
        let (is_struct, content, args) = match self {
            Self::Raw {
                is_struct,
                content,
                args,
                ..
            } => (is_struct, content, args),
            Self::TypeChecked {
                namespace, fields, ..
            } => {
                let mut ty_symt = ty_symt.to_owned();
                return Ok(Arc::new(Type::Type {
                    name: None,
                    namespace: namespace
                        .iter_mut()
                        .map(|(k, v)| {
                            Ok((
                                k.to_owned(),
                                Arc::clone(&*v.type_check(&mut ty_symt)?).into(),
                            ))
                        })
                        .collect::<ZResult<HashMap<_, _>>>()?,
                    fields: fields.to_owned(),
                    type_args: vec![],
                })
                .into());
            }
        };
        let mut namespace_ast = HashMap::new();
        let mut namespace_ty = HashMap::new();
        let mut fields = HashMap::new();
        let mut new_span = None;

        ty_symt.add_frame(TypeCheckFrameType::Function(None));

        let mut empty = vec![];
        let statements = if let Some(content) = content {
            &mut content.content
        } else {
            &mut empty
        };
        for statement in statements {
            let ty = statement.type_check(ty_symt)?;
            let Ast::Declare(dec) = statement else {
                return Err(ZError::t013().with_span(&*statement))
            };
            let Ast::Ident(ident) = *dec.variable.to_owned() else {
                return Err(ZError::t008().with_span(&dec.variable))
            };
            if ident.name == "_new" {
                if *is_struct {
                    return Err(ZError::t014().with_span(ident));
                }
                new_span = Some(ident.span());
            }
            if dec.flags.iter().any(|(k, _)| *k == Flag::Inst) {
                fields.insert(ident.name, Arc::clone(&*ty));
            } else {
                namespace_ty.insert(ident.name.to_owned(), Arc::clone(&*ty).into());
                namespace_ast.insert(ident.name, *dec.content.to_owned());
            }
        }

        let mut empty2 = vec![];
        let args = if let Some(args) = args {
            if let Some(new_span) = new_span {
                return Err(ZError::t012().with_span(new_span));
            }
            args
        } else {
            &mut empty2
        };
        let _arg_tys = args
            .iter_mut()
            .map(|arg| {
                let arg_ty = arg.type_check(ty_symt)?;
                fields.insert(arg.name.name.to_owned(), Arc::clone(&arg_ty));
                Ok(arg_ty)
            })
            .collect::<ZResult<Vec<_>>>()?;

        let ty = Arc::new(Type::Type {
            name: None,
            namespace: namespace_ty,
            fields: fields.to_owned(),
            type_args: vec![],
        });

        if new_span.is_none() { // todo when `$` is added
             /*let Some(Type::Type { namespace, ..}) = Arc::get_mut(&mut ty) else {
                 unreachable!()
             };
             namespace.insert("_new".into(), generic_proc(arg_tys, Arc::clone(&ty)).into());
             namespace_ast.insert("_new".into(), Value::Proc(Proc::Defined {
                 is_fn: false,
                 content: Block { brace_spans: None, content: vec![

                 ] },
                 args: vec![],
             }).as_ast().into());*/
        }

        *self = Self::TypeChecked {
            is_struct: *is_struct,
            reconstruction: self.reconstruct(),
            span: self.span(),
            namespace: namespace_ast,
            fields,
        };

        ty_symt.pop_frame()?;
        Ok(ty.into())
    }

    fn desugared(&self) -> ZResult<Ast> {
        debug!(span = ?self.span(), "Desugaring class");
        let mut new_self = self.to_owned();
        match &mut new_self {
            Self::Raw { content, args, .. } => {
                if let Some(content) = content {
                    *content = content.desugared()?.into_block().z()?;
                }
                if let Some(args) = args {
                    for arg in args {
                        arg.desugar()?;
                    }
                }
            }
            Self::TypeChecked { namespace, .. } => {
                for ast in namespace.values_mut() {
                    ast.desugar()?;
                }
            }
        }
        Ok(new_self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        let Self::TypeChecked { namespace, fields, .. } = self else {
            unreachable!()
        };
        let namespace = namespace
            .iter()
            .map(|(k, v)| Ok((k.to_owned(), v.interpret_expr(val_symt)?)))
            .collect::<ZResult<HashMap<_, _>>>()?;
        Ok(Value::Type(Arc::new(ValueType::Type {
            name: None,
            namespace,
            fields: fields.to_owned(),
            type_args: vec![],
        })))
    }
}

impl Reconstruct for Class {
    fn reconstruct(&self) -> String {
        let (is_struct, content, args) = match self {
            Self::TypeChecked { reconstruction, .. } => return reconstruction.to_owned(),
            Self::Raw {
                is_struct,
                content,
                args,
                ..
            } => (is_struct, content, args),
        };
        let mut s = String::new();
        s.push_str(if *is_struct { "struct" } else { "class" });
        if let Some(args) = args {
            s.push('|');
            s.push_str(&args.iter().map(Reconstruct::reconstruct).join(", "));
            s.push('|');
        }
        if let Some(content) = content {
            s.push(' ');
            s.push_str(&content.reconstruct());
        }
        s
    }
}
