use std::{collections::HashMap, sync::Arc};

use itertools::Itertools;
use smol_str::SmolStr;
use tracing::debug;

use crate::{
    ast::{argument::Argument, Ast, AstData, Block, Declare, Ident, Reconstruct},
    errors::ToZResult,
    primitives::{generic_proc, LazyGenericProc},
    types::{
        position::{GetSpan, Span},
        r#type::{LazyType, TypeCheckType},
        sym_table::TypeCheckFrameType,
        token::Flag,
        value::Proc,
    },
    InterpretSymTable, Type, TypeCheckSymTable, Value, ZResult,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Class {
    Raw {
        is_struct: bool,
        content: Option<Block>,
        args: Option<Vec<Argument>>,
    },
    TypeChecked {
        is_struct: bool,
        namespace: HashMap<SmolStr, Ast>,
        fields: HashMap<SmolStr, Arc<Type>>,
    },
}

impl GetSpan for Class {
    fn span(&self) -> Option<Span> {
        todo!()
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
            } => (is_struct, content, args),
            Self::TypeChecked {
                namespace, fields, ..
            } => {
                return Ok(Arc::new(Type::Type {
                    name: None,
                    namespace: namespace
                        .iter_mut()
                        .map(|(k, v)| {
                            Ok((k.to_owned(), Arc::clone(&*v.type_check(ty_symt)?).into()))
                        })
                        .collect::<ZResult<HashMap<_, _>>>()?,
                    fields: fields.to_owned(),
                    type_args: vec![],
                })
                .into())
            }
        };
        let mut namespace_ast = HashMap::new();
        let mut namespace_ty = HashMap::new();
        let mut fields = HashMap::new();
        let mut new_found = false;

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
                todo!()
            };
            let Ast::Ident(ident) = *dec.variable.to_owned() else {
                todo!()
            };
            if ident.name == "_new" {
                if *is_struct {
                    todo!()
                }
                new_found = true;
            }
            if dec.flags.iter().find(|(k, _)| *k == Flag::Inst).is_some() {
                fields.insert(ident.name, Arc::clone(&*ty));
            } else {
                namespace_ty.insert(ident.name.to_owned(), Arc::clone(&*ty).into());
                namespace_ast.insert(ident.name, dec.content.to_owned());
            }
        }

        let mut empty2 = vec![];
        let args = if let Some(args) = args {
            if new_found {
                todo!()
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

        let mut ty = Arc::new(Type::Type {
            name: None,
            namespace: namespace_ty,
            fields,
            type_args: vec![],
        });

        if !new_found { // todo when `$` is added
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

        ty_symt.pop_frame();
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

    fn interpret_expr(&self, _val_symt: &mut InterpretSymTable) -> ZResult<Value> {
        todo!()
    }
}

impl Reconstruct for Class {
    fn reconstruct(&self) -> String {
        "todo".to_owned()
    }
}
