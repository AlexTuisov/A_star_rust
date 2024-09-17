use std::rc::Rc;
use std::hash::{Hash, Hasher};
use crate::search::state::State;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub state: State,                 // The current state of the node
    pub parent: Option<Rc<Node>>,     // Parent node for path reconstruction
    pub cost: i32,                    // Cost to reach this node
}

impl Node {
    pub fn new(state: State, parent: Option<Rc<Node>>, cost: i32) -> Self {
        Node { state, parent, cost }
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.state.hash(state);       // Hash the state
        self.cost.hash(state);        // Include the cost in the hash
    }
}

// Unit tests for Node
#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::value::Value;  // Assuming `Value` is defined in `value.rs`

    #[test]
    fn test_node_creation() {
        let mut state = State::new();
        state.insert_field("health".to_string(), Value::Int(100));

        let node = Node::new(state.clone(), None, 0);
        assert_eq!(node.cost, 0);
        assert_eq!(node.state.get_field("health"), Some(&Value::Int(100)));
        assert!(node.parent.is_none());
    }

    #[test]
    fn test_node_with_parent() {
        let root_state = State::new();
        let root_node = Node::new(root_state.clone(), None, 0);

        let mut child_state = State::new();
        child_state.insert_field("energy".to_string(), Value::Int(50));
        let child_node = Node::new(child_state.clone(), Some(Rc::new(root_node.clone())), 10);

        assert_eq!(child_node.cost, 10);
        assert_eq!(child_node.state.get_field("energy"), Some(&Value::Int(50)));
        assert!(child_node.parent.is_some());
    }

    #[test]
    fn test_node_hashing() {
        let mut state1 = State::new();
        state1.insert_field("health".to_string(), Value::Int(100));

        let node1 = Node::new(state1.clone(), None, 0);

        let mut state2 = State::new();
        state2.insert_field("health".to_string(), Value::Int(100));

        let node2 = Node::new(state2.clone(), None, 0);

        // Ensure that two nodes with the same state and cost have the same hash
        use std::collections::hash_map::DefaultHasher;
        let mut hasher1 = DefaultHasher::new();
        let mut hasher2 = DefaultHasher::new();
        node1.hash(&mut hasher1);
        node2.hash(&mut hasher2);

        assert_eq!(hasher1.finish(), hasher2.finish());
    }
}
