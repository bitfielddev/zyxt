use std::sync::Arc;

use crate::{
    ast::{Ast, Block},
    errors::ToZResult,
    primitives::I32_T,
    types::{
        position::Span,
        sym_table::{InterpretFrameType, InterpretSymTable},
        value::Value,
    },
    ZError, ZResult,
};

pub fn interpret_asts(input: &Vec<Ast>, val_symt: &mut InterpretSymTable) -> ZResult<i32> {
    let input = Block {
        brace_spans: None,
        content: input.to_owned(),
    };
    let mut last = input.interpret_block(val_symt, true, true)?;
    while let Value::Return(v) = last {
        last = *v;
    }
    if last == Value::Unit {
        last = Value::I32(0);
    }
    last.as_i32().copied().z()
}
