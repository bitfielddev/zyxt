use tracing::debug;

use crate::{
    ast::{Ast, AstData, Reconstruct},
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
    debug!("{}", input.reconstruct());
    for ele in &mut input {
        ele.process(ty_symt)?;
    }
    Ok(input)
}
