use crate::{
    types::{element::Element, interpreter_data::InterpreterData, printer::Print, typeobj::Type},
    ZyxtError,
};

pub fn gen_instructions<O: Print>(
    mut input: Vec<Element>,
    typelist: &mut InterpreterData<Type, O>,
) -> Result<Vec<Element>, ZyxtError> {
    for ele in input.iter_mut() {
        ele.process(typelist)?;
    }
    Ok(input)
}
