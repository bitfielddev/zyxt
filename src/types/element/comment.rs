use smol_str::SmolStr;

use crate::types::element::{Element, ElementData};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Comment {
    pub content: SmolStr,
}
