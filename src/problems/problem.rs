use crate::search::{node::Node, state::State, action::Action};
use std::rc::Rc;

pub trait Problem {
    fn create_initial_node(&self, initial_state: State) -> Node;
    fn get_possible_actions(&self, state: &State) -> Vec<Action>;
    fn apply_action(&self, state: &State, action: &Action) -> State;
    fn is_goal_state(&self, state: &State) -> bool;
    fn heuristic(&self, state: &State) -> f64;
}
