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

    pub fn get_field(&self, key: &str) -> Option<&Value> {
        self.fields.get(key)
    }
}


#[test]
fn test_insert_field() {
    let mut state = State::new();
    state.insert_field("health".to_string(), Value::Int(100));
    state.insert_field("location".to_string(), Value::Text("right corner".to_string()));
    assert_eq!(state.get_field("health").and_then(|v| if let Value::Int(i) = v { Some(i) } else { None }), Some(&100));
}