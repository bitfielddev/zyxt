use crate::interpreter::Varstack;
use crate::objects::element::Element;

pub fn gen_instructions(mut input: Vec<Element>) -> Vec<Element> {
    let mut typelist = Varstack::<Element>::default_type();
    for ele in input.iter_mut() {
        ele.get_type(&mut typelist);
    }
    input
}