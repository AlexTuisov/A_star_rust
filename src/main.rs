#![allow(unused_variables, dead_code, unused_imports)]
mod search;
mod problem;
mod algorithms;

use crate::search::search_tree;
use crate::search::state::{Value, State};
use crate::problem::example_problem::SimpleProblem;
use crate::algorithms::bfs::BfsQueue;
use crate::algorithms::astar::AStarQueue;
use crate::algorithms::gbfs::GBFSQueue;
use crate::problem::problem::Problem;
use crate::search::search_tree::SearchTree;
use crate::search::search::generic_search;
use crate::problem::taxi_problem::taxi_problem::{load_state_from_json, TaxiProblem};
use std::time::Instant;



// fn solve_simple_problem(){
//     let mut initial_state = State::new();
//     initial_state.insert_field("health".to_string(), Value::Int(0));
//
//     let problem = SimpleProblem;
//     let mut tree = SearchTree::new(initial_state); // Remove the reference, tree should be mutable
//     let a_star_queue = AStarQueue::new();
//
//
//     match generic_search(
//         &mut tree, // Pass mutable reference to tree
//         |state| problem.get_possible_actions(state),
//         |state, action| problem.apply_action(state, action),
//         |state| problem.is_goal_state(state),
//         a_star_queue,
//         |state| problem.heuristic(state),
//     ) {
//         Ok(actions) => {
//             let total_cost: i32 = actions.iter().map(|action| action.cost).sum();
//             // println!("Solution found with actions: {:?}", actions);
//             println!("Total cost of actions: {}", total_cost);
//         }
//         Err(msg) => {
//             println!("Search failed: {}", msg);
//         }
//     }
// }

fn solve_taxi_problem(){
    let (state, config) = load_state_from_json("inputs/taxi_problem/generated_input.json").expect("Failed to load state from JSON");

    let problem = TaxiProblem {
        width: config.width,
        height: config.height,
        impassable_tiles: config.impassable_tiles,
        goals: config.goals,
    };

    let initial_node = problem.create_initial_node(state.clone());
    let actions = problem.get_possible_actions(&initial_node.state);
    let mut tree = SearchTree::new(state.clone());
    let a_star_queue = AStarQueue::new();
    let gbfs_queue = GBFSQueue::new();

    match generic_search(
        &mut tree, // Pass mutable reference to tree
        |state| problem.get_possible_actions(state),
        |state, action| problem.apply_action(state, action),
        |state| problem.is_goal_state(state),
        gbfs_queue,
        |state| problem.heuristic(state),
    ) {
        Ok(actions) => {
            let total_cost: i32 = actions.iter().map(|action| action.cost).sum();
            let action_names: Vec<_> = actions.iter().map(|action| &action.name).collect();
            println!("Solution found with actions: {:?}", action_names);
            println!("Total cost of actions: {}", total_cost);
        }
        Err(msg) => {
            println!("Search failed: {}", msg);
        }
    }

}


fn main() {
    let start_time = Instant::now();
    solve_taxi_problem();
    let elapsed_time = start_time.elapsed(); // Calculate elapsed time
    println!("Execution time: {:?}", elapsed_time);
}


