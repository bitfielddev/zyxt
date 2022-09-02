use crate::{
    types::{element::Element, frame_data::InterpreterData, printer::Print, typeobj::Type},
    ZyxtError,
};

pub fn gen_instructions<O: Print>(
    mut input: Vec<Element>,
    typelist: &mut InterpreterData<Type, O>,
) -> Result<Vec<Element>, ZyxtError> {
    for ele in input.iter_mut() {
        ele.eval_type(typelist)?;
    }
    Ok(input)
}
