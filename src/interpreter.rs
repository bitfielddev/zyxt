use crate::{
    types::{
        element::{block::Block, Element},
        interpreter_data::{FrameType, InterpreterData},
        printer::Print,
        token::OprType,
        typeobj::TypeDefinition,
        value::{Proc, Value},
    },
    Type, ZyxtError,
};

pub fn interpret_asts<O: Print>(
    input: &Vec<Element>,
    i_data: &mut InterpreterData<Value, O>,
) -> Result<i32, ZyxtError> {
    let input = Block {
        content: input.to_owned(),
    };
    let mut last = input.interpret_block(i_data, true, false)?;
    while let Value::Return(v) = last {
        last = *v
    }
    if last == Value::Unit {
        last == Value::I32(0);
    }
    return if let Value::I32(v) = *last {
        Ok(v)
    } else {
        Err(ZyxtError::error_4_2(*last).with_pos_raw(Default::default())) // TODO
    };
}
