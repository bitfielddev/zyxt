use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors;
use crate::lexer::Position;
use crate::syntax::element::{Argument, Element};
use crate::syntax::token::OprType;
use crate::syntax::typeobj::TypeObj;

pub struct Varstack<T: Clone>(Vec<HashMap<String, T>>);
impl <T: Clone> Varstack<T> {
    pub fn default_variable() -> Varstack<Variable> {
        let mut v = Varstack(vec![HashMap::new()]);
        for t in ["str", "i32", "f64", "#null", "type"] {
            v.0[0].insert(t.to_string(), Variable::Type(TypeObj::Prim{
                name: t.to_string(), type_args: vec![]
            }));
        }
        v
    }
    pub fn default_type() -> Varstack<TypeObj> {
        let mut v = Varstack(vec![HashMap::new()]);
        for t in ["str", "i32", "f64", "#null", "type"] {
            v.0[0].insert(t.to_string(), TypeObj::Prim{
                name: t.to_string(), type_args: vec![]
            });
        }
        v
    }
    pub fn add_set(&mut self) {
        self.0.push(HashMap::new());
    }
    pub fn pop_set(&mut self) {
        self.0.pop();
    }
    pub fn declare_val(&mut self, name: &String, value: &T) {
        self.0.last_mut().unwrap().insert(name.clone(), value.clone());
    }
    pub fn set_val(&mut self, name: &String, value: &T, position: &Position) {
        for set in self.0.iter_mut().rev() {
            if set.contains_key(name) {set.insert(name.clone(), value.clone()); return}
        }
        errors::error_pos(position);
        errors::error_3_0(name.clone());
    }
    pub fn get_val(&mut self, name: &String, position: &Position) -> T {
        for set in self.0.iter().rev() {
            if set.contains_key(name) {return set.get(name).unwrap().clone()}
        }
        errors::error_pos(position);
        errors::error_3_0(name.clone());
    }
    pub fn delete_val(&mut self, name: &String, position: &Position) -> T {
        self.0.last_mut().unwrap().remove(name).unwrap_or_else(|| {
            errors::error_pos(position);
            errors::error_3_0(name.clone());
        })
    }
}

#[derive(Clone)]
pub enum Variable {
    I32(i32),
    F64(f64),
    Str(String),
    Bool(bool),
    Type(TypeObj),
    Proc{
        is_fn: bool,
        args: Vec<Argument>,
        return_type: TypeObj,
        content: Vec<Element>
    },
    Null,
    Return(Box<Variable>)
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_displayed_value())
    }
}

