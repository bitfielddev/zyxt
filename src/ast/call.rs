use std::collections::HashMap;

use itertools::Itertools;
use smol_str::SmolStr;

use crate::{
    ast::{argument::Argument, Ast, AstData, Ident, Literal, Reconstruct},
    errors::ZError,
    primitives::UNIT_T,
    types::{
        position::{GetSpan, Position, Span},
        sym_table::{FrameData, FrameType},
        typeobj::{TypeDefinition, TypeInstance},
        value::Proc,
    },
    SymTable, Type, Value, ZResult,
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
    fn process(&mut self, ty_symt: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
        if let Ast::Ident(Ident {
            name,
            parent:
                Some(box Ast::Ident(Ident {
                    name: parent_name, ..
                })),
            ..
        }) = &*self.called
        {
            if &**name == "out" && &**parent_name == "ter" {
                self.args
                    .iter_mut()
                    .map(|a| a.process(ty_symt))
                    .collect::<ZResult<Vec<_>>>()?;
                return Ok(UNIT_T.get_instance().as_type_element());
            }
        }
        let called_type = self.called.process(ty_symt)?;
        if let Ast::Procedure(procedure) = &mut *self.called {
            for (i, arg) in self.args.iter_mut().enumerate() {
                let expected = procedure.args[i].ty.process(ty_symt)?;
                let actual = arg.process(ty_symt)?;
                if expected != actual {
                    return Err(ZError::t004(&expected, &actual).with_span(&*self));
                }
            }
            if let Some(ty) = &mut procedure.return_type {
                ty.process(ty_symt)
            } else {
                Ok(UNIT_T.as_type_element().as_type())
            }
        } else if let Ast::Literal(Literal {
            content: Value::Proc(proc),
            ..
        }) = &mut *self.called
        {
            Ok(match proc {
                Proc::Builtin { signature, .. } => {
                    let (arg_objs, ret): (Vec<Type<Value>>, Type<Value>) = signature[0]();
                    for (i, arg) in self.args.iter_mut().enumerate() {
                        let actual = arg.process(ty_symt)?;
                        let expected = arg_objs[i].as_type_element();
                        if actual != expected && actual != Type::Any && expected != Type::Any {
                            return Err(ZError::t004(&expected, &actual).with_span(&*self));
                        }
                    }
                    ret.as_type_element()
                }
                Proc::Defined {
                    args: arg_objs,
                    return_type,
                    ..
                } => {
                    for (i, arg) in self.args.iter_mut().enumerate() {
                        let expected = arg_objs[i].ty.process(ty_symt)?;
                        let actual = arg.process(ty_symt)?;
                        if expected != actual {
                            return Err(ZError::t004(&expected, &actual).with_span(&*self));
                        }
                    }
                    return_type.as_type_element()
                }
            })
        } else {
            if let Type::Instance(TypeInstance {
                name, type_args, ..
            }) = &called_type
            {
                if *name == Some(SmolStr::from("proc")) {
                    if let Some(return_type) = type_args.get(1) {
                        return Ok(return_type.to_owned());
                    }
                }
            }
            self.called = if let Type::Definition(TypeDefinition {
                implementations, ..
            }) = &called_type
            {
                if let Some(call) = implementations.get("_call") {
                    call.to_owned()
                } else {
                    return Err(ZError::t005(&called_type, "_call").with_span(&self.called));
                }
            } else {
                unreachable!()
            }
            .into();
            self.process(ty_symt)
        }
    }

    fn desugared(&self) -> ZResult<Ast> {
        // TODO
        Ok(self.as_variant())
    }

    fn interpret_expr(&self, val_symt: &mut SymTable<Value>) -> ZResult<Value> {
        if let Ast::Ident(Ident {
            name,
            parent:
                Some(box Ast::Ident(Ident {
                    name: parent_name, ..
                })),
            ..
        }) = &*self.called
        {
            if *name == "out" && *parent_name == "ter" {
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
        if let Value::Proc(proc) = self.called.interpret_expr(val_symt)? {
            match proc {
                Proc::Builtin { f, .. } => {
                    let processed_args = self
                        .args
                        .iter()
                        .map(|a| a.interpret_expr(val_symt))
                        .collect::<Result<Vec<_>, _>>()?;
                    if let Some(v) = f(&processed_args) {
                        Ok(v)
                    } else {
                        Err(ZError::i001(&processed_args).with_span(self))
                    }
                }
                Proc::Defined {
                    is_fn,
                    args,
                    content,
                    ..
                } => {
                    let mut processed_args = HashMap::new();
                    for (cursor, Argument { name, default, .. }) in args.iter().enumerate() {
                        let input_arg = if self.args.len() > cursor {
                            &self.args[cursor]
                        } else {
                            default.as_ref().unwrap_or_else(|| unreachable!())
                        };
                        processed_args
                            .insert(name.name.to_owned(), input_arg.interpret_expr(val_symt)?);
                    }
                    val_symt
                        .add_frame(
                            Some(FrameData {
                                pos: Position::default(), // TODO
                                raw_call: String::default(),
                                args: processed_args.to_owned(),
                            }),
                            if is_fn {
                                FrameType::Function
                            } else {
                                FrameType::Normal
                            },
                        )
                        .heap
                        .extend(processed_args);
                    let res = content.interpret_block(val_symt, true, false);
                    val_symt.pop_frame()?;
                    res
                }
            }
        } else {
            panic!()
        }
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
