use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use crate::Element;
use crate::objects::element::Argument;

#[derive(Clone, PartialEq, Debug)]
pub enum TypeObj {
    Type { // str, bool, cpx<int> etc, is of type Typedef
        name: String,
        type_args: Vec<TypeObj>,
        implementation: &'static TypeObj
    },
    Typedef { // class, struct, (anything that implements a Type), is of type <type> (Typedef)
        name: String,
        generics: Vec<Argument>,
        class_attrs: HashMap<String, Element>,
        inst_attrs: HashMap<String, Element>,
    },
    //Type // todo
}

impl Display for TypeObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TypeObj::Type {name, type_args, ..} =>
                if !type_args.is_empty() {
                    format!("{}<{}>", name,
                            type_args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(", "))
                } else {name.to_string()},
            TypeObj::Typedef {..} => todo!()
        })
    }
}
impl TypeObj {
    pub fn from_str(s: &str) -> Self {TypeObj::Type {name: s.to_string(), type_args: vec![], implementation: match s {
        "#null" => TypeObj::null_impl(),
        "#any" => TypeObj::any_impl(),
        "#num" => TypeObj::num_impl(),
        "bool" => TypeObj::bool_impl(),
        _ => panic!()
    }}}
    pub fn null() -> Self { TypeObj::from_str("#null") }
    pub fn any() -> Self { TypeObj::from_str("#any") }
    pub fn null_impl() -> &'static Self { &TypeObj::Typedef {
        name: "class".to_string(),
        generics: vec![],
        class_attrs: Default::default(),
        inst_attrs: Default::default()
    } }
    pub fn any_impl() -> &'static Self { &TypeObj::Typedef {
        name: "class".to_string(),
        generics: vec![],
        class_attrs: Default::default(),
        inst_attrs: Default::default()
    } }
    pub fn num_impl() -> &'static Self { &TypeObj::Typedef {
        name: "class".to_string(),
        generics: vec![],
        class_attrs: Default::default(),
        inst_attrs: Default::default()
    } }
    pub fn bool_impl() -> &'static Self { &TypeObj::Typedef {
        name: "class".to_string(),
        generics: vec![],
        class_attrs: Default::default(),
        inst_attrs: Default::default()
    } }
    pub fn proc_impl() -> &'static Self { &TypeObj::Typedef {
        name: "class".to_string(),
        generics: vec![],
        class_attrs: Default::default(),
        inst_attrs: Default::default()
    } }
    pub fn fn_impl() -> &'static Self { &TypeObj::Typedef {
        name: "class".to_string(),
        generics: vec![],
        class_attrs: Default::default(),
        inst_attrs: Default::default()
    } }
    pub fn as_element(&self) -> Element {
        match self {
            TypeObj::Type {name, ..} => Element::Variable {
                position: Default::default(),
                name: name.clone(), // TODO type args
                raw: self.to_string(),
                parent: Box::new(Element::NullElement)
            },
            TypeObj::Typedef { .. } => todo!()
        }
    }
}