impl Variable {
    pub fn get_displayed_value(&self) -> String {
        match self {
            Variable::I32(v) => v.to_string(),
            Variable::F64(v) => v.to_string(),
            Variable::Str(v) => v.clone(),
            Variable::Bool(v) => v.to_string(),
            Variable::Type(v) => "<".to_owned()+&*v.to_string()+">",
            Variable::Proc{is_fn, args, return_type, ..} =>
                format!("{}|{}|: {}",
                    if *is_fn {"fn"} else {"proc"},
                    args.iter().map(|a| format!("{}{}{}",
                        a.name, if a.type_ != TypeObj::any()
                            {": ".to_owned()+&*a.type_.to_string()} else {"".to_string()},
                        if let Some(_) = &a.default {": TODO"} else {""}
                    )).collect::<Vec<String>>().join(","),
                    return_type.to_string()),
            Variable::Null => "null".to_string(),
            Variable::Return(v) => v.get_displayed_value()
        }
    }
    pub fn un_opr(&self, type_: &OprType) -> Option<Variable> {
        if let Variable::Return(v) = self {return v.un_opr(type_)}
        match type_ { // will prob clean this up with macros
            OprType::PlusSign => match *self {
                Variable::I32(v) => Some(Variable::I32(v)),
                Variable::F64(v) => Some(Variable::F64(v)),
                _ => None
            },
            OprType::MinusSign => match *self {
                Variable::I32(v) => Some(Variable::I32(-v)),
                Variable::F64(v) => Some(Variable::F64(-v)),
                _ => None
            },
            _ => None
        }
    }
    pub fn bin_opr(&self, type_: &OprType, other: Variable) -> Option<Variable> {
        if let Variable::Return(v) = self {return v.bin_opr(type_, other)}
        match type_ { // will prob clean this up with macros
            OprType::Plus => match self {
                Variable::I32(v1) => match other {
                    Variable::I32(v2) => Some(Variable::I32(v1+v2)),
                    Variable::F64(v2) => Some(Variable::F64(*v1 as f64+v2)),
                    _ => None
                },
                Variable::F64(v1) => match other {
                    Variable::I32(v2) => Some(Variable::F64(v1+v2 as f64)),
                    Variable::F64(v2) => Some(Variable::F64(v1+v2)),
                    _ => None
                },
                _ => None
            },
            OprType::Minus => match self {
                Variable::I32(v1) => match other {
                    Variable::I32(v2) => Some(Variable::I32(v1-v2)),
                    Variable::F64(v2) => Some(Variable::F64(*v1 as f64-v2)),
                    _ => None
                },
                Variable::F64(v1) => match other {
                    Variable::I32(v2) => Some(Variable::F64(v1-v2 as f64)),
                    Variable::F64(v2) => Some(Variable::F64(v1-v2)),
                    _ => None
                },
                _ => None
            },
            OprType::Concat => match self {
                Variable::I32(v1) => match other {
                    Variable::I32(v2) => if let Ok(r2) = (v1.to_string()+&*v2.to_string()).parse::<i32>()
                        {Some(Variable::I32(r2))} else {None},
                    Variable::F64(v2) => if let Ok(r2) = (v1.to_string()+&*v2.to_string()).parse::<f64>()
                        {Some(Variable::F64(r2))} else {None},
                    Variable::Str(v2) => Some(Variable::Str(v1.to_string()+&*v2)),
                    _ => None
                },
                Variable::F64(v1) => match other {
                    Variable::I32(v2) => if let Ok(r2) = (v1.to_string()+&*v2.to_string()).parse::<f64>()
                        {Some(Variable::F64(r2))} else {None},
                    Variable::Str(v2) => Some(Variable::Str(v1.to_string()+&*v2)),
                    _ => None
                },
                Variable::Str(v1) => match other {
                    Variable::I32(v2) => Some(Variable::Str(v1.to_string()+&*v2.to_string())),
                    Variable::F64(v2) => Some(Variable::Str(v1.to_string()+&*v2.to_string())),
                    Variable::Str(v2) => Some(Variable::Str(v1.to_string()+&*v2)),
                    Variable::Bool(v2) => Some(Variable::Str(v1.to_string()+&*v2.to_string())),
                    _ => None
                },
                Variable::Bool(v1) => match other {
                    Variable::Str(v2) =>Some(Variable::Str(v1.to_string()+&*v2.to_string())),
                    _ => None
                },
                _ => None
            },
            OprType::TypeCast => match other {
                Variable::Type(t) => match &*t.to_string() {
                    "i32" => match self {
                        Variable::I32(..) => Some(self.clone()),
                        Variable::F64(v) => Some(Variable::I32(*v as i32)),
                        Variable::Str(v) => if let Ok(r) = v.parse::<i32>()
                            {Some(Variable::I32(r))} else {None},
                        Variable::Bool(v) => Some(Variable::I32(if *v {1} else {0})),
                        Variable::Null => Some(Variable::I32(0)),
                        _ => None
                    },
                    "f64" => match self {
                        Variable::I32(v) => Some(Variable::F64(*v as f64)),
                        Variable::F64(..) => Some(self.clone()),
                        Variable::Str(v) => if let Ok(r) = v.parse::<f64>()
                            {Some(Variable::F64(r))} else {None},
                        Variable::Bool(v) => Some(Variable::F64(if *v {1.0} else {0.0})),
                        Variable::Null => Some(Variable::F64(0.0)),
                        _ => None
                    },
                    "str" => Some(Variable::Str(self.get_displayed_value())),
                    "bool" => match self {
                        Variable::I32(v) => Some(Variable::Bool(*v != 0)),
                        Variable::F64(v) => Some(Variable::Bool(*v != 0.0)),
                        Variable::Str(v) => Some(Variable::Bool(v.len() != 0)),
                        Variable::Bool(..) => Some(self.clone()),
                        Variable::Type(..) => Some(Variable::Bool(true)),
                        Variable::Null => Some(Variable::Bool(false)),
                        _ => None
                    }
                    "type" => Some(self.get_type()),
                    _ => None
                },
                _ => None
            }
            _ => None
        }
    }
    pub fn default(type_: TypeObj) -> Self {
        match type_.clone() {
            TypeObj::Prim{name, ..} => match &*name {
                "i32" => Variable::I32(0),
                "f64" => Variable::F64(0.0),
                "str" => Variable::Str("".to_string()),
                "bool" => Variable::Bool(false),
                "#null" => Variable::Null,
                "type" => Variable::Type(TypeObj::null()),
                _ => panic!("{}", type_)
            }
            _ => panic!("{}", type_)
        }
    }
    pub fn from_type_content(type_: TypeObj, content: String) -> Variable {
        match type_ {
            TypeObj::Prim{name, ..} => match &*name {
                "i32" => Variable::I32(content.parse::<i32>().unwrap()),
                "f64" => Variable::F64(content.parse::<f64>().unwrap()),
                "str" => Variable::Str(content),
                "bool" => Variable::Bool(&*content == "true"),
                _ => panic!()
            }
            _ => panic!()
        }
    }
    pub fn get_type_obj(&self) -> TypeObj {
        if let Variable::Return(v) = self {
            return v.get_type_obj();
        }
        match self {
            Variable::I32(..) => TypeObj::from_str("i32"),
            Variable::F64(..) => TypeObj::from_str("f64"),
            Variable::Str(..) => TypeObj::from_str("str"),
            Variable::Bool(..) => TypeObj::from_str("bool"),
            Variable::Type(..) => TypeObj::from_str("type"),
            Variable::Proc {is_fn, return_type, ..} =>
                TypeObj::Prim{
                    name: if *is_fn {"fn"} else {"proc"}.to_string(),
                    type_args: vec![TypeObj::null(), return_type.clone()]
                }, // TODO angle bracket thingy when it is implemented
            Variable::Null => TypeObj::null(),
            _ => panic!()
        }
    }
    pub fn get_type(&self) -> Variable {
        Variable::Type(self.get_type_obj())
    }
}

