use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::errors::ZyxtError;
use crate::objects::position::Position;
use crate::objects::typeobj::Type;
use crate::objects::variable::Variable;

const PRIM_NAMES: [&str; 19] = ["str",
    "i8", "i16", "i32", "i64", "i128", "isize",
    "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64", "bool",
    "#null", "#any", "type"];

pub struct Heap<T: Clone>(pub Vec<HashMap<String, T>>);

impl Display for Heap<Variable> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter()
            .map(|hmap| hmap.iter()
                .map(|(k, v)| format!("{}: {} = {}", k, v.get_type_obj(), v))
                .collect::<Vec<String>>()
                .join("\n"))
            .collect::<Vec<String>>()
            .join("\n-------\n")
        )
    }
}

impl <T: Clone> Heap<T> {
    pub fn default_variable() -> Heap<Variable> {
        let mut v = Heap(vec![HashMap::new()]);
        for t in PRIM_NAMES {
            v.0[0].insert(t.to_string(), Variable::Type(Type::Instance {
                name: t.to_string(),
                type_args: vec![],
                inst_attrs: Default::default(),
                implementation: None
            }));
        }
        v.add_set();
        v
    }
    pub fn default_type() -> Heap<Type> {
        let mut v = Heap(vec![HashMap::new()]);
        for t in PRIM_NAMES {
            v.0[0].insert(t.to_string(), Type::Instance {
                name: "type".to_string(),
                type_args: vec![],
                inst_attrs: Default::default(),
                implementation: None
            });
        }
        v.add_set();
        v
    }
    pub fn add_set(&mut self) {
        self.0.push(HashMap::new());
    }
    pub fn pop_set(&mut self) {
        self.0.pop();
    }
    pub fn declare_val(&mut self, name: &str, value: &T) {
        self.0.last_mut().unwrap().insert(name.to_string(), value.clone());
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
