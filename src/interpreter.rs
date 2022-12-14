use crate::{
    ast::{block::Block, Ast},
    types::{interpreter_data::SymTable, position::Span, value::Value},
    ZError, ZResult,
};

pub fn interpret_asts(input: &Vec<Ast>, i_data: &mut SymTable<Value>) -> ZResult<i32> {
    let input = Block {
        brace_spans: None,
        content: input.to_owned(),
    };
    let mut last = input.interpret_block(i_data, true, false)?;
    while let Value::Return(v) = last {
        last = *v
    }
    if last == Value::Unit {
        last = Value::I32(0);
    }
    if let Value::I32(v) = last {
        Ok(v)
    } else {
        Err(ZError::error_4_2(last).with_span(&Span::default())) // TODO
    }
}
