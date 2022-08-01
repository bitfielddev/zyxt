use crate::objects::element::Element;
use crate::objects::interpreter_data::{InterpreterData, Print};
use crate::objects::typeobj::Type;
use crate::ZyxtError;

pub fn gen_instructions<O: Print>(
    mut input: Vec<Element>,
    typelist: &mut InterpreterData<Type, O>,
) -> Result<Vec<Element>, ZyxtError> {
    for ele in input.iter_mut() {
        ele.eval_type(typelist)?;
    }
    Ok(input)
}
