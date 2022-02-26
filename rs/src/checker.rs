use std::collections::HashMap;
use crate::syntax::element::Element;
use crate::syntax::token::OprType;

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