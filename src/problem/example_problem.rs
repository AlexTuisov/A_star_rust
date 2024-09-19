use crate::search::{node::Node, state::State, action::Action, value::Value};
use crate::problem::problem::Problem;
use std::rc::Rc;

pub struct SimpleProblem;

impl Problem for SimpleProblem {
    fn create_initial_node(&self, initial_state: State) -> Node {
        Node::new_empty(initial_state)
    }

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        // Example: Define actions based on the state
        vec![
            Action::new("increase_health".to_string(), 10),
            Action::new("decrease_health".to_string(), -10),
        ]
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        if let Some(Value::Int(health)) = new_state.get_field("health") {
            match action.name.as_str() {
                "increase_health" => new_state.insert_field("health".to_string(), Value::Int(health + action.cost)),
                "decrease_health" => new_state.insert_field("health".to_string(), Value::Int(health + action.cost)),
                _ => (),
            }
        }
        new_state
    }

    fn is_goal_state(&self, state: &State) -> bool {
        // Example goal check: health reaching 200
        if let Some(Value::Int(health)) = state.get_field("health") {
            *health == 200
        } else {
            false
        }
    }
}
