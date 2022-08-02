use crate::objects::element::{Argument, Element};
use crate::objects::interpreter_data::{FrameData, InterpreterData, Print};
use crate::objects::token::OprType;
use crate::objects::value::{logic, Value};
use crate::{Type, ZyxtError};
use std::collections::HashMap;

pub fn interpret_expr<O: Print>(
    input: &Element,
    i_data: &mut InterpreterData<Value, O>,
) -> Result<Value, ZyxtError> {
    match input {
        Element::Token(..) | Element::Comment { .. } | Element::Preprocess { .. } => panic!(),
        Element::NullElement => Ok(Value::Null),
        Element::UnaryOpr {
            type_,
            operand,
            position,
            raw,
            ..
        } => {
            if let Ok(v) = interpret_expr(operand, i_data)?.un_opr(type_) {
                Ok(v)
            } else {
                Err(
                    ZyxtError::error_4_1_1(type_.to_string(), interpret_expr(operand, i_data)?)
                        .with_pos_and_raw(position, raw),
                )
            }
        }
        Element::BinaryOpr {
            type_,
            operand1,
            operand2,
            position,
            raw,
            ..
        } => {
            if let Ok(v) = match type_ {
                OprType::And => Ok(logic::and(operand1, operand2, i_data)?),
                OprType::Or => Ok(logic::or(operand1, operand2, i_data)?),
                _ => interpret_expr(operand1, i_data)?
                    .bin_opr(type_, interpret_expr(operand2, i_data)?),
            } {
                Ok(v)
            } else {
                Err(ZyxtError::error_4_1_0(
                    type_.to_string(),
                    interpret_expr(operand1, i_data)?,
                    interpret_expr(operand2, i_data)?,
                )
                .with_pos_and_raw(position, raw))
            }
        }
        Element::Variable {
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
        Element::Literal { type_, content, .. } => Ok(Value::from_type_content(
            type_.to_owned(),
            content.to_owned(),
        )),
        Element::Call {
            called,
            args: input_args,
            position,
            raw,
            ..
        } => {
            if let Element::Variable { parent, name, .. } = called.as_ref() {
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
                    return Ok(Value::Null);
                }
            }
            let to_call = interpret_expr(called, i_data)?;
            if let Value::Proc {
                is_fn,
                args,
                content,
                ..
            } = to_call
            {
                let mut processed_args = HashMap::new();
                for (cursor, Argument { name, default, .. }) in args.into_iter().enumerate() {
                    let default = default.unwrap();
                    let input_arg = if input_args.len() > cursor {
                        input_args.get(cursor).unwrap()
                    } else {
                        &default
                    };
                    processed_args.insert(name, interpret_expr(input_arg, i_data)?);
                }

                if is_fn {
                    let mut fn_i_data = InterpreterData::default_variable(i_data.out);
                    fn_i_data.heap.last_mut().unwrap().extend(processed_args);
                    let res = interpret_block(&content, &mut fn_i_data, true, false);
                    fn_i_data.pop_frame()?;
                    res
                } else {
                    i_data.add_frame(Some(FrameData {
                        position: position.to_owned(),
                        raw_call: raw.to_owned(),
                        args: processed_args.to_owned(),
                    }));
                    i_data.heap.last_mut().unwrap().extend(processed_args);
                    let res = interpret_block(&content, i_data, true, false);
                    i_data.pop_frame()?;
                    res
                }
            } else if let Ok(v) = to_call.call(
                input_args
                    .iter()
                    .map(|a| interpret_expr(a, i_data))
                    .collect::<Result<Vec<_>, _>>()?,
            ) {
                Ok(v)
            } else {
                Err(ZyxtError::error_3_1_1(to_call, "_call".to_string())
                    .with_pos_and_raw(position, raw))
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
            Ok(Value::Null)
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
            Ok(Value::Null)
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
        } => Ok(Value::Proc {
            is_fn: *is_fn,
            args: args.to_owned(),
            return_type: return_type.to_owned(),
            content: content.to_owned(),
        }),
        Element::Defer { content, .. } => {
            i_data.add_defer(content.to_owned());
            Ok(Value::Null)
        }
        Element::Class {
            class_attrs,
            inst_attrs,
            is_struct,
            ..
        } => Ok(Value::Type(Type::Definition {
            name: if *is_struct { "struct" } else { "class" }.to_string(),
            generics: vec![],
            class_attrs: class_attrs.to_owned(),
            inst_attrs: inst_attrs.to_owned(),
        })),
    }
}

pub fn interpret_block<O: Print>(
    input: &Vec<Element>,
    i_data: &mut InterpreterData<Value, O>,
    returnable: bool,
    add_frame: bool,
) -> Result<Value, ZyxtError> {
    let mut last = Value::Null;

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
        i_data.add_frame(None);
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
    let mut last = Value::Null;
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
    } else if let Value::Null = last {
        Ok(0)
    } else {
        let last_ele = input.last().unwrap();
        Err(ZyxtError::error_4_2(last).with_pos_and_raw(last_ele.get_pos(), &last_ele.get_raw()))
    };
}
