use crate::search::{node::Node, state::State, action::Action, state::Value};
use crate::algorithms::priority_queue::PriorityQueue;
pub struct SearchTree {
    pub nodes: Vec<Node>,    // A vector to store all nodes
}

impl SearchTree {
    // Create a new empty tree with an initial node
    pub fn new(initial_state: State) -> Self {
        let root = Node {
            state: initial_state,
            parent: None,
            children: Vec::new(),
            action: None,
            cost: 0,
        };

        SearchTree {
            nodes: vec![root], // Add the root node to the nodes vector
        }
    }

    // Add a new node to the tree given a parent index and an action
    pub fn add_node<F>(&mut self, parent_index: usize, action: Action, apply_action: F) -> usize
    where
        F: Fn(&State, &Action) -> State,
    {
        let parent_node = &self.nodes[parent_index];
        let new_state = apply_action(&parent_node.state, &action);
        let new_cost = parent_node.cost + action.cost;

        let new_node = Node {
            state: new_state,
            parent: Some(parent_index),
            children: Vec::new(),
            action: Some(action),
            cost: new_cost,
        };

        let new_node_index = self.nodes.len();
        self.nodes.push(new_node);

        // Update the parent's children list
        self.nodes[parent_index].children.push(new_node_index);

        new_node_index
    }

    // Get the node by its index
    pub fn get_node(&self, index: usize) -> Option<&Node> {
        self.nodes.get(index)
    }


    pub fn trace_actions(&self, node_index: usize) -> Vec<Action> {
        let mut actions = Vec::new();
        let mut current_index = Some(node_index);
        while let Some(index) = current_index {
            if let Some(node) = self.get_node(index) {
                if let Some(action) = &node.action {
                    actions.push(action.clone());
                }
                current_index = node.parent;
            } else {
                break;
            }
        }
        actions.reverse();
        actions
    }


    pub fn expand_node<F, G>(&mut self, node_index: usize, get_possible_actions: F, apply_action: G) -> Vec<usize>
    where
        F: Fn(&State) -> Vec<Action>,
        G: Fn(&State, &Action) -> State,
    {
        let mut successors = Vec::new();
        if let Some(node) = self.get_node(node_index) {
            let actions = get_possible_actions(&node.state);
            for action in actions {
                let new_node_index = self.add_node(node_index, action, &apply_action);
                successors.push(new_node_index);
            }
        }
        successors
    }

    pub fn print_tree(&self, node_index: usize, indent: usize) {
        if let Some(node) = self.get_node(node_index) {
            // Print the current node details with indentation to show hierarchy
            println!(
                "{:indent$}Node Index: {}, Cost: {}, Action: {:?}, State: {:?}",
                "",
                node_index,
                node.cost,
                node.action.as_ref().map(|a| &a.name),
                node.state,
                indent = indent
            );

            // Recursively print all children of the current node
            for &child_index in &node.children {
                self.print_tree(child_index, indent + 4); // Increase indent for child nodes
            }
        }
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // Helper function to create an action with specified parameters
    fn create_action(name: &str, cost: i32, amount: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("amount".to_string(), Value::Int(amount));
        Action::new(name.to_string(), cost, parameters)
    }

    // Test for tracing actions from the root to a specific node
    #[test]
    fn test_trace_actions() {
        let initial_state = State::new();
        let mut tree = SearchTree::new(initial_state);

        // Define apply_action logic
        let apply_action = |state: &State, action: &Action| {
            let mut new_state = state.clone();
            if let Some(Value::Int(amount)) = action.parameters.get("amount") {
                if let Some(Value::Int(health)) = new_state.get_field("health") {
                    new_state.insert_field("health".to_string(), Value::Int(health + amount));
                } else {
                    new_state.insert_field("health".to_string(), Value::Int(*amount));
                }
            }
            new_state
        };

        // Add nodes to the tree
        let action1 = create_action("increase_health", 5, 10);
        let first_node_index = tree.add_node(0, action1.clone(), &apply_action);

        let action2 = create_action("increase_health", 3, 5);
        let second_node_index = tree.add_node(first_node_index, action2.clone(), &apply_action);

        // Trace actions back from the second node to the root
        let traced_actions = tree.trace_actions(second_node_index);
        assert_eq!(traced_actions, vec![action1, action2]);
    }

    // Test for expanding a node
    #[test]
    fn test_expand_node() {
        let mut initial_state = State::new();
        initial_state.insert_field("health".to_string(), Value::Int(50));
        let mut tree = SearchTree::new(initial_state);

        // Define get_possible_actions logic
        let get_possible_actions = |state: &State| {
            let mut actions = Vec::new();
            for amount in 1..=3 {
                let mut parameters = HashMap::new();
                parameters.insert("amount".to_string(), Value::Int(amount));
                actions.push(Action::new("increase_health".to_string(), amount, parameters));
            }
            actions
        };

        // Define apply_action logic
        let apply_action = |state: &State, action: &Action| {
            let mut new_state = state.clone();
            if let Some(Value::Int(amount)) = action.parameters.get("amount") {
                if let Some(Value::Int(health)) = new_state.get_field("health") {
                    new_state.insert_field("health".to_string(), Value::Int(health + amount));
                }
            }
            new_state
        };

        // Expand the root node
        let successors = tree.expand_node(0, get_possible_actions, apply_action);

        // Check that the correct number of successors were created
        assert_eq!(successors.len(), 3);

        // Check that the successors have the expected health values
        assert_eq!(tree.get_node(successors[0]).unwrap().state.get_field("health"), Some(&Value::Int(51)));
        assert_eq!(tree.get_node(successors[1]).unwrap().state.get_field("health"), Some(&Value::Int(52)));
        assert_eq!(tree.get_node(successors[2]).unwrap().state.get_field("health"), Some(&Value::Int(53)));
        tree.print_tree(0, 2);
    }
}
