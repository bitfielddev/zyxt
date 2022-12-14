use std::collections::HashMap;

use smol_str::SmolStr;

use crate::{
    ast::{argument::Argument, Ast, AstData, Ident, Literal},
    primitives::UNIT_T,
    types::{
        interpreter_data::{FrameData, FrameType},
        position::{GetSpan, Position, Span},
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
    fn process(&mut self, typelist: &mut SymTable<Type<Ast>>) -> ZResult<Type<Ast>> {
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
                    .map(|a| a.process(typelist))
                    .collect::<ZResult<Vec<_>>>()?;
                return Ok(UNIT_T.get_instance().as_type_element());
            }
        }
        let called_type = self.called.process(typelist)?;
        if let Ast::Procedure(procedure) = &mut *self.called {
            for (i, arg) in self.args.iter_mut().enumerate() {
                if arg.process(typelist)? != procedure.args[i].ty.process(typelist)? {
                    todo!("errors")
                }
            }
            if let Some(ty) = &mut procedure.return_type {
                ty.process(typelist)
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
                        let arg = arg.process(typelist)?;
                        let arg_req = arg_objs[i].as_type_element();
                        if arg != arg_req && arg != Type::Any && arg_req != Type::Any {
                            todo!("{:#?} != {:#?}", arg, arg_req)
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
                        if arg.process(typelist)? != arg_objs[i].ty.process(typelist)? {
                            todo!("errors")
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
            }) = called_type
            {
                if let Some(call) = implementations.get("_call") {
                    call.to_owned()
                } else {
                    todo!();
                }
                // TODO handle error
            } else {
                unreachable!()
            }
            .into();
            self.process(typelist)
        }
    }

    fn desugared(&self) -> ZResult<Ast> {
        // TODO
        Ok(self.as_variant())
    }

    fn interpret_expr(&self, i_data: &mut SymTable<Value>) -> ZResult<Value> {
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
                    .map(|arg| arg.interpret_expr(i_data))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                println!("{s}");
                return Ok(Value::Unit);
            }
        }
        if let Value::Proc(proc) = self.called.interpret_expr(i_data)? {
            match proc {
                Proc::Builtin { f, .. } => {
                    let processed_args = self
                        .args
                        .iter()
                        .map(|a| a.interpret_expr(i_data))
                        .collect::<Result<Vec<_>, _>>()?;
                    if let Some(v) = f(&processed_args) {
                        Ok(v)
                    } else {
                        todo!()
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
                            .insert(name.name.to_owned(), input_arg.interpret_expr(i_data)?);
                    }
                    i_data
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
                    let res = content.interpret_block(i_data, true, false);
                    i_data.pop_frame()?;
                    res
                }
            }
        } else {
            panic!()
        }
    }
}