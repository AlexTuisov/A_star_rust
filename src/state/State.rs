use std::collections::HashMap;
use std::collections::BTreeMap;
use super::value::Value;

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
}
