use crate::{
    types::{
        element::Element,
        interpreter_data::InterpreterData,
        printer::Print,
        token::{OprType, Token},
        typeobj::{
            bool_t::BOOL_T, proc_t::PROC_T, type_t::TYPE_T, unit_t::UNIT_T, Type, TypeDefinition,
            TypeInstance,
        },
        value::Proc,
    },
    Value, ZyxtError,
};

pub fn gen_instructions<O: Print>(
    mut input: Vec<Element>,
    typelist: &mut InterpreterData<Type<Element>, O>,
) -> Result<Vec<Element>, ZyxtError> {
    for ele in input.iter_mut() {
        *ele = ele.desugared(typelist.out)?;
    }
    for ele in input.iter_mut() {
        ele.process(typelist)?;
    }
    Ok(input)
}
