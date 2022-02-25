use std::collections::HashMap;
use crate::errors;
use crate::lexer::Position;
use crate::syntax::parsing::{Element, OprType};

pub fn bin_op_return_type(type_: &OprType, type1: String, type2: String, position: &Position) -> String {
    match (type_, &*type1, &*type2) { // This is a temporary setup for primitives before they become non-hardcoded
        (OprType::Plus, "i32", "i32") | // Don't mind me
        (OprType::Minus, "i32", "i32") => "i32",
        (OprType::Plus, "i32", "f64") |
        (OprType::Plus, "f64", "i32") |
        (OprType::Plus, "f64", "f64") |
        (OprType::Minus, "i32", "f64") |
        (OprType::Minus, "f64", "i32") |
        (OprType::Minus, "f64", "f64") => "f64",
        _ => { // TODO more operators
            errors::error_pos(position);
            errors::error_4_0_0("TODO".to_string(), type1, type2)
        }
    }.to_string()
}

pub fn un_op_return_type(type_: &OprType, opnd_type: String, position: &Position) -> String {
    match (type_, &*opnd_type) {
        (OprType::PlusSign, "i32") |
        (OprType::MinusSign, "i32") => "i32",
        (OprType::PlusSign, "f64") |
        (OprType::MinusSign, "f64") => "f64",
        _ => {
            errors::error_pos(position);
            errors::error_4_0_1("TODO".to_string(), opnd_type)
        }
    }.to_string()
}



pub fn typecheck(mut input: Vec<Element>) -> Vec<Element> {
    for ele in input.iter_mut() {
        if let Element::DeclarationStmt {position, variable, content, flags, type_} = ele.clone() {
            if type_ == Box::new(Element::NullElement) {
                *ele = Element::DeclarationStmt {
                    type_: Box::new(content.get_type()),
                    content,
                    variable,
                    position,
                    flags
                };
            } else {
                if content.get_type() != *type_ {
                    *ele = Element::DeclarationStmt {
                        type_: type_.clone(),
                        content: Box::new(Element::Call {
                            position: position.clone(),
                            called: Box::new(Element::Variable {
                                position: position.clone(),
                                name: "to".to_string(),
                                parent: content,
                            }),
                            args: vec![*type_],
                            kwargs: Box::new(HashMap::new())
                        }),
                        variable,
                        position,
                        flags
                    };
                }
            }
        }
    }
    input
}