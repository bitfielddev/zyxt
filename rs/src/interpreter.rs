use std::collections::HashMap;
use crate::{Type, ZyxtError};
use crate::objects::element::{Argument, Element};
use crate::objects::value::Value;
use crate::objects::interpreter_data::{FrameData, InterpreterData};


pub fn interpret_expr(input: Element, i_data: &mut InterpreterData<Value>) -> Result<Value, ZyxtError> {
    match input {
        Element::Token(..) | Element::Comment {..} | Element::Preprocess {..} => panic!(),
        Element::NullElement => Ok(Value::Null),
        Element::UnaryOpr {type_, operand, position, raw, ..} =>
            if let Ok(v) = interpret_expr(*operand.clone(), i_data)?.un_opr(&type_)
                {Ok(v)} else {
                    Err(ZyxtError::from_pos_and_raw(&position, &raw)
                        .error_4_1_1(type_.to_string(),
                                     interpret_expr(*operand, i_data)?))
                },
        Element::BinaryOpr {type_, operand1, operand2, position, raw, ..} =>
            if let Ok(v) = interpret_expr(*operand1.clone(), i_data)?
                .bin_opr(&type_, interpret_expr(*operand2.clone(), i_data)?)
                {Ok(v)} else {
                    Err(ZyxtError::from_pos_and_raw(&position, &raw)
                        .error_4_1_0(type_.to_string(),
                                 interpret_expr(*operand1, i_data)?,
                                 interpret_expr(*operand2, i_data)?))
            },
        Element::Variable {name, position, raw, ..} => i_data.get_val(&name, &position, &raw),
        Element::Declare {variable, content, ..} => {
            let var = interpret_expr(*content, i_data);
            i_data.declare_val(&variable.get_name(), &var.clone()?);
            var
        },
        Element::Set {variable, content, position, raw, ..} => {
            let var = interpret_expr(*content, i_data);
            i_data.set_val(&variable.get_name(), &var.clone()?, &position, &raw)?;
            var
        },
        Element::Literal {type_, content, ..} => {
            Ok(Value::from_type_content(type_, content))
        },
        Element::Call {called, args: input_args, position, raw, ..} => {
            if let Element::Variable {ref parent, ref name, ..} = *called {
                if name == &"out".to_string() && parent.get_name() == *"ter" {
                    println!("{}", input_args.into_iter().
                        map(|arg| interpret_expr(arg, i_data))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>().join(" "));
                    return Ok(Value::Null)
                }
            }
            let to_call = interpret_expr(*called, i_data)?;
            if let Value::Proc {is_fn, args, content, ..} = to_call {
                let mut processed_args = HashMap::new();
                for (cursor, Argument {name, default, ..}) in args.into_iter().enumerate() {
                    let input_arg = if input_args.len() > cursor {input_args.get(cursor).unwrap().clone()}
                    else {default.unwrap()};
                    processed_args.insert(name, interpret_expr(input_arg, i_data)?);
                }

                let mut fn_i_data = InterpreterData::<Value>::default_variable();
                let fn_i_data=  if is_fn {
                    &mut fn_i_data
                } else {
                    i_data.add_frame(Some(FrameData {
                        position,
                        raw_call: raw,
                        args: processed_args.clone(),
                    }));
                    i_data
                };
                fn_i_data.heap.last_mut().unwrap().extend(processed_args);

                let res = interpret_block(content, fn_i_data, true, false);
                fn_i_data.pop_frame()?;
                res
            } else if let Ok(v) = to_call.call(input_args.into_iter()
                .map(|a| interpret_expr(a, i_data))
                .collect::<Result<Vec<_>, _>>()?) {Ok(v)} else {
                Err(ZyxtError::from_pos_and_raw(&position, &raw)
                    .error_3_1_1(to_call, "#call".to_string()))
            }
        },
        Element::If {conditions, ..} => {
            for cond in conditions {
                if cond.condition == Element::NullElement {
                    return interpret_block(cond.if_true, i_data, false, true)
                } else if let Value::Bool(true) = interpret_expr(cond.condition, i_data)? {
                    return interpret_block(cond.if_true, i_data, false, true)
                }
            }
            Ok(Value::Null)
        },
        Element::Block {content, ..} => interpret_block(content, i_data, true, true),
        Element::Delete {names, position, raw, ..} => {
            for name in names {i_data.delete_val(&name, &position, &raw)?;}
            Ok(Value::Null)
        },
        Element::Return { value, ..} => Ok(Value::Return(Box::new(interpret_expr(*value, i_data)?))),
        Element::Procedure {is_fn, args, return_type, content, ..} => Ok(Value::Proc {
            is_fn, args, return_type, content
        }),
        Element::Defer {content, ..} => {
            i_data.add_defer(content);
            Ok(Value::Null)
        },
        Element::Class {class_attrs, inst_attrs, is_struct, ..} => Ok(Value::Type(Type::Definition {
            name: if is_struct {"struct"} else {"class"}.to_string(),
            generics: vec![],
            class_attrs, inst_attrs
        })),
    }
}

pub fn interpret_block(input: Vec<Element>, i_data: &mut InterpreterData<Value>,
                       returnable: bool, add_frame: bool) -> Result<Value, ZyxtError> {
    let mut last = Value::Null;

    macro_rules! pop {
        () => {
            if add_frame {
                let res = i_data.pop_frame()?;
                if let Some(res) = res {
                    return Ok(res)
                }
            }
        }
    }

    if add_frame {
        i_data.add_frame(None);
    }
    for ele in input {
        if let Element::Return { value, ..} = &ele {
            if returnable { last = interpret_expr(*value.clone(), i_data)? }
            else { last = interpret_expr(ele, i_data)?; }
            pop!();
            return Ok(last)
        } else {
            last = interpret_expr(ele, i_data)?;
            if let Value::Return(value) = last {
                pop!();
                return if returnable {Ok(*value)} else {Ok(Value::Return(value))}
            }
        }
    }
    pop!();
    Ok(last)
}

pub fn interpret_asts(input: Vec<Element>) -> Result<i32, ZyxtError> {
    let mut i_data = InterpreterData::<Value>::default_variable();
    let mut last = Value::Null;
    for ele in &input {
        if let Element::Return { value, position, raw, ..} = ele {
            let mut return_val = interpret_expr(*value.clone(), &mut i_data)?;
            let res = i_data.pop_frame()?;
            if let Some(res) = res {
                return_val = res;
            }
            return if let Value::I32(v) = return_val { Ok(v) } else {
                Err(ZyxtError::from_pos_and_raw(position, raw).error_4_2(return_val))
            }
        } else {
            last = interpret_expr(ele.clone(), &mut i_data)?;
            if let Value::Return(mut value) = last {
                let res = i_data.pop_frame()?;
                if let Some(res) = res {
                    value = Box::new(res);
                }
                return if let Value::I32(v) = *value { Ok(v) } else {
                    Err(ZyxtError::from_pos_and_raw(ele.get_pos(), &ele.get_raw()).error_4_2(*value))
                }
            }}
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
        Err(ZyxtError::from_pos_and_raw(last_ele.get_pos(), &last_ele.get_raw())
            .error_4_2(last))
    }
}
