use crate::Element;

pub enum TypeObj {
    Prim {
        name: String,
        type_args: Vec<TypeObj>
    },
    Compound {
        name: String,
        type_args: Vec<TypeObj>
    }
}