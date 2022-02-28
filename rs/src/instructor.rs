use std::collections::HashMap;
use crate::syntax::element::Element;

pub fn gen_instructions(mut input: Vec<Element>) -> Vec<Element> {
    let mut typelist: HashMap<String, Element> = HashMap::new();
    for t in ["str", "i32", "f64", "#null", "type"] {
        typelist.insert(t.to_string(), Element::Variable {
            position: Default::default(),
            name: "type".to_string(),
            parent: Box::new(Element::NullElement)
        });
    }
    for ele in input.iter_mut() {
        ele.get_type(&mut typelist);
    }
    input
}