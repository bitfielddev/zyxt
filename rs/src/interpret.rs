use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::syntax::element::Element;
use crate::syntax::token::OprType;

#[derive(Clone)]
enum Variable {
    I32(i32),
    F64(f64),
    Str(String),
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
            Variable::Null => "null".to_string()
        }
    }
    pub fn un_opr(&self, type_: &OprType) -> Variable {
        match type_ { // will prob clean this up with macros
            OprType::PlusSign => match *self {
                Variable::I32(v) => Variable::I32(v),
                Variable::F64(v) => Variable::F64(v),
                _ => panic!()
            },
            OprType::MinusSign => match *self {
                Variable::I32(v) => Variable::I32(-v),
                Variable::F64(v) => Variable::F64(-v),
                _ => panic!()
            },
            _ => panic!()
        }
    }
    pub fn bin_opr(&self, type_: &OprType, other: Variable) -> Variable {
        match type_ { // will prob clean this up with macros
            OprType::Plus => match self {
                Variable::I32(v1) => match other {
                    Variable::I32(v2) => Variable::I32(v1+v2),
                    Variable::F64(v2) => Variable::F64(*v1 as f64+v2),
                    _ => panic!()
                },
                Variable::F64(v1) => match other {
                    Variable::I32(v2) => Variable::F64(v1+v2 as f64),
                    Variable::F64(v2) => Variable::F64(v1+v2),
                    _ => panic!()
                },
                _ => panic!()
            },
            OprType::Minus => match self {
                Variable::I32(v1) => match other {
                    Variable::I32(v2) => Variable::I32(v1-v2),
                    Variable::F64(v2) => Variable::F64(*v1 as f64-v2),
                    _ => panic!()
                },
                Variable::F64(v1) => match other {
                    Variable::I32(v2) => Variable::F64(v1-v2 as f64),
                    Variable::F64(v2) => Variable::F64(v1-v2),
                    _ => panic!()
                },
                _ => panic!()
            },
            OprType::Concat => match self {
                Variable::I32(v1) => match other {
                    Variable::I32(v2) => Variable::I32((v1.to_string()+&*v2.to_string()).parse::<i32>().unwrap()),
                    Variable::F64(v2) => Variable::F64((v1.to_string()+&*v2.to_string()).parse::<f64>().unwrap()),
                    Variable::Str(v2) => Variable::Str(v1.to_string()+&*v2),
                    _ => panic!()
                },
                Variable::F64(v1) => match other {
                    Variable::I32(v2) => Variable::F64((v1.to_string()+&*v2.to_string()).parse::<f64>().unwrap()),
                    Variable::Str(v2) => Variable::Str(v1.to_string()+&*v2),
                    _ => panic!()
                },
                Variable::Str(v1) => match other {
                    Variable::I32(v2) => Variable::Str(v1.to_string()+&*v2.to_string()),
                    Variable::F64(v2) => Variable::Str(v1.to_string()+&*v2.to_string()),
                    Variable::Str(v2) => Variable::Str(v1.to_string()+&*v2),
                    _ => panic!()
                },
                _ => panic!()
            },
            _ => panic!()
        }
    }
    pub fn from_type_content(type_: Element, content: String) -> Variable {
        match &*type_.get_name() {
            "i32" => Variable::I32(content.parse::<i32>().unwrap()),
            "f64" => Variable::F64(content.parse::<f64>().unwrap()),
            "str" => Variable::Str(content),
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
    for ele in input {interpret_expr(ele, &mut varlist);}
}