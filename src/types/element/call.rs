use std::collections::HashMap;

use smol_str::SmolStr;

use crate::{
    types::{
        element::{
            ident::Ident, literal::Literal, procedure::Argument, Element, ElementData,
            ElementVariant,
        },
        interpreter_data::{FrameData, FrameType},
        position::PosRaw,
        typeobj::{unit_t::UNIT_T, TypeDefinition, TypeInstance},
        value::Proc,
    },
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Call {
    pub called: Element,
    pub args: Vec<Element>,
    pub kwargs: HashMap<SmolStr, Element>,
}

impl ElementData for Call {
    fn as_variant(&self) -> ElementVariant {
        ElementVariant::Call(self.to_owned())
    }
    fn process<O: Print>(
        &mut self,
        _pos_raw: &PosRaw,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        if let ElementVariant::Ident(Ident {
            name,
            parent:
                Some(Element {
                    data:
                        ElementVariant::Ident(Ident {
                            name: parent_name, ..
                        }),
                    ..
                }),
        }) = &self.called.data
        {
            if &**name == "out" && &**parent_name == "ter" {
                self.args
                    .iter_mut()
                    .map(|a| a.process(typelist))
                    .collect::<Result<Vec<_>, ZyxtError>>()?;
                return Ok(UNIT_T.get_instance().as_type_element());
            }
        }
        let called_type = self.called.process(typelist)?;
        if let ElementVariant::Procedure(procedure) = self.called.data.as_mut() {
            for (i, arg) in self.args.iter_mut().enumerate() {
                if arg.process(typelist)?
                    != procedure.args.get_mut(i).unwrap().ty.process(typelist)?
                {
                    todo!("errors")
                }
            }
            procedure.return_type.process(typelist)
        } else if let ElementVariant::Literal(Literal {
            content: Value::Proc(proc),
        }) = self.called.data.as_mut()
        {
            Ok(match proc {
                Proc::Builtin { signature, .. } => {
                    let (mut arg_objs, ret): (Vec<Type<Value>>, Type<Value>) = signature[0]();
                    for (i, arg) in self.args.iter_mut().enumerate() {
                        let arg = arg.process(typelist)?;
                        let arg_req = arg_objs.get_mut(i).unwrap().as_type_element();
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
                        if arg.process(typelist)?
                            != arg_objs.get_mut(i).unwrap().ty.process(typelist)?
                        {
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
            };
            self.process(typelist)
        }
    }

    fn desugared(
        &self,
        _pos_raw: &PosRaw,
        out: &mut impl Print,
    ) -> Result<ElementVariant, ZyxtError> {
        todo!()
    }

    fn interpret_expr<O: Print>(
        &self,
        i_data: &mut InterpreterData<Value, O>,
    ) -> Result<Value, ZyxtError> {
        if let ElementVariant::Ident(Ident {
            name,
            parent:
                Some(Element {
                    data:
                        ElementVariant::Ident(Ident {
                            name: parent_name, ..
                        }),
                    ..
                }),
        }) = &self.called.data
        {
            if *name == "out" && parent_name == *"ter" {
                let s = self
                    .args
                    .iter()
                    .map(|arg| arg.interpret_expr(i_data))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<String>>()
                    .join(" ");
                i_data.out.println(s);
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
                            self.args.get(cursor).unwrap()
                        } else {
                            default.as_ref().unwrap()
                        };
                        processed_args.insert(name.to_owned(), input_arg.interpret_expr(i_data)?);
                    }
                    i_data
                        .add_frame(
                            Some(FrameData {
                                position: Default::default(), // TODO
                                raw_call: Default::default(),
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
                    let res = content.data.interpret_block(i_data, true, false);
                    i_data.pop_frame()?;
                    res
                }
            }
        } else {
            panic!()
        }
    }
}
