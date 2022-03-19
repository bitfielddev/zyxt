use crate::objects::varstack::Varstack;
use crate::objects::element::Element;
use crate::objects::typeobj::TypeObj;
use crate::ZyxtError;

pub fn gen_instructions(mut input: Vec<Element>, typelist: &mut Varstack<TypeObj>) -> Result<Vec<Element>, ZyxtError> {
    for ele in input.iter_mut() {
        ele.get_type(typelist)?;
    }
    Ok(input)
}