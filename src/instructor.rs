use crate::{
    ast::{Ast, AstData},
    types::{interpreter_data::InterpreterData, printer::Print, typeobj::Type},
    ZResult,
};

pub fn gen_instructions<O: Print>(
    mut input: Vec<Ast>,
    typelist: &mut InterpreterData<Type<Ast>, O>,
) -> ZResult<Vec<Ast>> {
    for ele in input.iter_mut() {
        *ele = ele.desugared(typelist.out)?;
    }
    for ele in input.iter_mut() {
        ele.process(typelist)?;
    }
    Ok(input)
}
