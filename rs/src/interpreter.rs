use crate::ZyxtError;
use crate::objects::element::{Argument, Element};
use crate::objects::variable::Variable;
use crate::objects::varstack::Varstack;


pub(crate) fn interpret_expr(input: Element, varlist: &mut Varstack<Variable>) -> Result<Variable, ZyxtError> {
    match input {
        Element::Token(..) | Element::Comment {..} => panic!(),
        Element::NullElement => Ok(Variable::Null),
        Element::UnaryOpr {type_, operand, position, ..} =>
            if let Some(v) = interpret_expr(*operand.clone(), varlist)?.un_opr(&type_)
                {Ok(v)} else {
                    Err(ZyxtError::from_pos(&position)
                        .error_4_1_1(type_.to_string(),
                                     interpret_expr(*operand, varlist)?))
                },
        Element::BinaryOpr {type_, operand1, operand2, position, ..} =>
            if let Some(v) = interpret_expr(*operand1.clone(), varlist)?
                .bin_opr(&type_, interpret_expr(*operand2.clone(), varlist)?)
                {Ok(v)} else {
                    Err(ZyxtError::from_pos(&position)
                        .error_4_1_0(type_.to_string(),
                                 interpret_expr(*operand1, varlist)?,
                                 interpret_expr(*operand2, varlist)?))
            },
        Element::Variable {name, position, ..} => varlist.get_val(&name, &position),
        Element::Declare {variable, content, ..} => {
            let var = interpret_expr(*content, varlist);
            varlist.declare_val(&variable.get_name(), &var.clone()?);
            var
        },
        Element::Set {variable, content, position, ..} => {
            let var = interpret_expr(*content, varlist);
            varlist.set_val(&variable.get_name(), &var.clone()?, &position)?;
            var
        },
        Element::Literal {type_, content, ..} => {
            Ok(Variable::from_type_content(type_, content))
        },
        Element::Call {called, args: input_args, position, ..} => {
            if let Element::Variable {ref parent, ref name, ..} = *called {
                if name == &"println".to_string() && parent.get_name() == "std".to_string() {
                    println!("{}", input_args.into_iter().
                        map(|arg| interpret_expr(arg, varlist))
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .map(|v| v.get_displayed_value())
                        .collect::<Vec<String>>().join(" "));
                    return Ok(Variable::Null)
                }
            }
            let to_call = interpret_expr(*called.clone(), varlist)?;
            if let Variable::Proc {is_fn, args, content, ..} = to_call {
                let mut fn_varlist: Varstack<Variable> = Varstack::<Variable>::default_variable();
                let mut cursor = 0;
                for Argument {name, default, ..} in args {
                    let input_arg = if input_args.len() > cursor {input_args.get(cursor).unwrap().clone()}
                    else {default.unwrap()};
                    fn_varlist.declare_val(&name, &interpret_expr(input_arg, varlist)?);
                    cursor += 1;
                }
                let proc_varlist = if is_fn {&mut fn_varlist} else {
                    varlist.add_set();
                    for (k, v) in fn_varlist.0[0].iter() {varlist.declare_val(k, v);}
                    varlist
                };
                let res = interpret_block(content, proc_varlist, true, false);
                proc_varlist.pop_set();
                res
            } else {
                if let Some(v) = to_call.call(input_args.into_iter()
                    .map(|a| interpret_expr(a, varlist))
                    .collect::<Result<Vec<_>, _>>()?) {Ok(v)} else {
                    Err(ZyxtError::from_pos(&position).error_3_1(to_call, "#call".to_string()))
                }
            }
        },
        Element::If {conditions, ..} => {
            for cond in conditions {
                if cond.condition == Element::NullElement {
                    return interpret_block(cond.if_true, varlist, false, true)
                } else if let Variable::Bool(true) = interpret_expr(cond.condition, varlist)? {
                    return interpret_block(cond.if_true, varlist, false, true)
                }
            }
            Ok(Variable::Null)
        },
        Element::Block {content, ..} => interpret_block(content, varlist, true, true),
        Element::Delete {names, position, ..} => {
            for name in names {varlist.delete_val(&name, &position)?;}
            Ok(Variable::Null)
        },
        Element::Return { value, ..} => Ok(Variable::Return(Box::new(interpret_expr(*value, varlist)?))),
        Element::Procedure {is_fn, args, return_type, content, ..} => Ok(Variable::Proc {
            is_fn, args, return_type, content
        })
    }
}

pub fn interpret_block(input: Vec<Element>, varlist: &mut Varstack<Variable>, returnable: bool, add_set: bool) -> Result<Variable, ZyxtError> {
    let mut last = Variable::Null;
    if add_set {varlist.add_set();}
    for ele in input {
        if let Element::Return { value, ..} = &ele {
            if returnable {last = interpret_expr(*value.clone(), varlist)?}
            else {last = interpret_expr(ele, varlist)?;}
            if add_set {varlist.pop_set();}
            return Ok(last)
        } else {
            last = interpret_expr(ele, varlist)?;
            if let Variable::Return(value) = last {
                if add_set {varlist.pop_set();}
                return if returnable {Ok(*value)} else {Ok(Variable::Return(value))}
            }
        }
    }
    if add_set {varlist.pop_set();}
    Ok(last)
}

pub fn interpret_asts(input: Vec<Element>) -> Result<i32, ZyxtError> {
    let mut varlist = Varstack::<Variable>::default_variable();
    for ele in input {
        if let Element::Return { value, position, ..} = ele {
            let return_val = interpret_expr(*value, &mut varlist)?;
            return if let Variable::I32(v) = return_val { Ok(v) } else {
                Err(ZyxtError::from_pos(&position).error_4_2(return_val))
            }
        } else {
            if let Variable::Return(value) = interpret_expr(ele.clone(), &mut varlist)? {
                varlist.pop_set();
                return if let Variable::I32(v) = *value { Ok(v) } else {
                    Err(ZyxtError::from_pos(ele.get_pos()).error_4_2(*value))
                }
            }}
    }
    Ok(0)
}
