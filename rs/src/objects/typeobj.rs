use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use crate::Element;

#[derive(Clone, PartialEq, Debug)]
pub enum TypeObj {
    Prim {
        name: String,
        type_args: Vec<TypeObj>
    },
    Compound {
        class_attrs: HashMap<String, Element>,
        inst_attrs: HashMap<String, Element>,
    },
    //Type // todo
}

impl Display for TypeObj {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            TypeObj::Prim {name, type_args} =>
                if !type_args.is_empty() {
                    format!("{}<{}>", name,
                            type_args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(", "))
                } else {name.to_string()},
            TypeObj::Compound {..} => todo!()
        })
    }
}
impl TypeObj {
    pub fn from_str(s: &str) -> Self {TypeObj::Prim{name: s.to_string(), type_args: vec![]}}
    pub fn null() -> Self { TypeObj::from_str("#null") }
    pub fn any() -> Self { TypeObj::from_str("#any") }
    pub fn as_element(&self) -> Element {
        match self {
            TypeObj::Prim {name, ..} => Element::Variable {
                position: Default::default(),
                name: name.clone(), // TODO type args
                raw: self.to_string(),
                parent: Box::new(Element::NullElement)
            },
            TypeObj::Compound { .. } => todo!()
        }
    }
}
