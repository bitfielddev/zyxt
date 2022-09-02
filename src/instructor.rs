use crate::types::element::Element;
use crate::types::frame_data::InterpreterData;
use crate::types::printer::Print;
use crate::types::typeobj::Type;
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
