use crate::syntax::parsing::Element;

fn typeof_(expr: Element) -> Element {
    Element::Variable {
        position: expr.get_pos().clone(),
        name: "".to_string(),
        parent: Box::new(Element::NullElement)
    }
}

fn typecheck(input: Vec<Element>) -> Vec<Element> {
    let mut new_eles = vec![];
    for ele in input.iter() {
        if let &Element::DeclarationStmt {position, variable, content, flags, type_} = ele {
            new_eles.push(Element::DeclarationStmt{
                type_: Box::new(typeof_(*content.clone())),
                position, variable, content, flags
            });
        }
    }
    vec![]
}