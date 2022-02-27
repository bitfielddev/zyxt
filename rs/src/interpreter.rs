use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors;
use crate::syntax::element::Element;
use crate::syntax::token::OprType;

#[derive(Clone)]
pub enum Variable {
    I32(i32),
    F64(f64),
    Str(String),
    Bool(bool),
    Type(String),
    Null
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
            Variable::Type(v) => "<".to_owned()+&**v+">",
            Variable::Null => "null".to_string()
        }
    }
    pub fn un_opr(&self, type_: &OprType) -> Option<Variable> {
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
                Variable::Type(t) => match &*t {
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
                        Variable::Null => Some(Variable::Bool(false))
                    }
                    "type" => Some(self.get_type()),
                    _ => None
                },
                _ => None
            }
            _ => None
        }
    }
    pub fn default(type_: String) -> Self {
        match &*type_ {
            "i32" => Variable::I32(0),
            "f64" => Variable::F64(0.0),
            "str" => Variable::Str("".to_string()),
            "bool" => Variable::Bool(false),
            "#null" => Variable::Null,
            "type" => Variable::Type("#null".to_string()),
            _ => panic!("{}", type_)
        }
    }
    pub fn from_type_content(type_: Element, content: String) -> Variable {
        match &*type_.get_name() {
            "i32" => Variable::I32(content.parse::<i32>().unwrap()),
            "f64" => Variable::F64(content.parse::<f64>().unwrap()),
            "str" => Variable::Str(content),
            "bool" => Variable::Bool(&*content == "true"),
            _ => panic!()
        }
    }
    pub fn get_type_name(&self) -> String {
        match self {
            Variable::I32(..) => "i32",
            Variable::F64(..) => "f64",
            Variable::Str(..) => "str",
            Variable::Bool(..) => "bool",
            Variable::Type(..) => "type",
            Variable::Null => "#null"
        }.to_string()
    }
    pub fn get_type(&self) -> Variable {
        Variable::Type(self.get_type_name())
    }
}

fn interpret_expr(input: Element, varlist: &mut HashMap<String, Variable>) -> Variable {
    match input {
        Element::NullElement | Element::Token(..) | Element::Comment {..} => panic!(),
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
        Element::Variable {name, ..} => (*varlist.get(&*name).unwrap()).clone(),
        Element::Declare {variable, content, ..} |
        Element::Set {variable, content, ..} => {
            let var = interpret_expr(*content, varlist);
            varlist.insert(variable.get_name(), var);
            (*varlist.get(&*variable.get_name()).unwrap()).clone()
        },
        Element::Literal {type_, content, ..} => {
            Variable::from_type_content(*type_, content)
        },
        Element::Call {called, args, ..} => {
            if let Element::Variable {parent, name, ..} = *called {
                if name == "println".to_string() && parent.get_name() == "std".to_string() {
                    println!("{}", args.into_iter().
                        map(|arg| interpret_expr(arg, varlist).get_displayed_value())
                        .collect::<Vec<String>>().join(" "));
                    Variable::Null
                } else {panic!()}
            } else {panic!()}
        }
    }
}

pub fn interpret_asts(input: Vec<Element>) {
    let mut varlist: HashMap<String, Variable> = HashMap::new();
    for t in ["str", "i32", "f64", "#null", "type"] {
        varlist.insert(t.to_string(), Variable::Type(t.to_string()));
    }
    for ele in input {interpret_expr(ele, &mut varlist);}
}