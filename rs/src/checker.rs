use std::collections::HashMap;
use crate::errors;
use crate::interpreter::Variable;
use crate::lexer::Position;
use crate::syntax::element::Element;
use crate::syntax::token::OprType;

pub fn bin_op_return_type(type_: &OprType, type1: String, type2: String, position: &Position) -> String {
    if type_ == &OprType::TypeCast {
        return type2
    }
    if let Some(v) = Variable::default(type1.clone())
        .bin_opr(type_, Variable::default(type2.clone())) {
        return v.get_type_name()
    } else {
        errors::error_pos(position);
        errors::error_4_0_0(type_.to_string(), type1, type2)
    }
}

pub fn un_op_return_type(type_: &OprType, opnd_type: String, position: &Position) -> String {
    if let Some(v) = Variable::default(opnd_type.clone()).un_opr(type_) {
        return v.get_type_name()
    } else{
        errors::error_pos(position);
        errors::error_4_0_1(type_.to_string(), opnd_type)
    }
}

pub fn check(mut input: Vec<Element>) -> Vec<Element> {
    let mut typelist: HashMap<String, Element> = HashMap::new();
    for t in ["str", "i32", "f64", "#null", "type"] {
        typelist.insert(t.to_string(), Element::Variable {
            position: Default::default(),
            name: "type".to_string(),
            parent: Box::new(Element::NullElement)
        });
    }
    for ele in input.iter_mut() {
        if let Element::DeclarationStmt {position, variable, content, flags, type_} = ele.clone() {
            let content_type = content.get_type(&typelist);
            if type_ == Box::new(Element::NullElement) {
                typelist.insert(variable.get_name(), content_type.clone());
                *ele = Element::DeclarationStmt {
                    type_: Box::new(content_type),
                    content,
                    variable,
                    position,
                    flags
                };
            } else {
                typelist.insert(variable.get_name(), *type_.clone());
                if content.get_type(&typelist) != *type_ {
                    *ele = Element::DeclarationStmt {
                        type_: type_.clone(),
                        content: Box::new(Element::BinaryOpr {
                            position: position.clone(),
                            type_: OprType::TypeCast,
                            operand1: content,
                            operand2: Box::new(*type_)
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