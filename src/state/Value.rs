#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Value {
    Int(i32),
    Text(String),
    Bool(bool),
    IntArray(Vec<i32>),
}
