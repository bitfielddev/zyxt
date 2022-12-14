use crate::{
    ast::{Ast, AstData},
    types::{interpreter_data::InterpreterData, typeobj::Type},
    ZResult,
};

pub fn gen_instructions(
    mut input: Vec<Ast>,
    typelist: &mut InterpreterData<Type<Ast>>,
) -> ZResult<Vec<Ast>> {
    for ele in input.iter_mut() {
        *ele = ele.desugared()?;
    }
    for ele in input.iter_mut() {
        ele.process(typelist)?;
    }
    Ok(input)
}
