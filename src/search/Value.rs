use std::collections::HashSet;
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
}
