use crate::objects::varstack::Stack;
use crate::objects::element::Element;
use crate::objects::typeobj::TypeObj;
use crate::ZyxtError;

pub fn gen_instructions(mut input: Vec<Element>, typelist: &mut Stack<TypeObj>) -> Result<Vec<Element>, ZyxtError> {
    for ele in input.iter_mut() {
        ele.eval_type(typelist)?;
    }
    Ok(input)
}