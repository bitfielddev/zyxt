use crate::objects::stack::Stack;
use crate::objects::element::Element;
use crate::objects::typeobj::Type;
use crate::ZyxtError;

pub fn gen_instructions(mut input: Vec<Element>, typelist: &mut Stack<Type>) -> Result<Vec<Element>, ZyxtError> {
    for ele in input.iter_mut() {
        ele.eval_type(typelist)?;
    }
    Ok(input)
}