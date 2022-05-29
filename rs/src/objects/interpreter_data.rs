use std::collections::HashMap;
use std::fmt::{Display};
use crate::{Element};
use crate::errors::ZyxtError;
use crate::interpreter::interpret_block;
use crate::objects::position::Position;
use crate::objects::typeobj::Type;
use crate::objects::variable::Variable;

const PRIM_NAMES: [&str; 21] = ["str",
    "i8", "i16", "i32", "i64", "i128", "isize", "ibig",
    "u8", "u16", "u32", "u64", "u128", "usize", "ubig",
    "f32", "f64", "bool",
    "#null", "#any", "type"];

pub struct FrameData<T: Clone> {
    pub position: Position,
    pub raw_call: String,
    pub args: HashMap<String, T>
}
pub struct InterpreterData<T: Clone> {
    pub heap: Vec<HashMap<String, T>>,
    pub defer: Vec<Vec<Vec<Element>>>,
    pub frame_data: Vec<Option<FrameData<T>>>
}

impl InterpreterData<Variable> {
    pub fn heap_to_string(&self) -> String {
        self.heap.iter()
            .map(|hmap| hmap.iter()
                .map(|(k, v)| format!("{}: {} = {}", k, v.get_type_obj(), v))
                .collect::<Vec<String>>()
                .join("\n"))
            .collect::<Vec<String>>()
            .join("\n-------\n")
    }
    pub fn pop_frame(&mut self) -> Result<Option<Variable>, ZyxtError> {
        self.heap.pop();
        self.frame_data.pop();

        for content in self.defer.last().unwrap().clone() {
            if let Variable::Return(v) = interpret_block(content, self, false, false)? {
                self.defer.pop();
                return Ok(Some(*v))
            }
        }
        self.defer.pop();
        Ok(None)
    }
}

impl InterpreterData<Type> {
    pub fn pop_frame(&mut self) {
        self.heap.pop();
        self.frame_data.pop();
        self.defer.pop();
    }
}

impl <T: Clone + Display> InterpreterData<T> {
    pub fn default_variable() -> InterpreterData<Variable> {
        let mut v = InterpreterData {
            heap: vec![HashMap::new()],
            defer: vec![vec![]],
            frame_data: vec![]
        };
        for t in PRIM_NAMES {
            v.heap[0].insert(t.to_string(), Variable::Type(Type::Instance {
                name: t.to_string(),
                type_args: vec![],
                inst_attrs: Default::default(),
                implementation: None
            }));
        }
        v.add_frame(None);
        v
    }
    pub fn default_type() -> InterpreterData<Type> {
        let mut v = InterpreterData {
            heap: vec![HashMap::new()],
            defer: vec![vec![]],
            frame_data: vec![]
        };
        for t in PRIM_NAMES {
            v.heap[0].insert(t.to_string(), Type::Instance {
                name: "type".to_string(),
                type_args: vec![],
                inst_attrs: Default::default(),
                implementation: None
            });
        }
        v.add_frame(None);
        v
    }
    pub fn add_frame(&mut self, frame_data: Option<FrameData<T>>) {
        self.heap.push(HashMap::new());
        self.defer.push(vec![]);
        self.frame_data.push(frame_data);
    }
    pub fn declare_val(&mut self, name: &str, value: &T) {
        self.heap.last_mut().unwrap().insert(name.to_string(), value.clone());
    }
    pub fn set_val(&mut self, name: &String, value: &T, position: &Position, raw: &String) -> Result<(), ZyxtError>{
        for set in self.heap.iter_mut().rev() {
            if set.contains_key(name) {set.insert(name.clone(), value.clone()); return Ok(())}
        }
        Err(ZyxtError::from_pos_and_raw(position, raw).error_3_0(name.clone()))
    }
    pub fn get_val(&mut self, name: &String, position: &Position, raw: &String) -> Result<T, ZyxtError> {
        for set in self.heap.iter().rev() {
            if set.contains_key(name) {return Ok(set.get(name).unwrap().clone())}
        }
        Err(ZyxtError::from_pos_and_raw(position, raw).error_3_0(name.clone()))
    }
    pub fn delete_val(&mut self, name: &String, position: &Position, raw: &String) -> Result<T, ZyxtError> {
        if let Some(v) = self.heap.last_mut().unwrap().remove(name) {Ok(v)}
        else {Err(ZyxtError::from_pos_and_raw(position, raw).error_3_0(name.clone()))}
    }
    pub fn add_defer(&mut self, content: Vec<Element>) {
        self.defer.last_mut().unwrap().push(content);
    }
}
