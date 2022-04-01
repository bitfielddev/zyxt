use std::collections::HashMap;
use crate::errors::ZyxtError;
use crate::objects::position::Position;
use crate::objects::typeobj::TypeObj;
use crate::objects::variable::Variable;

const PRIM_NAMES: [&str; 18] = ["str",
    "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64",
    "#null", "#any", "type"];

pub struct Stack<T: Clone>(pub Vec<HashMap<String, T>>);

impl <T: Clone> Stack<T> {
    pub fn default_variable() -> Stack<Variable> {
        let mut v = Stack(vec![HashMap::new()]);
        for t in PRIM_NAMES {
            v.0[0].insert(t.to_string(), Variable::Type(TypeObj::Type {
                name: t.to_string(), type_args: vec![]
            }));
        }
        v
    }
    pub fn default_type() -> Stack<TypeObj> {
        let mut v = Stack(vec![HashMap::new()]);
        for t in PRIM_NAMES {
            v.0[0].insert(t.to_string(), TypeObj::Type {
                name: "type".to_string(), type_args: vec![]
            });
        }
        v
    }
    pub fn add_set(&mut self) {
        self.0.push(HashMap::new());
    }
    pub fn pop_set(&mut self) {
        self.0.pop();
    }
    pub fn declare_val(&mut self, name: &String, value: &T) {
        self.0.last_mut().unwrap().insert(name.clone(), value.clone());
    }
    pub fn set_val(&mut self, name: &String, value: &T, position: &Position) -> Result<(), ZyxtError>{
        for set in self.0.iter_mut().rev() {
            if set.contains_key(name) {set.insert(name.clone(), value.clone()); return Ok(())}
        }
        Err(ZyxtError::from_pos(position).error_3_0(name.clone()))
    }
    pub fn get_val(&mut self, name: &String, position: &Position) -> Result<T, ZyxtError> {
        for set in self.0.iter().rev() {
            if set.contains_key(name) {return Ok(set.get(name).unwrap().clone())}
        }
        Err(ZyxtError::from_pos(position).error_3_0(name.clone()))
    }
    pub fn delete_val(&mut self, name: &String, position: &Position) -> Result<T, ZyxtError> {
        if let Some(v) = self.0.last_mut().unwrap().remove(name) {Ok(v)}
        else {Err(ZyxtError::from_pos(position).error_3_0(name.clone()))}
    }
}
