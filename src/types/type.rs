use std::{
    collections::HashMap,
    fmt::{Debug, Display, Formatter},
};

use itertools::Itertools;
use smol_str::SmolStr;

use crate::{
    ast::{Argument, Ast, AstData},
    primitives::{TYPE_T, TYPE_T_ELE, UNIT_T, UNIT_T_ELE},
    SymTable, Value, ZResult,
};

#[derive(Clone, PartialEq)]
pub struct TypeDefinition<T: Clone + PartialEq + Debug> {
    // class, struct, (anything that implements a Type). Is of type <type> (Typedef)
    pub inst_name: Option<SmolStr>, // TODO inheritance
    pub name: Option<SmolStr>,
    pub generics: Vec<Argument>,
    pub implementations: HashMap<SmolStr, T>,
    pub inst_fields: HashMap<SmolStr, (Box<Type<T>>, Option<T>)>,
}

#[derive(Clone, PartialEq)]
pub struct TypeInstance<T: Clone + PartialEq + Debug> {
    // str, bool, cpx<int> etc. Is of type Typedef
    pub name: Option<SmolStr>,
    pub type_args: Vec<Type<T>>,
    pub implementation: TypeDefinition<T>,
}

#[derive(Clone, PartialEq)]
pub enum Type<T: Clone + PartialEq + Debug> {
    Instance(TypeInstance<T>),
    Definition(TypeDefinition<T>),
    Any,
    Return(Box<Type<T>>),
}
impl<T: Clone + PartialEq + Debug> Debug for TypeInstance<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self} (implementation: {:?})", self.implementation)
    }
}
impl<T: Clone + PartialEq + Debug> Debug for TypeDefinition<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} for {} (implementations: {{{}}}; fields: {{{}}})",
            self,
            self.inst_name
                .to_owned()
                .unwrap_or_else(|| "{unknown}".into()),
            self.implementations.keys().join(", "),
            self.inst_fields.keys().join(", ")
        )
    }
}
impl<T: Clone + PartialEq + Debug> Debug for Type<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Instance(inst) => {
                write!(f, "{inst:?}")
            }
            Self::Definition(def) => {
                write!(f, "{def:?}")
            }
            Self::Any => write!(f, "_any"),
            Self::Return(t) => <Self as Debug>::fmt(t, f),
        }
    }
}
impl<T: Clone + PartialEq + Debug> Display for TypeDefinition<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.name.to_owned().unwrap_or_else(|| "{unknown}".into())
        )
    }
}
impl<T: Clone + PartialEq + Debug> Display for TypeInstance<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}<{}>",
            self.name.as_ref().unwrap_or(&"{unknown}".into()),
            self.type_args
                .iter()
                .map(|arg| format!("{arg}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<T: Clone + PartialEq + Debug> Display for Type<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Instance(inst) => inst.to_string(),
                Self::Definition(def) => def.to_string(),
                Self::Any => "_any".into(),
                Self::Return(ty) => ty.to_string(),
            }
        )
    }
}

impl TypeDefinition<Ast> {
    pub fn get_instance(&self) -> Type<Ast> {
        if *self == TYPE_T.as_type_element() {
            Type::Definition(TYPE_T.as_type_element())
        } else {
            Type::Instance(TypeInstance {
                name: self.inst_name.to_owned(),
                type_args: vec![],
                implementation: self.to_owned(),
            })
        }
    }
}
impl TypeDefinition<Value> {
    pub fn get_instance(&self) -> Type<Value> {
        if *self == *TYPE_T {
            Type::Definition(TYPE_T.to_owned())
        } else {
            Type::Instance(TypeInstance {
                name: self.inst_name.to_owned(),
                type_args: vec![],
                implementation: self.to_owned(),
            })
        }
    }
}

impl<T: Clone + PartialEq + Debug> TypeDefinition<T> {
    #[must_use]
    pub fn as_type(&self) -> Type<T> {
        Type::Definition(self.to_owned())
    }
}
impl<T: Clone + PartialEq + Debug> TypeInstance<T> {
    #[must_use]
    pub fn as_type(&self) -> Type<T> {
        Type::Instance(self.to_owned())
    }
}