fn interpret_expr(input: Element, varlist: &mut Varstack<Variable>) -> Variable {
    match input {
        Element::Token(..) | Element::Comment {..} => panic!(),
        Element::NullElement => Variable::Null,
        Element::UnaryOpr {type_, operand, position, ..} =>
            interpret_expr(*operand.clone(), varlist)
                .un_opr(&type_).unwrap_or_else(|| {
                errors::error_pos(&position);
                errors::error_4_1_1(type_.to_string(),
                                    interpret_expr(*operand, varlist));
            }),
        Element::BinaryOpr {type_, operand1, operand2, position, ..} =>
            interpret_expr(*operand1.clone(), varlist)
                .bin_opr(&type_, interpret_expr(*operand2.clone(), varlist))
                .unwrap_or_else(|| {
                errors::error_pos(&position);
                errors::error_4_1_0(type_.to_string(),
                                    interpret_expr(*operand1, varlist),
                                    interpret_expr(*operand2, varlist));
            }),
        Element::Variable {name, position, ..} => varlist.get_val(&name, &position),
        Element::Declare {variable, content, ..} => {
            let var = interpret_expr(*content, varlist);
            varlist.declare_val(&variable.get_name(), &var);
            var
        },
        Element::Set {variable, content, position, ..} => {
            let var = interpret_expr(*content, varlist);
            varlist.set_val(&variable.get_name(), &var, &position);
            var
        },
        Element::Literal {type_, content, ..} => {
            Variable::from_type_content(type_, content)
        },
        Element::Call {called, args, ..} => {
            let input_args = args;
            if let Element::Variable {ref parent, ref name, ..} = *called {
                if name == &"println".to_string() && parent.get_name() == "std".to_string() {
                    println!("{}", input_args.into_iter().
                        map(|arg| interpret_expr(arg, varlist).get_displayed_value())
                        .collect::<Vec<String>>().join(" "));
                    return Variable::Null
                }
            }
            let to_call = interpret_expr(*called.clone(), varlist);
            if let Variable::Proc {is_fn, args, content, ..} = to_call {
                let mut fn_varlist: Varstack<Variable> = Varstack::<Variable>::default_variable();
                let mut cursor = 0;
                for Argument {name, default, ..} in args {
                    let input_arg = if input_args.len() > cursor {input_args.get(cursor).unwrap().clone()}
                        else {default.unwrap()};
                    fn_varlist.declare_val(&name, &interpret_expr(input_arg, varlist));
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
                todo!("Call opr thingy")
            }
        },
        Element::If {conditions, ..} => {
            for cond in conditions {
                if cond.condition == Element::NullElement {
                    return interpret_block(cond.if_true, varlist, false, true)
                } else if let Variable::Bool(true) = interpret_expr(cond.condition, varlist) {
                    return interpret_block(cond.if_true, varlist, false, true)
                }
            }
            Variable::Null
        },
        Element::Block {content, ..} => interpret_block(content, varlist, true, true),
        Element::Delete {names, position, ..} => {
            for name in names {varlist.delete_val(&name, &position);}
            Variable::Null
        },
        Element::Return {value, ..} => Variable::Return(Box::new(interpret_expr(*value, varlist))),
        Element::Procedure {is_fn, args, return_type, content, ..} => Variable::Proc {
            is_fn, args, return_type, content
        }
    }
}

pub fn interpret_block(input: Vec<Element>, varlist: &mut Varstack<Variable>, returnable: bool, add_set: bool) -> Variable {
    let mut last = Variable::Null;
    if add_set {varlist.add_set();}
    for ele in input {
        if let Element::Return {value, ..} = &ele {
            if returnable {last = interpret_expr(*value.clone(), varlist)}
            else {last = interpret_expr(ele, varlist);}
            if add_set {varlist.pop_set();}
            return last
        } else {
            last = interpret_expr(ele, varlist);
            if let Variable::Return(value) = last {
                if add_set {varlist.pop_set();}
                return if returnable {*value} else {Variable::Return(value)}
            }
        }
    }
    if add_set {varlist.pop_set();}
    last
}

pub fn interpret_asts(input: Vec<Element>) -> i32 {
    let mut varlist = Varstack::<Variable>::default_variable();
    for ele in input {
        if let Element::Return {value, position} = ele {
            let return_val = interpret_expr(*value, &mut varlist);
            if let Variable::I32(v) = return_val {return v}
            else {
                errors::error_pos(&position);
                errors::error_4_2(return_val);
            }
        } else {
            if let Variable::Return(value) = interpret_expr(ele.clone(), &mut varlist) {
                varlist.pop_set();
                if let Variable::I32(v) = *value {return v}
                else {
                    errors::error_pos(ele.get_pos());
                    errors::error_4_2(*value);
                }
            }}
    }
    0
}