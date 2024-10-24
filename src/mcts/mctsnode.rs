use crate::search::{state::{State, Value}, action::Action};

pub struct MCTSNode {
    pub state: State,                 // Current state of the node (from Node)
    pub parent: Option<usize>,        // Parent node index (reused from Node)
    pub children: Vec<usize>,         // Children node indices (reused from Node)
    pub action: Option<Action>,       // Action leading to this node (reused from Node)
    pub visits: u32,                  // Visit count
    pub value: f64,                   // Accumulated value (reward)
}

impl MCTSNode {
    // Create a new MCTSNode, reusing existing Node components
    pub fn new(state: State, parent: Option<usize>, action: Option<Action>) -> Self {
        MCTSNode {
            state,
            parent,
            children: Vec::new(),
            action,
            visits: 0,
            value: 0.0,
        }
    }

    // Function to update the node's statistics after a simulation
    pub fn update(&mut self, reward: f64) {
        self.visits += 1;
        self.value += reward;  // Update the total accumulated value
    }

    // Function to calculate the UCT (Upper Confidence Bound for Trees)
    pub fn uct_value(&self, exploration_weight: f64, total_visits: u32) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY;  // Encourage exploration of unvisited nodes
        }
        (self.value / self.visits as f64)
            + exploration_weight * ((total_visits as f64).ln() / self.visits as f64).sqrt()
    }
}