impl TypeDefinition<Ast> {
    pub fn as_type_value(&self, val_symt: &mut SymTable<Value>) -> ZResult<TypeDefinition<Value>> {
        Ok(TypeDefinition {
            inst_name: self.inst_name.to_owned(),
            name: self.name.to_owned(),
            generics: self.generics.to_owned(),
            implementations: self
                .implementations
                .iter()
                .map(|(k, v)| Ok((k.to_owned(), v.interpret_expr(val_symt)?)))
                .collect::<Result<HashMap<_, _>, _>>()?,
            inst_fields: self
                .inst_fields
                .iter()
                .map(|(k, (v1, v2))| {
                    Ok((
                        k.to_owned(),
                        (
                            Box::new(v1.as_type_value(val_symt)?),
                            v2.to_owned()
                                .map(|v2| v2.interpret_expr(val_symt))
                                .transpose()?,
                        ),
                    ))
                })
                .collect::<Result<HashMap<_, _>, _>>()?,
        })
    }
}
impl TypeInstance<Ast> {
    pub fn as_type_value(&self, val_symt: &mut SymTable<Value>) -> ZResult<TypeInstance<Value>> {
        Ok(TypeInstance {
            name: self.name.to_owned(),
            type_args: self
                .type_args
                .iter()
                .map(|a| a.as_type_value(val_symt))
                .collect::<Result<Vec<_>, _>>()?,
            implementation: self.implementation.as_type_value(val_symt)?,
        })
    }
}
impl TypeInstance<Value> {
    #[must_use]
    pub fn as_type_element(&self) -> TypeInstance<Ast> {
        TypeInstance {
            name: self.name.to_owned(),
            type_args: self.type_args.iter().map(Type::as_type_element).collect(),
            implementation: self.implementation.as_type_element(),
        }
    }
}
impl TypeDefinition<Value> {
    #[must_use]
    pub fn as_type_element(&self) -> TypeDefinition<Ast> {
        TypeDefinition {
            inst_name: self.inst_name.to_owned(),
            name: self.name.to_owned(),
            generics: self.generics.to_owned(),
            implementations: self
                .implementations
                .iter()
                .map(|(k, v)| (k.to_owned(), v.as_ast()))
                .collect(),
            inst_fields: self
                .inst_fields
                .iter()
                .map(|(k, (v1, v2))| {
                    (
                        k.to_owned(),
                        (
                            Box::new(v1.as_type_element()),
                            v2.to_owned().map(|v2| v2.as_ast()),
                        ),
                    )
                })
                .collect(),
        }
    }
}

impl Type<Ast> {
    #[must_use]
    pub fn as_literal(&self) -> Ast {
        Value::PreType(self.to_owned()).as_ast()
    }
    #[must_use]
    pub fn implementation(&self) -> &TypeDefinition<Ast> {
        match &self {
            Self::Instance(TypeInstance { implementation, .. }) => implementation,
            Self::Definition { .. } => &TYPE_T_ELE,
            Self::Any => &UNIT_T_ELE,
            Self::Return(ty) => ty.implementation(),
        }
    }
    pub fn as_type_value(&self, val_symt: &mut SymTable<Value>) -> ZResult<Type<Value>> {
        Ok(match &self {
            Self::Instance(inst) => Type::Instance(inst.as_type_value(val_symt)?),
            Self::Definition(def) => Type::Definition(def.as_type_value(val_symt)?),
            Self::Any => Type::Any,
            Self::Return(t) => Type::Return(Box::new(t.as_type_value(val_symt)?)),
        })
    }
}

impl Type<Value> {
    #[must_use]
    pub fn implementation(&self) -> &TypeDefinition<Value> {
        match &self {
            Self::Instance(TypeInstance { implementation, .. }) => implementation,
            Self::Definition { .. } => &TYPE_T,
            Self::Any => &UNIT_T,
            Self::Return(ty) => ty.implementation(),
        }
    }
    #[must_use]
    pub fn as_type_element(&self) -> Type<Ast> {
        match &self {
            Self::Instance(inst) => Type::Instance(inst.as_type_element()),
            Self::Definition(def) => Type::Definition(def.as_type_element()),
            Self::Any => Type::Any,
            Self::Return(t) => Type::Return(Box::new(t.as_type_element())),
        }
    }
}
