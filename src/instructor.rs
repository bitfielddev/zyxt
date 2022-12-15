use crate::{
    ast::{Ast, AstData},
    types::{sym_table::SymTable, typeobj::Type},
    ZResult,
};

pub fn gen_instructions(
    mut input: Vec<Ast>,
    ty_symt: &mut SymTable<Type<Ast>>,
) -> ZResult<Vec<Ast>> {
    for ele in &mut input {
        *ele = ele.desugared()?;
    }
    for ele in &mut input {
        ele.process(ty_symt)?;
    }
    Ok(input)
}
