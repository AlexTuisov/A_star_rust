use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::search::{state::State, action::Action};


#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub state: State,        // The current state of the node
    pub parent: Option<usize>, // Index of the parent node
    pub children: Vec<usize>,  // Indexes of the child nodes
    pub action: Option<Action>, // The action that led to this node
    pub cost: i32,             // Cost to reach this node
}


// #[derive(Debug, Clone, PartialEq)]
// pub struct Node {
//     pub state: State,                 // The current state of the node
//     pub parent: Option<Rc<Node>>,     // Parent node for path reconstruction
//     pub action: Option<Action>,       // The action that led to this node (None for root)
//     pub cost: i32,                    // Cost to reach this node
// }
//
impl Node {
    pub fn new_empty(state: State) -> Self {
        Node {
            state,
            parent: None,
            children: Vec::new(),
            action: None,
            cost: 0,

        }
    }
}
//
//     // Create a new node from a parent and an action, applying external action logic
//     pub fn from_parent_and_action<F>(parent: Rc<&Node>, action: Action, apply_action: F) -> Self
//     where
//         F: Fn(&State, &Action) -> State,
//     {
//         // Use the external logic to create a new state
//         let new_state = apply_action(&parent.state, &action);
//
//         // Calculate the cost as the parent's cost + action's cost
//         let new_cost = parent.cost + action.cost;
//
//         Node {
//             state: new_state,
//             parent: Some(parent),
//             action: Some(action),
//             cost: new_cost,
//         }
//     }
//
//
//     // Trace the sequence of actions from the initial node to this node
//     pub fn trace_actions(&self) -> Vec<Action> {
//         let mut actions = Vec::new();
//         let mut current_node = Some(Rc::new(self.clone()));  // Start with the current node
//
//         while let Some(node) = current_node {
//             if let Some(action) = &node.action {
//                 actions.push(action.clone());  // Add the action to the sequence
//             }
//             current_node = node.parent.clone();  // Move to the parent node
//         }
//
//         actions.reverse();  // Reverse to get actions in the correct order from root to current node
//         actions
//     }
//
//     pub fn expand<F, G>(&self, get_possible_actions: F, apply_action: G) -> Vec<Node>
//     where
//         F: Fn(&State) -> Vec<Action>,
//         G: Fn(&State, &Action) -> State,
//     {
//         let actions = get_possible_actions(&self.state); // Get possible actions based on the current state
//         let mut successors = Vec::new();
//
//         let parent_rc = Rc::new(self);
//
//         for action in actions {
//             // Create a successor node using from_parent_and_action with apply_action passed in
//             let successor = Node::from_parent_and_action(parent_rc, action, &apply_action);
//             successors.push(successor);
//         }
//         successors
//     }
//
// }



// #[cfg(test)]
// mod tests {
//     use std::collections::HashMap;
//     use crate::search::value::Value;
//     use super::*;
//
//     #[test]
//     fn test_create_empty_node() {
//         let mut state = State::new();
//         state.insert_field("health".to_string(), Value::Int(100));
//
//         let node = Node::new_empty(state.clone());
//
//         // Ensure the node has no parent and no action
//         assert!(node.parent.is_none());
//         assert!(node.action.is_none());
//         assert_eq!(node.cost, 0);
//         assert_eq!(node.state.get_field("health"), Some(&Value::Int(100)));
//     }
//
//     #[test]
//     fn test_apply_action_externally() {
//         let mut root_state = State::new();
//         root_state.insert_field("health".to_string(), Value::Int(100));
//         let root_node = Node::new_empty(root_state.clone());
//
//         // External action application logic
//         let apply_action = |state: &State, action: &Action| {
//             let mut new_state = state.clone();
//             if action.name == "increase_health" {
//                 if let Some(Value::Int(health)) = new_state.get_field("health") {
//                     new_state.insert_field("health".to_string(), Value::Int(health + action.cost));
//                 }
//             }
//             new_state
//         };
//         let mut parameters = HashMap::new();
//         parameters.insert("speed".to_string(), Value::Int(10));
//         parameters.insert("direction".to_string(), Value::Text("north".to_string()));
//         let action = Action::new("increase_health".to_string(), 10, parameters);
//         let child_node = Node::from_parent_and_action(Rc::new(root_node), action, apply_action);
//
//         // Verify the state was modified correctly
//         assert_eq!(child_node.state.get_field("health"), Some(&Value::Int(110)));
//         assert_eq!(child_node.cost, 10);
//     }
// }

