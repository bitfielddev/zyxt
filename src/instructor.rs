use std::collections::HashMap;

use smol_str::SmolStr;

use crate::{
    interpreter::interpret_block,
    types::{
        element::Element,
        interpreter_data::{FrameType, InterpreterData},
        printer::Print,
        token::{Flag, OprType, Token},
        typeobj::{
            bool_t::BOOL_T, proc_t::PROC_T, type_t::TYPE_T, unit_t::UNIT_T, Type, TypeDefinition,
            TypeInstance,
        },
        value::Proc,
    },
    Value, ZyxtError,
};

pub fn gen_instructions<O: Print>(
    mut input: Vec<Element>,
    typelist: &mut InterpreterData<Type<Element>, O>,
) -> Result<Vec<Element>, ZyxtError> {
    for ele in input.iter_mut() {
        ele.process(typelist)?;
    }
    Ok(input)
}

pub trait Process {
    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError>;
}

impl Process for Element {
    fn process<O: Print>(
        &mut self,
        typelist: &mut InterpreterData<Type<Element>, O>,
    ) -> Result<Type<Element>, ZyxtError> {
        match self {
            Element::Literal { content, .. } => Ok(content.get_type_obj().as_type_element()),
            Element::Ident {
                name,
                position,
                raw,
                ..
            } => typelist.get_val(name, pos_raw),
            Element::Block { content, .. } => Ok(Element::block_type(content, typelist, true)?.0),
            Element::Call { called, args, .. } => {
                if let Element::Ident { name, parent, .. } = called.as_ref() {
                    if let Element::Ident {
                        name: parent_name, ..
                    } = parent.as_ref()
                    {
                        if &**name == "out" && &**parent_name == "ter" {
                            args.iter_mut()
                                .map(|a| a.process(typelist))
                                .collect::<Result<Vec<_>, ZyxtError>>()?;
                            return Ok(UNIT_T.get_instance().as_type_element());
                        }
                    }
                }
                let called_type = called.process(typelist)?;
                if let Element::Procedure {
                    args: args_objs,
                    return_type: pre_return_type,
                    ..
                } = called.as_mut()
                {
                    for (i, arg) in args.iter_mut().enumerate() {
                        if arg.process(typelist)?
                            != args_objs.get_mut(i).unwrap().type_.process(typelist)?
                        {
                            todo!("errors")
                        }
                    }
                    pre_return_type.process(typelist)
                } else if let Element::Literal {
                    content: Value::Proc(proc),
                    ..
                } = called.as_mut()
                {
                    Ok(match proc {
                        Proc::Builtin { signature, .. } => {
                            let (mut arg_objs, ret): (Vec<Type<Value>>, Type<Value>) =
                                signature[0]();
                            for (i, arg) in args.iter_mut().enumerate() {
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
                            for (i, arg) in args.iter_mut().enumerate() {
                                if arg.process(typelist)?
                                    != arg_objs.get_mut(i).unwrap().type_.process(typelist)?
                                {
                                    todo!("errors")
                                }
                            }
                            return_type.as_type_element()
                        }
                    })
                } else {
                    if let Type::Instance(TypeInstance {
                        ref name,
                        ref type_args,
                        ..
                    }) = called_type
                    {
                        if *name == Some(SmolStr::from("proc")) {
                            if let Some(return_type) = type_args.get(1) {
                                return Ok(return_type.to_owned());
                            }
                        }
                    }
                    *called = if let Type::Definition(TypeDefinition {
                        implementations, ..
                    }) = called_type
                    {
                        if let Some(call) = implementations.get("_call") {
                            Box::new(call.to_owned())
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
            Element::Declare {
                position,
                variable,
                content,
                flags,
                type_: pre_type,
                raw,
            } => {
                if !variable.is_pattern() {
                    return Err(
                        ZyxtError::error_2_2(*variable.to_owned()).with_element(&**variable)
                    );
                }
                let content_type = content.process(typelist)?;
                let type_ = if let Element::Literal { content, .. } = pre_type.as_ref() {
                    if let Value::Type(t) = content {
                        t
                    } else {
                        todo!()
                    }
                } else {
                    todo!()
                }
                .as_type_element();
                if type_ == Type::Any {
                    typelist.declare_val(&variable.get_name(), &content_type);
                } else {
                    typelist.declare_val(&variable.get_name(), &type_);
                    if content_type != type_ {
                        let mut new_content = Element::BinaryOpr {
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            type_: OprType::TypeCast,
                            operand1: content.to_owned(),
                            operand2: Box::new(type_.as_literal()),
                        };
                        new_content.process(typelist)?;
                        *self = Element::Declare {
                            type_: pre_type.to_owned(),
                            content: Box::new(new_content),
                            variable: variable.to_owned(),
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            flags: flags.to_owned(),
                        };
                    }
                };
                Ok(content_type)
            }
            Element::If { conditions, .. } => {
                Ok(Element::block_type(&mut conditions[0].if_true, typelist, true)?.0)
            } // TODO consider all returns
            Element::BinaryOpr {
                type_,
                operand1,
                operand2,
                position,
                raw,
                ..
            } => {
                let type1 = operand1.process(typelist)?;
                let type2 = operand2.process(typelist)?;
                Ok(match type_ {
                    OprType::TypeCast => {
                        if type2 == TYPE_T.as_type().as_type_element() {
                            *self = Element::Call {
                                position: Default::default(),
                                raw: "".to_string(),
                                called: Box::new(
                                    if let Type::Definition(TypeDefinition {
                                        implementations,
                                        ..
                                    }) = type1
                                    {
                                        implementations.get("_typecast").unwrap().to_owned()
                                    // TODO handle error
                                    } else {
                                        unreachable!()
                                    },
                                ),
                                args: vec![*operand1.to_owned(), *operand2.to_owned()],
                                kwargs: Default::default(),
                            };
                            if let Type::Definition(def) = type2 {
                                def.get_instance()
                            } else {
                                todo!()
                            }
                        } else {
                            todo!("Error here")
                        }
                    }
                    OprType::And | OprType::Or => {
                        for (type_, operand) in [(&type1, operand1), (&type2, operand2)] {
                            if *type_ != BOOL_T.as_type().as_type_element() {
                                *operand = Box::new(Element::Call {
                                    position: Default::default(),
                                    raw: "".to_string(),
                                    called: Box::new(
                                        if let Type::Definition(TypeDefinition {
                                            ref implementations,
                                            ..
                                        }) = type_
                                        {
                                            implementations.get("_typecast").unwrap().to_owned()
                                        // TODO handle error
                                        } else {
                                            unreachable!()
                                        },
                                    ),
                                    args: vec![
                                        *operand.to_owned(),
                                        BOOL_T.as_type().as_type_element().as_literal(),
                                    ],
                                    kwargs: Default::default(),
                                });
                            }
                        }
                        BOOL_T.get_instance().as_type_element()
                    }
                    _ => {
                        *self = Element::Call {
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            called: Box::new(
                                if let Type::Definition(TypeDefinition {
                                    ref implementations,
                                    ..
                                }) = type1
                                {
                                    implementations
                                        .get(match type_ {
                                            OprType::Plus => "_add",
                                            OprType::Minus => "_sub",
                                            OprType::AstMult => "_mul",
                                            OprType::FractDiv => "_div",
                                            OprType::Modulo => "_rem",
                                            OprType::Eq => "_eq",
                                            OprType::Noteq => "_ne",
                                            OprType::Lt => "_lt",
                                            OprType::Lteq => "_le",
                                            OprType::Gt => "_gt",
                                            OprType::Gteq => "_ge",
                                            OprType::Concat => "_concat",
                                            _ => unimplemented!("{:#?}", type_),
                                        })
                                        .expect(&*format!("{operand1:?} ; {type1:?} ; {type_:?}"))
                                        .to_owned() // TODO handle error
                                } else {
                                    unreachable!()
                                },
                            ),
                            args: vec![*operand1.to_owned(), *operand2.to_owned()],
                            kwargs: Default::default(),
                        };
                        self.process(typelist)?
                    }
                })
            }
            Element::UnaryOpr {
                type_,
                operand,
                position,
                raw,
                ..
            } => {
                let operand_type = operand.process(typelist)?;
                *self = Element::Call {
                    position: position.to_owned(),
                    raw: raw.to_owned(),
                    called: Box::new(
                        if let Type::Definition(TypeDefinition {
                            implementations, ..
                        }) = operand_type
                        {
                            implementations
                                .get(match type_ {
                                    OprType::Not => "_not",
                                    OprType::PlusSign => "_un_plus",
                                    OprType::MinusSign => "_un_minus",
                                    _ => panic!(),
                                })
                                .unwrap()
                                .to_owned() // TODO handle error
                        } else {
                            unreachable!()
                        },
                    ),
                    args: vec![*operand.to_owned()],
                    kwargs: Default::default(),
                };
                self.process(typelist)
            }
            Element::Procedure {
                is_fn,
                return_type: pre_return_type,
                content,
                args,
                position,
                raw,
            } => {
                typelist.add_frame(
                    None,
                    if *is_fn {
                        FrameType::Function
                    } else {
                        FrameType::Normal
                    },
                );
                let return_type = pre_return_type.process(typelist)?;
                for arg in args {
                    let value = arg.type_.process(typelist)?;
                    typelist.declare_val(&arg.name, &value);
                }
                let (res, block_return_type) = Element::block_type(content, typelist, false)?;
                if return_type == UNIT_T.get_instance().as_type_element()
                    || block_return_type.is_none()
                {
                    *pre_return_type = Box::new(res.as_literal());
                } else if let Some(block_return_type) = block_return_type {
                    if return_type != block_return_type {
                        return Err(ZyxtError::error_4_t(return_type, block_return_type)
                            .with_pos_raw(pos_raw));
                    }
                }
                typelist.pop_frame();
                Ok(Type::Instance(TypeInstance {
                    name: Some("proc".into()),
                    //name: Some(if *is_fn { "fn" } else { "proc" }.into()),
                    type_args: vec![UNIT_T.as_type().as_type_element(), return_type],
                    implementation: PROC_T.as_type_element(),
                }))
            } // TODO angle bracket thingy when it is implemented
            Element::Preprocess { content, .. } => {
                let mut pre_typelist = InterpreterData::<Type<Element>, _>::new(typelist.out);
                let pre_instructions = gen_instructions(content.to_owned(), &mut pre_typelist)?;
                let mut i_data = InterpreterData::<Value, _>::new(typelist.out);
                let pre_value = interpret_block(&pre_instructions, &mut i_data, true, false)?;
                *self = pre_value.as_element();
                self.process(typelist)
            }
            Element::Defer { content, .. } =>
            // TODO check block return against call stack
            {
                Ok(Element::block_type(content, typelist, false)?.0)
            }
            Element::Set {
                position,
                variable,
                content,
                raw,
                ..
            } => {
                if !variable.is_pattern() {
                    return Err(
                        ZyxtError::error_2_2(*variable.to_owned()).with_element(&**variable)
                    );
                }
                let content_type = content.process(typelist)?;
                let var_type = typelist.get_val(&variable.get_name(), pos_raw)?;
                if content_type != var_type {
                    Err(
                        ZyxtError::error_4_3(variable.get_name(), var_type, content_type)
                            .with_pos_raw(pos_raw),
                    )
                } else {
                    Ok(var_type)
                }
            }
            Element::Class {
                content,
                implementations,
                inst_fields,
                args,
                is_struct,
                ..
            } => {
                typelist.add_frame(None, FrameType::Normal);
                for expr in content.iter_mut() {
                    expr.process(typelist)?;
                    if let Element::Declare {
                        variable,
                        content,
                        flags,
                        type_,
                        ..
                    } = expr
                    {
                        if flags.contains(&Flag::Inst) && args != &None {
                            todo!("raise error here")
                        }
                        if flags.contains(&Flag::Inst) {
                            inst_fields.insert(
                                variable.get_name(),
                                (*type_.to_owned(), Some(content.to_owned())),
                            );
                        }
                    }
                }
                if args.is_some() && implementations.contains_key("_init") {
                    todo!("raise error here")
                }
                for item in implementations.values_mut() {
                    item.process(typelist)?;
                }
                let new_inst_fields = inst_fields
                    .iter_mut()
                    .map(|(ident, (ty, default))| {
                        let ty = ty.process(typelist)?;
                        if let Some(default) = default {
                            if ty != default.process(typelist)? {
                                todo!("raise error")
                            }
                        }
                        Ok((
                            ident.to_owned(),
                            (Box::new(ty), default.to_owned().map(|a| *a)),
                        ))
                    })
                    .collect::<Result<HashMap<_, _>, _>>()?;
                typelist.pop_frame();
                Ok(Type::Definition(TypeDefinition {
                    inst_name: None,
                    name: Some(if *is_struct { "struct" } else { "class" }.into()),
                    generics: vec![],
                    implementations: implementations.to_owned(),
                    inst_fields: new_inst_fields,
                }))
            }
            Element::NullElement
            | Element::Delete { .. }
            | Element::Comment { .. }
            | Element::Return { .. } => Ok(UNIT_T.get_instance().as_type_element()),
            Element::Token(Token {
                position, value, ..
            }) => Err(ZyxtError::error_2_1_0(value.to_owned())
                .with_pos_and_raw(position, &value.to_string())),
        }
    }
}
