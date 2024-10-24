use std::collections::{HashSet, HashMap};
use std::collections::BTreeMap;


#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Position{
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Int(i32),
    Text(String),
    Bool(bool),
    IntArray(Vec<i32>),
    Positions(BTreeMap<String, Position>),
    MapToVecString(BTreeMap<String, Vec<String>>),
    MapToString(BTreeMap<String, String>),
    MapToInt(BTreeMap<String, i32>),
}



#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct State {
    fields: BTreeMap<String, Value>,
}

impl State {
    pub fn new() -> Self {
        State {
            fields: BTreeMap::new(),
        }
    }

    pub fn insert_field(&mut self, key: String, value: Value) {
        self.fields.insert(key, value);
    }

    pub fn get_field(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }
}