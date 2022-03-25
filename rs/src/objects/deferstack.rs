use crate::{Element, Stack, ZyxtError};
use crate::interpreter::interpret_block;
use crate::objects::variable::Variable;

pub struct DeferStack(Vec<Vec<Vec<Element>>>);

impl DeferStack {
    pub fn new() -> Self {
        DeferStack(vec![vec![]])
    }
    pub fn add_set(&mut self) {
        self.0.push(vec![]);
    }
    pub fn pop_set(&mut self) {
        self.0.pop();
    }
    pub fn add_defer(&mut self, content: Vec<Element>) {
        self.0.last_mut().unwrap().push(content);
    }
    pub fn execute_and_clear(&mut self, varlist: &mut Stack<Variable>) -> Result<Variable, ZyxtError>{
        for content in self.0.last().unwrap().clone() {
            if let Variable::Return(v) = interpret_block(content, varlist,
                                                         self, false, false)? {
                self.pop_set();
                return Ok(*v)
            }
        }
        self.pop_set();
        Ok(Variable::Null)
    }
}