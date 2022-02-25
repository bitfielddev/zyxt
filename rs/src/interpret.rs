use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::syntax::parsing::{Element, OprType};

#[derive(Clone)]
enum Variable {
    I32(i32),
    F64(f64),
}

impl Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Variable::I32(v) => v.to_string(), Variable::F64(v) => v.to_string()
        })
    }
}

impl Variable {
    pub fn un_opr(&self, type_: &OprType) -> Variable {
        match type_ { // will prob clean this up with macros
            OprType::PlusSign => match *self {
                Variable::I32(v) => Variable::I32(v),
                Variable::F64(v) => Variable::F64(v)
            },
            OprType::MinusSign => match *self {
                Variable::I32(v) => Variable::I32(-v),
                Variable::F64(v) => Variable::F64(-v)
            },
            _ => panic!()
        }
    }
    pub fn bin_opr(&self, type_: &OprType, other: Variable) -> Variable {
        match type_ { // will prob clean this up with macros
            OprType::Plus => match *self {
                Variable::I32(v1) => match other {
                    Variable::I32(v2) => Variable::I32(v1+v2),
                    Variable::F64(v2) => Variable::F64(v1 as f64+v2),
                },
                Variable::F64(v1) => match other {
                    Variable::I32(v2) => Variable::F64(v1+v2 as f64),
                    Variable::F64(v2) => Variable::F64(v1+v2),
                }
            },
            OprType::Minus => match *self {
                Variable::I32(v1) => match other {
                    Variable::I32(v2) => Variable::I32(v1-v2),
                    Variable::F64(v2) => Variable::F64(v1 as f64-v2),
                },
                Variable::F64(v1) => match other {
                    Variable::I32(v2) => Variable::F64(v1-v2 as f64),
                    Variable::F64(v2) => Variable::F64(v1-v2),
                }
            },
            _ => panic!()
        }
    }
    pub fn from_type_content(type_: Element, content: String) -> Variable {
        match &*type_.get_name() {
            "i32" => Variable::I32(content.parse::<i32>().unwrap()),
            "f64" => Variable::F64(content.parse::<f64>().unwrap()),
            _ => panic!()
        }
    }
}

fn interpret_expr(input: Element, varlist: &mut HashMap<String, Variable>) -> Variable {
    match input {
        Element::NullElement | Element::Token(..) | Element::Comment {..} => panic!(),
        Element::UnaryOpr {type_, operand, ..} =>
            interpret_expr(*operand, varlist).un_opr(&type_),
        Element::BinaryOpr {type_, operand1, operand2, ..} =>
            interpret_expr(*operand1, varlist).bin_opr(&type_, interpret_expr(*operand2, varlist)),
        Element::Variable {name, ..} => (*varlist.get(&*name).unwrap()).clone(),
        Element::DeclarationStmt {variable, content, ..} => {
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
                    println!("{}", varlist.get(&*args[0].get_name()).unwrap());
                    Variable::I32(0)
                } else {panic!()}
            } else {panic!()}
        }
    }
}

pub fn interpret_asts(input: Vec<Element>) {
    let mut varlist: HashMap<String, Variable> = HashMap::new();
    for ele in input {interpret_expr(ele, &mut varlist);}
}