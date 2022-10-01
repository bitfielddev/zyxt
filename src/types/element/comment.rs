use smol_str::SmolStr;

use crate::{
    types::element::{Element, ElementData, ElementVariants},
    InterpreterData, Print, Type, Value, ZyxtError,
};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Comment {
    content: SmolStr,
}

impl ElementData for Comment {
    fn as_variant(&self) -> ElementVariants {
        ElementVariants::Comment(self.to_owned())
    }
}
