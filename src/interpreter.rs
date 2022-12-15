use crate::{
    ast::{Ast, Block},
    primitives::I32_T,
    types::{position::Span, sym_table::SymTable, value::Value},
    ZError, ZResult,
};

pub fn interpret_asts(input: &Vec<Ast>, val_symt: &mut SymTable<Value>) -> ZResult<i32> {
    let input = Block {
        brace_spans: None,
        content: input.to_owned(),
    };
    let mut last = input.interpret_block(val_symt, true, false)?;
    while let Value::Return(v) = last {
        last = *v;
    }
    if last == Value::Unit {
        last = Value::I32(0);
    }
    if let Value::I32(v) = last {
        Ok(v)
    } else {
        Err(ZError::t009(&I32_T.as_type(), &last.get_type_obj()).with_span(&Span::default()))
        // TODO
    }
}
