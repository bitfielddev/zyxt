use crate::syntax::parsing::Element;


fn typeof_(expr: &Element) -> Element {
    Element::Variable {
        position: expr.get_pos().clone(),
        name: "".to_string(),
        parent: Box::new(Element::NullElement)
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
                                parent: content
                            }),
                            args: vec![*type_]
                        }),
                        variable,
                        position,
                        flags
                    };
                }
            }
        }
    }
    vec![]
}