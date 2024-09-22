use crate::algorithms::priority_queue::PriorityQueue;
use crate::search::search_tree::SearchTree;
use crate::search::action::Action;
use crate::search::state::State;

// Generic search function that operates on a SearchTree and uses a priority queue for the search strategy
pub fn generic_search<F, G, H, Q>(
    tree: &mut SearchTree,
    get_possible_actions: F,
    apply_action: G,
    is_goal: H,
    mut queue: Q,
) -> Result<Vec<Action>, &'static str>
where
    F: Fn(&State) -> Vec<Action>,
    G: Fn(&State, &Action) -> State,
    H: Fn(&State) -> bool,
    Q: PriorityQueue,
{
    queue.insert(0, 0);
    while let Some(current_index) = queue.pop() {
        let successors = tree.expand_node(current_index, &get_possible_actions, &apply_action);
        for &successor_index in &successors {
            let successor_node = tree.get_node(successor_index).unwrap();
            if is_goal(&successor_node.state) {
                return Ok(tree.trace_actions(successor_index));
            }
            queue.insert(successor_index, successor_node.cost);
        }
    }
    Err("No solution found")
}




#[cfg(test)]
mod tests {
    use super::*; // Import the function and necessary types from the current module
    use crate::search::search_tree::SearchTree;
    use crate::search::state::State;
    use crate::search::action::Action;
    use crate::search::value::Value;
    use crate::algorithms::bfs::BfsQueue; // Assuming BfsQueue is implemented under priority_queue/bfs.rs
    use std::collections::HashMap;

    // Helper function to create an action with specified parameters
    fn create_action(name: &str, cost: i32, amount: i32) -> Action {
        let mut parameters = HashMap::new();
        parameters.insert("amount".to_string(), Value::Int(amount));
        Action::new(name.to_string(), cost, parameters)
    }

    #[test]
    fn test_generic_search_with_bfs() {
        // Initialize the initial state
        let mut initial_state = State::new();
        initial_state.insert_field("health".to_string(), Value::Int(50));

        // Create a search tree
        let mut tree = SearchTree::new(initial_state.clone());

        // Define the get_possible_actions function
        let get_possible_actions = |state: &State| {
            let mut actions = Vec::new();
            for amount in 1..=3 {
                actions.push(create_action("increase_health", amount, amount));
            }
            actions
        };

        // Define the apply_action function
        let apply_action = |state: &State, action: &Action| {
            let mut new_state = state.clone();
            if let Some(Value::Int(amount)) = action.parameters.get("amount") {
                if let Some(Value::Int(health)) = new_state.get_field("health") {
                    new_state.insert_field("health".to_string(), Value::Int(health + amount));
                }
            }
            new_state
        };

        // Define the is_goal function
        let is_goal = |state: &State| {
            if let Some(Value::Int(health)) = state.get_field("health") {
                *health >= 60 // Example goal condition: health should reach or exceed 60
            } else {
                false
            }
        };

        // Initialize a BFS queue
        let bfs_queue = BfsQueue::new();

        // Run the generic search with BFS
        let result = generic_search(&mut tree, get_possible_actions, apply_action, is_goal, bfs_queue);

        // Check if the search found a solution
        assert!(result.is_ok(), "Expected to find a solution");

        // Verify the actions returned lead to the goal
        let actions = result.unwrap();
        let mut health = 50;
        for action in &actions {
            if let Some(Value::Int(amount)) = action.parameters.get("amount") {
                health += amount;
            }
        }
        assert!(health >= 60, "Expected health to reach or exceed 60, got {}", health);
    }
}

