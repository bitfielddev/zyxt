use std::collections::HashMap;

use crate::{
    types::{
        element::{Argument, Element},
        interpreter_data::{FrameData, FrameType, InterpreterData},
        printer::Print,
        token::OprType,
        value::{Proc, Value},
    },
    Type, ZyxtError,
};

pub fn interpret_expr<O: Print>(
    input: &Element,
    i_data: &mut InterpreterData<Value, O>,
) -> Result<Value, ZyxtError> {
    match input {
        Element::Token(..)
        | Element::Comment { .. }
        | Element::Preprocess { .. }
        | Element::UnaryOpr { .. } => panic!("{input:#?}"),
        Element::NullElement => Ok(Value::Unit),
        Element::BinaryOpr {
            type_,
            operand1,
            operand2,
            ..
        } => match type_ {
            OprType::And => {
                if let Value::Bool(b) = interpret_expr(operand1, i_data)? {
                    if b {
                        if let Value::Bool(b) = interpret_expr(operand2, i_data)? {
                            Ok(Value::Bool(b))
                        } else {
                            panic!()
                        }
                    } else {
                        Ok(Value::Bool(false))
                    }
                } else {
                    panic!()
                }
            }
            OprType::Or => {
                if let Value::Bool(b) = interpret_expr(operand1, i_data)? {
                    if !b {
                        if let Value::Bool(b) = interpret_expr(operand2, i_data)? {
                            Ok(Value::Bool(b))
                        } else {
                            panic!()
                        }
                    } else {
                        Ok(Value::Bool(true))
                    }
                } else {
                    panic!()
                }
            }
            opr => panic!("{opr:?}"),
        },
        Element::Ident {
            name,
            position,
            raw,
            ..
        } => i_data.get_val(name, position, raw),
        Element::Declare {
            variable, content, ..
        } => {
            let var = interpret_expr(content, i_data);
            i_data.declare_val(&variable.get_name(), &var.to_owned()?);
            var
        }
        Element::Set {
            variable,
            content,
            position,
            raw,
            ..
        } => {
            let var = interpret_expr(content, i_data);
            i_data.set_val(&variable.get_name(), &var.to_owned()?, position, raw)?;
            var
        }
        Element::Literal { content, .. } => Ok(if let Value::PreType(v) = content {
            Value::Type(v.as_type_value(i_data)?)
        } else {
            content.to_owned()
        }),
        Element::Call {
            called,
            args: input_args,
            position,
            raw,
            ..
        } => {
            if let Element::Ident { parent, name, .. } = called.as_ref() {
                if *name == "out" && parent.get_name() == *"ter" {
                    let s = input_args
                        .iter()
                        .map(|arg| interpret_expr(arg, i_data))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    i_data.out.println(s);
                    return Ok(Value::Unit);
                }
            }
            if let Value::Proc(proc) = interpret_expr(called, i_data)? {
                match proc {
                    Proc::Builtin { f, .. } => {
                        let processed_args = input_args
                            .iter()
                            .map(|a| interpret_expr(a, i_data))
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
                        for (
                            cursor,
                            Argument {
                                name, ref default, ..
                            },
                        ) in args.into_iter().enumerate()
                        {
                            let input_arg = if input_args.len() > cursor {
                                input_args.get(cursor).unwrap()
                            } else {
                                default.as_ref().unwrap()
                            };
                            processed_args.insert(name, interpret_expr(input_arg, i_data)?);
                        }
                        i_data
                            .add_frame(
                                Some(FrameData {
                                    position: position.to_owned(),
                                    raw_call: raw.to_owned(),
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
                        let res = interpret_block(&content, i_data, true, false);
                        i_data.pop_frame()?;
                        res
                    }
                }
            } else {
                panic!()
            }
        }
        Element::If { conditions, .. } => {
            for cond in conditions {
                if cond.condition == Element::NullElement {
                    return interpret_block(&cond.if_true, i_data, false, true);
                } else if let Value::Bool(true) = interpret_expr(&cond.condition, i_data)? {
                    return interpret_block(&cond.if_true, i_data, false, true);
                }
            }
            Ok(Value::Unit)
        }
        Element::Block { content, .. } => interpret_block(content, i_data, true, true),
        Element::Delete {
            names,
            position,
            raw,
            ..
        } => {
            for name in names {
                i_data.delete_val(name, position, raw)?;
            }
            Ok(Value::Unit)
        }
        Element::Return { value, .. } => {
            Ok(Value::Return(Box::new(interpret_expr(value, i_data)?)))
        }
        Element::Procedure {
            is_fn,
            args,
            return_type,
            content,
            ..
        } => Ok(Value::Proc(Proc::Defined {
            is_fn: *is_fn,
            args: args.to_owned(),
            return_type: if let Value::Type(value) = interpret_expr(return_type, i_data)? {
                value
            } else {
                panic!("{:#?}", input)
            },
            content: content.to_owned(),
        })),
        Element::Defer { content, .. } => {
            i_data.add_defer(content.to_owned());
            Ok(Value::Unit)
        }
        Element::Class {
            implementations,
            inst_fields,
            is_struct,
            ..
        } => Ok(Value::Type(Type::Definition {
            name: Some(if *is_struct { "struct" } else { "class" }.into()),
            inst_name: None,
            generics: vec![],
            implementations: implementations
                .iter()
                .map(|(k, v)| Ok((k.to_owned(), interpret_expr(v, i_data)?)))
                .collect::<Result<HashMap<_, _>, _>>()?,
            inst_fields: inst_fields
                .iter()
                .map(|(k, (v1, v2))| {
                    Ok((
                        k.to_owned(),
                        (
                            Box::new(if let Value::Type(value) = interpret_expr(v1, i_data)? {
                                value
                            } else {
                                panic!()
                            }),
                            v2.to_owned()
                                .map(|v2| interpret_expr(v2.as_ref(), i_data))
                                .transpose()?,
                        ),
                    ))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
        })),
    }
}

pub fn interpret_block<O: Print>(
    input: &Vec<Element>,
    i_data: &mut InterpreterData<Value, O>,
    returnable: bool,
    add_frame: bool,
) -> Result<Value, ZyxtError> {
    let mut last = Value::Unit;

    macro_rules! pop {
        () => {
            if add_frame {
                let res = i_data.pop_frame()?;
                if let Some(res) = res {
                    return Ok(res);
                }
            }
        };
    }

    if add_frame {
        i_data.add_frame(None, FrameType::Normal);
    }
    for ele in input {
        if let Element::Return { value, .. } = &ele {
            if returnable {
                last = interpret_expr(value, i_data)?
            } else {
                last = interpret_expr(ele, i_data)?;
            }
            pop!();
            return Ok(last);
        } else {
            last = interpret_expr(ele, i_data)?;
            if let Value::Return(value) = last {
                pop!();
                return if returnable {
                    Ok(*value)
                } else {
                    Ok(Value::Return(value))
                };
            }
        }
    }
    pop!();
    Ok(last)
}

pub fn interpret_asts<O: Print>(
    input: &Vec<Element>,
    i_data: &mut InterpreterData<Value, O>,
) -> Result<i32, ZyxtError> {
    let mut last = Value::Unit;
    for ele in input {
        if let Element::Return {
            value,
            position,
            raw,
            ..
        } = ele
        {
            let mut return_val = interpret_expr(value, i_data)?;
            let res = i_data.pop_frame()?;
            if let Some(res) = res {
                return_val = res;
            }
            return if let Value::I32(v) = return_val {
                Ok(v)
            } else {
                Err(ZyxtError::error_4_2(return_val).with_pos_and_raw(position, raw))
            };
        } else {
            last = interpret_expr(ele, i_data)?;
            if let Value::Return(mut value) = last {
                let res = i_data.pop_frame()?;
                if let Some(res) = res {
                    value = Box::new(res);
                }
                return if let Value::I32(v) = *value {
                    Ok(v)
                } else {
                    Err(ZyxtError::error_4_2(*value)
                        .with_pos_and_raw(ele.get_pos(), &ele.get_raw()))
                };
            }
        }
    }
    let res = i_data.pop_frame()?;
    if let Some(res) = res {
        last = res;
    }
    return if let Value::I32(v) = last {
        Ok(v)
    } else if let Value::Unit = last {
        Ok(0)
    } else {
        let last_ele = input.last().unwrap();
        Err(ZyxtError::error_4_2(last).with_pos_and_raw(last_ele.get_pos(), &last_ele.get_raw()))
    };
}
