use crate::objects::varstack::Varstack;
use crate::objects::element::Element;
use crate::objects::typeobj::TypeObj;

pub fn gen_instructions_from_program(mut input: Vec<Element>) -> Vec<Element> {
    let mut typelist = Varstack::<TypeObj>::default_type();
    for ele in input.iter_mut() {
        ele.get_type(&mut typelist);
    }
    input
}
pub fn gen_instructions_from_block(mut input: Vec<Element>, typelist: &mut Varstack<TypeObj>) -> Vec<Element> {
    for ele in input.iter_mut() {
        ele.get_type(typelist);
    }
    input
}