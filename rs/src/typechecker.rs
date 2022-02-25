use std::collections::HashMap;
use crate::errors;
use crate::syntax::parsing::{Element, OprType};


fn typeof_(expr: &Element) -> Element {
    match expr {
        Element::Literal {type_, ..} => **type_,
        _ => Element::Variable {
            position: expr.get_pos().clone(),
            name: match expr {
                Element::BinaryOpr {type_, operand1, operand2, ..} => {
                    let Element::Variable {name: type1, ..} = typeof_(&**operand1);
                    let Element::Variable {name: type2, ..} = typeof_(&**operand2);
                    match (type_, &*type1, &*type2) { // This is a temporary setup for primitives before they become non-hardcoded
                        (OprType::Plus, "int", "int") | // Please do not mind
                        (OprType::Minus, "int", "int") => "int",
                        (OprType::Plus, "int", "double") |
                        (OprType::Plus, "double", "int") |
                        (OprType::Plus, "double", "double") |
                        (OprType::Minus, "int", "double") |
                        (OprType::Minus, "double", "int") |
                        (OprType::Minus, "double", "double") => "double",
                        _ => { // TODO more operators
                            errors::error_pos(expr.get_pos());
                            errors::error_4_0_0("TODO".to_string(), type1, type2)
                        }
                    }.to_string()
                } // TODO Element::UnaryOpr, Element::Call etc etc etc
                _ => "".to_string()
            },
            parent: Box::new(Element::NullElement)
        }
    }
}

fn typecheck(mut input: Vec<Element>) -> Vec<Element> {
    for ele in input.iter_mut() {
        if let Element::DeclarationStmt {position, variable, content, flags, type_} = ele.clone() {
            if type_ == Box::new(Element::NullElement) {
                *ele = Element::DeclarationStmt {
                    type_: Box::new(typeof_(&content)),
                    content,
                    variable,
                    position,
                    flags
                };
            } else {
                if typeof_(&content) != *type_ {
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