use crate::{
    types::{element::Element, interpreter_data::InterpreterData, printer::Print, typeobj::Type},
    ZResult,
};

pub fn gen_instructions<O: Print>(
    mut input: Vec<Element>,
    typelist: &mut InterpreterData<Type<Element>, O>,
) -> ZResult<Vec<Element>> {
    for ele in input.iter_mut() {
        *ele = ele.desugared(typelist.out)?;
    }
    for ele in input.iter_mut() {
        ele.process(typelist)?;
    }
    Ok(input)
}
