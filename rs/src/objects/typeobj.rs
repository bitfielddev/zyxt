use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use crate::Element;
use crate::objects::element::Argument;

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Instance { // str, bool, cpx<int> etc. Is of type Typedef
        name: String,
        type_args: Vec<Type>,
        implementation: Option<&'static Type>
    },
    Definition { // class, struct, (anything that implements a Type). Is of type <type> (Typedef)
        name: String,
        generics: Vec<Argument>,
        class_attrs: HashMap<String, Element>,
        inst_attrs: HashMap<String, Element>,
    },
    //Type // todo
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Type::Instance {name, type_args, ..} =>
                if !type_args.is_empty() {
                    format!("{}<{}>", name,
                            type_args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(", "))
                } else {name.to_string()},
            Type::Definition {name, ..} => name.clone()
        })
    }
}
impl Type {
    pub fn from_str(s: &str) -> Self { Type::Instance {name: s.to_string(), type_args: vec![], implementation: None}}
    pub fn null() -> Self { Type::from_str("#null") }
    pub fn any() -> Self { Type::from_str("#any") }
    pub fn as_element(&self) -> Element {
        match self {
            Type::Instance {name, ..} => Element::Variable {
                position: Default::default(),
                name: name.clone(), // TODO type args
                raw: self.to_string(),
                parent: Box::new(Element::NullElement)
            },
            Type::Definition { .. } => todo!()
        }
    }
}
