use crate::search::{node::Node, state::State, action::Action};
use std::rc::Rc;

pub trait Problem {
    // 1. Given an initial state, create an initial Node
    fn create_initial_node(&self, initial_state: State) -> Node;

    // 2. Defining the transition dynamics - given a state, output a vector of all possible actions from the state
    fn get_possible_actions(&self, state: &State) -> Vec<Action>;

    // 3. Apply action function - given a state and action, return a new state
    fn apply_action(&self, state: &State, action: &Action) -> State;

    // 4. Goal detection - given a state, return a binary indicating whether it is a goal state or not
    fn is_goal_state(&self, state: &State) -> bool;
}
