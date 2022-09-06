pub mod bool_t;
pub mod f16_t;
pub mod f32_t;
pub mod f64_t;
pub mod i128_t;
pub mod i16_t;
pub mod i32_t;
pub mod i64_t;
pub mod i8_t;
pub mod ibig_t;
pub mod isize_t;
pub mod macros;
pub mod str_t;
pub mod type_t;
pub mod u128_t;
pub mod u16_t;
pub mod u32_t;
pub mod u64_t;
pub mod u8_t;
pub mod ubig_t;
pub mod unit_t;
pub mod usize_t;
pub mod proc_t;

use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use smol_str::SmolStr;

use crate::{
    types::{
        element::Argument,
        typeobj::{type_t::TYPE_T, unit_t::UNIT_T},
    },
    Element,
};

#[derive(Clone, PartialEq, Debug)]
pub enum Type<T: Clone + PartialEq + Debug> {
    Instance {
        // str, bool, cpx<int> etc. Is of type Typedef
        name: Option<SmolStr>,
        type_args: Vec<Type<T>>,
        implementation: Box<Type<T>>,
    },
    Definition {
        // class, struct, (anything that implements a Type). Is of type <type> (Typedef)
        inst_name: Option<SmolStr>, // TODO inheritance
        name: Option<SmolStr>,
        generics: Vec<Argument>,
        implementations: HashMap<SmolStr, T>,
        inst_fields: HashMap<SmolStr, (Box<Type<T>>, Option<T>)>,
    },
    Any,
    Return(Box<Type<T>>),
}

impl<T: Clone + PartialEq + Debug> Display for Type<T> {
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
                            name.unwrap_or_else(|| "{unknown}".into()),
                            type_args
                                .iter()
                                .map(|arg| format!("{}", arg))
                                .collect::<Vec<String>>()
                                .join(", ")
                        )
                    } else {
                        name.to_string()
                    },
                Type::Definition { name, .. } => name.to_owned().into(),
                Type::Any => "_any".into(),
                Type::Return(ty) => format!("{}", ty),
            }
        )
    }
}
impl<T: Clone + PartialEq + Debug> Type<T> {
    pub fn as_element(&self) -> Element {
        match self {
            Type::Instance { name, .. } => Element::Ident {
                position: Default::default(),
                name: name.to_owned().unwrap_or_default(), // TODO type args
                raw: self.to_string(),
                parent: Box::new(Element::NullElement),
            },
            Type::Definition { .. } => todo!(),
            Type::Any => todo!(),
            Type::Return(ty) => ty.as_element(),
        }
    }
    pub fn implementation(&self) -> &Type<T> {
        match &self {
            Type::Instance { implementation, .. } => implementation,
            Type::Definition { .. } => &TYPE_T,
            Type::Any => &UNIT_T,
            Type::Return(ty) => ty.implementation(),
        }
    }
}
