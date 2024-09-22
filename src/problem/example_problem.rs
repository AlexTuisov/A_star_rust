use std::collections::HashMap;
use crate::search::{node::Node, state::State, action::Action, value::Value};
use crate::problem::problem::Problem;
use std::rc::Rc;

pub struct SimpleProblem;

impl Problem for SimpleProblem {
    fn create_initial_node(&self, initial_state: State) -> Node {
        Node::new_empty(initial_state)
    }

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();

        // Generate actions for increasing health by any integer value from 1 to 10
        for amount in 1..=10 {
            let mut params = HashMap::new();
            params.insert("amount".to_string(), Value::Int(amount));
            actions.push(Action::new("increase_health".to_string(), 1, params));
        }
        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        if let Some(Value::Int(amount)) = action.parameters.get("amount") {
            if let Some(Value::Int(health)) = new_state.get_field("health") {
                new_state.insert_field("health".to_string(), Value::Int(health + amount));
            }
        }
        new_state
    }


    fn is_goal_state(&self, state: &State) -> bool {
        // Example goal check: health reaching 200
        if let Some(Value::Int(health)) = state.get_field("health") {
            *health >= 100000
        } else {
            false
        }
    }

    fn heuristic(&self, state: &State) -> f64 {
        if let Some(Value::Int(health)) = state.get_field("health") {
            (100000.0 - *health as f64) / 5.0
        } else {
            0.0
        }
    }

}
