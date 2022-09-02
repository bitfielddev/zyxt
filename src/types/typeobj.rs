use crate::types::element::Argument;
use crate::Element;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Instance {
        // str, bool, cpx<int> etc. Is of type Typedef
        name: String,
        type_args: Vec<Type>,
        inst_attrs: HashMap<String, Element>,
        implementation: Option<&'static Type>,
    },
    Definition {
        // class, struct, (anything that implements a Type). Is of type <type> (Typedef)
        name: String, // TODO inheritance
        generics: Vec<Argument>,
        class_attrs: HashMap<String, Element>,
        inst_attrs: HashMap<String, Element>,
    },
    Return(Box<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Type::Instance {
                    name, type_args, ..
                } =>
                    if !type_args.is_empty() {
                        format!(
                            "{}<{}>",
                            name,
                            type_args
                                .iter()
                                .map(|arg| format!("{}", arg))
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    } else {
                        name.to_string()
                    },
                Type::Definition { name, .. } => name.to_owned(),
                Type::Return(ty) => format!("{}", ty),
            }
        )
    }
}
impl Type {
    pub fn from_name(s: &str) -> Self {
        Type::Instance {
            name: s.to_string(),
            type_args: vec![],
            inst_attrs: Default::default(),
            implementation: None,
        }
    }
    pub fn null() -> Self {
        Type::from_name("_null")
    }
    pub fn any() -> Self {
        Type::from_name("_any")
    }
    pub fn as_element(&self) -> Element {
        match self {
            Type::Instance { name, .. } => Element::Ident {
                position: Default::default(),
                name: name.to_owned(), // TODO type args
                raw: self.to_string(),
                parent: Box::new(Element::NullElement),
            },
            Type::Definition { .. } => todo!(),
            Type::Return(ty) => ty.as_element(),
        }
    }
    pub fn get_attrs(&self) -> HashMap<String, Element> {
        match self {
            Type::Instance {
                implementation,
                inst_attrs,
                ..
            } => {
                let mut attrs = HashMap::new();
                for (key, value) in inst_attrs {
                    attrs.insert(key.to_owned(), value.to_owned());
                }
                if let Some(implementation) = implementation {
                    for (name, element) in implementation.get_attrs() {
                        attrs.insert(name, element);
                    }
                }
                attrs
            }
            Type::Definition {
                class_attrs,
                inst_attrs,
                ..
            } => {
                let mut attrs = HashMap::new();
                for (name, element) in class_attrs.iter() {
                    attrs.insert(name.to_owned(), element.to_owned());
                }
                for (name, element) in inst_attrs.iter() {
                    attrs.insert(name.to_owned(), element.to_owned());
                }
                attrs
            }
            Type::Return(ty) => ty.get_attrs(), // TODO get class from type, maybe?
        }
    }
}
