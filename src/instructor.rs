use std::collections::HashMap;
use crate::{
    types::{element::Element, interpreter_data::InterpreterData, printer::Print, typeobj::Type},
    ZyxtError,
};
use crate::interpreter::interpret_block;
use crate::types::token::{Flag, OprType, Token};
use crate::types::typeobj::bool_t::BOOL_T;
use crate::types::typeobj::proc_t::PROC_T;
use crate::types::typeobj::type_t::TYPE_T;
use crate::types::typeobj::unit_t::UNIT_T;

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
            } => typelist.get_val(name, position, raw),
            Element::Block { content, .. } => Ok(Element::block_type(content, typelist, true)?.0),
            Element::Call { called, args, .. } => {
                let called_type = called.process(typelist)?;
                if let Element::Procedure {args: args_objs, return_type: pre_return_type, ..} = called.as_mut() {
                    for (i, arg) in args.iter_mut().enumerate() {
                        if arg.process(typelist)? != args_objs.get_mut(i).unwrap().type_.process(typelist)? {
                            todo!("errors")
                        }
                    }
                    pre_return_type.process(typelist)
                } else {
                    *called = if let Type::Definition {implementations, ..} = called_type.implementation() {
                        Box::new(implementations.get("_call").unwrap().to_owned()) // TODO handle error
                    } else {
                        unreachable!()
                    };
                    self.process(typelist)
                }
            },
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
                let type_ = pre_type.process(typelist)?;
                if type_ == UNIT_T.as_type_element() {
                    typelist.declare_val(&variable.get_name(), &content_type);
                } else {
                    typelist.declare_val(&variable.get_name(), &type_);
                    if content_type != type_ {
                        let new_content = Element::BinaryOpr {
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            type_: OprType::TypeCast,
                            operand1: content.to_owned(),
                            operand2: Box::new(type_.as_literal()),
                        };
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
                        if type2 == TYPE_T.as_type_element() {
                            *self = Element::Call {
                                position: Default::default(),
                                raw: "".to_string(),
                                called: Box::new(if let Type::Definition {implementations, ..} = type1.implementation() {
                                    implementations.get("_typecast").unwrap().to_owned() // TODO handle error
                                } else {
                                    unreachable!()
                                }),
                                args: vec![*operand1.to_owned(), *operand2.to_owned()],
                                kwargs: Default::default()
                            };
                            type2.get_instance().unwrap() // TODO handle error
                        } else {
                            todo!("Error here")
                        }
                    }
                    OprType::And | OprType::Or => {
                        for (type_, operand) in [(&type1, operand1), (&type2, operand2)] {
                            if *type_ != BOOL_T.as_type_element() {
                                *operand = Box::new(Element::Call {
                                    position: Default::default(),
                                    raw: "".to_string(),
                                    called: Box::new(if let Type::Definition {implementations, ..} = type1.implementation() {
                                        implementations.get("_typecast").unwrap().to_owned() // TODO handle error
                                    } else {
                                        unreachable!()
                                    }),
                                    args: vec![*operand.to_owned(), BOOL_T.as_type_element().as_literal()],
                                    kwargs: Default::default()
                                });
                            }
                        }
                        BOOL_T.get_instance().unwrap().as_type_element()
                    },
                    _ => {
                        *self = Element::Call {
                            position: position.to_owned(),
                            raw: raw.to_owned(),
                            called: Box::new(if let Type::Definition {implementations, ..} = type1.implementation() {
                                implementations.get(match type_ {
                                    OprType::Plus => "_add",
                                    OprType::Minus => "_sub",
                                    OprType::AstMult => "_mul",
                                    OprType::Div => "_div",
                                    OprType::Modulo => "_rem",
                                    OprType::Eq => "_eq",
                                    OprType::Noteq => "_ne",
                                    OprType::Lt => "_lt",
                                    OprType::Lteq => "_le",
                                    OprType::Gt => "_gt",
                                    OprType::Gteq => "_ge",
                                    OprType::Concat => "_concat",
                                    _ => unimplemented!()
                                }).unwrap().to_owned() // TODO handle error
                            } else {
                                unreachable!()
                            }),
                            args: vec![*operand1.to_owned(), *operand2.to_owned()],
                            kwargs: Default::default()
                        };
                        self.process(typelist)?
                    },
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
                    called: Box::new(if let Type::Definition {implementations, ..} = operand_type.implementation() {
                        implementations.get(match type_ {
                            OprType::Not => "_not",
                            OprType::PlusSign => "_un_plus",
                            OprType::MinusSign => "_un_minus",
                            _ => panic!()
                        }).unwrap().to_owned() // TODO handle error
                    } else {
                        unreachable!()
                    }),
                    args: vec![*operand.to_owned()],
                    kwargs: Default::default()
                };
                self.process(typelist)
            }
            Element::Procedure {
                is_fn,
                return_type: pre_return_type,
                content,
                args,
                position,
                raw
            } => {
                /*let mut a = InterpreterData::default_type(typelist.out);
                let typelist = if *is_fn {
                    &mut a
                } else {
                    typelist
                };*/ // TODO
                typelist.add_frame(None);
                let return_type = pre_return_type.process(typelist)?;
                for arg in args {
                    let value = arg.type_.process(typelist)?;
                    typelist.declare_val(&arg.name, &value);
                }
                let (res, block_return_type) = Element::block_type(content, typelist, false)?;
                if return_type == UNIT_T.as_type_element() || block_return_type.is_none() {
                    *pre_return_type = Box::new(res.as_literal());
                } else if let Some(block_return_type) = block_return_type {
                    if return_type != block_return_type {
                        return Err(ZyxtError::error_4_t(
                            return_type.to_owned(),
                            block_return_type,
                        )
                            .with_pos_and_raw(position, raw));
                    }
                }
                typelist.pop_frame();
                Ok(Type::Instance {
                    name: Some("proc".into()),
                    //name: Some(if *is_fn { "fn" } else { "proc" }.into()),
                    type_args: vec![UNIT_T.as_type_element(), return_type],
                    implementation: Box::new(PROC_T.as_type_element()),
                })
            } // TODO angle bracket thingy when it is implemented
            Element::Preprocess { content, .. } => {
                let mut pre_typelist = InterpreterData::default_type(typelist.out);
                let pre_instructions = gen_instructions(content.to_owned(), &mut pre_typelist)?;
                let mut i_data = InterpreterData::default_variable(typelist.out);
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
                let var_type = typelist.get_val(&variable.get_name(), position, raw)?;
                if content_type != var_type {
                    Err(
                        ZyxtError::error_4_3(variable.get_name(), var_type, content_type)
                            .with_pos_and_raw(position, raw),
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
                typelist.add_frame(None);
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
                let new_inst_fields = inst_fields.iter_mut()
                    .map(|(ident, (ty, default))| {
                        let ty = ty.process(typelist)?;
                        if let Some(default) = default {
                            if ty != default.process(typelist)? {
                                todo!("raise error")
                            }
                        }
                        Ok((ident.to_owned(), (Box::new(ty), default.to_owned().map(|a| *a))))
                    }).collect::<Result<HashMap<_, _>, _>>()?;
                typelist.pop_frame();
                Ok(Type::Definition {
                    inst_name: None,
                    name: Some(if *is_struct { "struct" } else { "class" }.into()),
                    generics: vec![],
                    implementations: implementations.to_owned(),
                    inst_fields: new_inst_fields
                })
            }
            Element::NullElement
            | Element::Delete { .. }
            | Element::Comment { .. }
            | Element::Return { .. } => Ok(UNIT_T.as_type_element()),
            Element::Token(Token {
                               position, value, ..
                           }) => Err(ZyxtError::error_2_1_0(value.to_owned())
                .with_pos_and_raw(position, &value.to_string())),
        }
    }
}