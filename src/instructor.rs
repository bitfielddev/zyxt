use crate::{
    ast::{Ast, AstData},
    types::{interpreter_data::SymTable, typeobj::Type},
    ZResult,
};

pub fn gen_instructions(
    mut input: Vec<Ast>,
    typelist: &mut SymTable<Type<Ast>>,
) -> ZResult<Vec<Ast>> {
    for ele in &mut input {
        *ele = ele.desugared()?;
    }
    for ele in &mut input {
        ele.process(typelist)?;
    }
    Ok(input)
}
