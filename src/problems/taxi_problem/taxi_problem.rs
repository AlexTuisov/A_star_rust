use std::collections::BTreeMap;
use std::collections::HashMap;
use crate::search::{node::Node, state::State, action::Action, state::Value, state::Position};
use crate::problems::problem::Problem;
use std::rc::Rc;
// use std::collections::HashSet;
use std::cmp::Ordering;
use std::collections::{HashSet, BinaryHeap};



use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct ProblemConfig {
    pub width: i32,
    pub height: i32,
    pub impassable_tiles: HashSet<Position>,
    pub goals: BTreeMap<String, Position>,
}


pub fn load_state_from_json(file_path: &str) -> Result<(State, ProblemConfig), Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let raw_state: serde_json::Value = from_reader(reader)?;

    let bounds = raw_state["bounds"].as_array().unwrap();
    let width = bounds[0].as_i64().unwrap() as i32;
    let height = bounds[1].as_i64().unwrap() as i32;

    let entities = raw_state["entities"].as_object().unwrap();
    let mut positions_map = BTreeMap::new();
    let mut goals_map = BTreeMap::new();

    if let Some(taxi) = entities.get("taxi") {
        let taxi_coords = taxi.as_array().unwrap();
        positions_map.insert(
            "taxi".to_string(),
            Position::new(
                taxi_coords[0].as_i64().unwrap() as i32,
                taxi_coords[1].as_i64().unwrap() as i32,
            ),
        );
    }

    let passengers = entities.get("passengers").unwrap().as_object().unwrap();
    for (key, value) in passengers {
        let coords = value.as_array().unwrap();
        let position = Position::new(coords[0].as_i64().unwrap() as i32, coords[1].as_i64().unwrap() as i32);

        if key.starts_with("goal") {
            goals_map.insert(key.clone(), position);
        } else {
            positions_map.insert(key.clone(), position);
        }
    }

    let impassable_tiles = entities.get("impassable_tiles").unwrap().as_array().unwrap();
    let impassable_positions: HashSet<Position> = impassable_tiles
        .iter()
        .map(|tile| {
            let coords = tile.as_array().unwrap();
            Position::new(coords[0].as_i64().unwrap() as i32, coords[1].as_i64().unwrap() as i32)
        })
        .collect();

    let mut state = State::new();
    state.insert_field("positions".to_string(), Value::Positions(positions_map));

    let config = ProblemConfig {
        width,
        height,
        impassable_tiles: impassable_positions,
        goals: goals_map,
    };

    Ok((state, config))
}


pub struct TaxiProblem {
    pub width: i32,
    pub height: i32,
    pub impassable_tiles: HashSet<Position>,
    pub goals: BTreeMap<String, Position>,
}

impl TaxiProblem{

    fn is_position_valid(&self, pos: &Position) -> bool {
        let within_bounds = pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height;
        if !within_bounds {
            return false;
        }
        !self.impassable_tiles.contains(pos)
    }

    fn manhattan_distance(pos1: &Position, pos2: &Position) -> f64 {
        (pos1.x - pos2.x).abs() as f64 + (pos1.y - pos2.y).abs() as f64
    }
}

impl Problem for TaxiProblem {
    fn create_initial_node(&self, initial_state: State) -> Node {
        Node {
            state: initial_state,
            parent: None,
            children: Vec::new(),
            action: None,
            cost: 0,
        }
    }

    fn get_possible_actions(&self, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        let positions = match state.get_field("positions") {
            Some(Value::Positions(positions)) => positions,
            _ => return actions,
        };

        let taxi_pos = match positions.get("taxi") {
            Some(pos) => pos,
            None => return actions,
        };

        let moves = vec![
            ("move_up", Position::new(taxi_pos.x, taxi_pos.y - 1)),
            ("move_down", Position::new(taxi_pos.x, taxi_pos.y + 1)),
            ("move_left", Position::new(taxi_pos.x - 1, taxi_pos.y)),
            ("move_right", Position::new(taxi_pos.x + 1, taxi_pos.y)),
            ("stay", Position::new(taxi_pos.x, taxi_pos.y)),
        ];

        for (action_name, new_pos) in moves {
            if self.is_position_valid(&new_pos) {
                actions.push(Action::new(action_name.to_string(), 1, HashMap::new()));
            }
        }

        for (key, pos) in positions.iter() {
            if key.starts_with("passenger") && pos == taxi_pos {
                actions.push(Action::new(
                    format!("pick_up_{}", key),
                    1,
                    HashMap::new(),
                ));
            }
        }

        for (goal_key, goal_pos) in self.goals.iter() {
            let passenger_key = format!("in_taxi_{}", goal_key.replace("goal", "passenger"));
            if goal_pos == taxi_pos && positions.contains_key(&passenger_key) {
                actions.push(Action::new(
                    format!("disembark_{}", goal_key),
                    1,
                    HashMap::new(),
                ));
            }
        }

        actions
    }

    fn apply_action(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();
        let positions = match new_state.get_field("positions") {
            Some(Value::Positions(positions)) => positions.clone(),
            _ => return new_state,
        };

        let mut updated_positions = positions.clone();

        if action.name.starts_with("move") {
            let taxi_pos = positions.get("taxi").unwrap();
            let new_taxi_pos = match action.name.as_str() {
                "move_up" => Position::new(taxi_pos.x, taxi_pos.y - 1),
                "move_down" => Position::new(taxi_pos.x, taxi_pos.y + 1),
                "move_left" => Position::new(taxi_pos.x - 1, taxi_pos.y),
                "move_right" => Position::new(taxi_pos.x + 1, taxi_pos.y),
                "stay" => taxi_pos.clone(),
                _ => taxi_pos.clone(),
            };
            updated_positions.insert("taxi".to_string(), new_taxi_pos);
        }

        if action.name.starts_with("pick_up") {
            let passenger_key = action.name.replace("pick_up_", "");
            if let Some(passenger_pos) = positions.get(&passenger_key) {
                if let Some(taxi_pos) = positions.get("taxi") {
                    if passenger_pos == taxi_pos {
                        updated_positions.remove(&passenger_key);
                        updated_positions.insert(format!("in_taxi_{}", passenger_key), taxi_pos.clone());
                    }
                }
            }
        }

        if action.name.starts_with("disembark") {
            let goal_key = action.name.replace("disembark_", "");
            let passenger_key = format!("in_taxi_{}", goal_key.replace("goal", "passenger"));

            // Check if the passenger is in the taxi and the taxi is at the correct goal position
            if let Some(goal_pos) = self.goals.get(&goal_key) {
                if let Some(taxi_pos) = positions.get("taxi") {
                    if taxi_pos == goal_pos && positions.contains_key(&passenger_key) {
                        updated_positions.remove(&passenger_key);
                        updated_positions.insert(
                            passenger_key.replace("in_taxi_", ""),
                            goal_pos.clone(),
                        );
                    }
                }
            }
        }

        new_state.insert_field("positions".to_string(), Value::Positions(updated_positions));
        new_state
    }

    fn is_goal_state(&self, state: &State) -> bool {
        let positions = match state.get_field("positions") {
            Some(Value::Positions(positions)) => positions,
            _ => {
                println!("Failed to retrieve positions from state.");
                return false;
            }
        };

        // Check if all goals have their corresponding passengers disembarked
        for (goal_key, goal_pos) in &self.goals {
            let passenger_key = goal_key.replace("goal", "passenger");

            // Check if the passenger is still in the taxi
            if positions.contains_key(&format!("in_taxi_{}", passenger_key)) {
                return false; // Passenger is still in the taxi
            }

            // Ensure the passenger is at the correct goal position
            if let Some(pos) = positions.get(&passenger_key) {
                if pos != goal_pos {
                    return false; // Passenger is not at their goal position
                }
            } else {
                return false; // Passenger is missing from positions
            }
        }

        println!("Goal state reached successfully.");
        true // All passengers have been disembarked at their respective goals
    }

    // fn heuristic(&self, state: &State) -> f64 {
    //     let positions = match state.get_field("positions") {
    //         Some(Value::Positions(positions)) => positions,
    //         _ => return f64::MAX,
    //     };
    //
    //     let mut total_distance = 0.0;
    //     let mut min_taxi_to_passenger_distance = f64::MAX;
    //
    //     for (goal_key, goal_pos) in &self.goals {
    //         let passenger_key = goal_key.replace("goal", "passenger");
    //
    //         if positions.contains_key(&format!("in_taxi_{}", passenger_key)) {
    //             // Passenger is in the taxi, add the distance to the goal + 1 for disembark
    //             total_distance += Self::manhattan_distance(positions.get("taxi").unwrap(), goal_pos) + 1.0;
    //         } else if let Some(passenger_pos) = positions.get(&passenger_key) {
    //             // Passenger is not in the taxi and not at the goal, add the distance + 2 (for embark and disembark)
    //             total_distance += Self::manhattan_distance(passenger_pos, goal_pos) + 2.0;
    //
    //             // Calculate the distance from the taxi to this passenger
    //             if let Some(taxi_pos) = positions.get("taxi") {
    //                 let taxi_to_passenger_distance = Self::manhattan_distance(taxi_pos, passenger_pos);
    //                 // Track the minimum distance between the taxi and any passenger
    //                 if taxi_to_passenger_distance < min_taxi_to_passenger_distance {
    //                     min_taxi_to_passenger_distance = taxi_to_passenger_distance;
    //                 }
    //             }
    //         }
    //     }
    //
    //     // Add the minimal taxi-to-passenger distance to the total heuristic
    //     total_distance += min_taxi_to_passenger_distance;
    //
    //     total_distance
    // }



    fn heuristic(&self, state: &State) -> f64 {
        // Helper function to compute Manhattan distance between two positions
        fn manhattan_distance(pos1: &Position, pos2: &Position) -> f64 {
            ((pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs()) as f64
        }

        // Extract positions from the state
        let positions = match state.get_field("positions") {
            Some(Value::Positions(positions)) => positions,
            _ => return 0.0,
        };

        let taxi_pos = match positions.get("taxi") {
            Some(pos) => pos,
            None => return 0.0,
        };

        let mut total_cost = 0.0;

        // Check if the taxi is carrying a passenger
        let mut in_taxi_passenger: Option<String> = None;
        for key in positions.keys() {
            if key.starts_with("in_taxi_passenger") {
                in_taxi_passenger = Some(key.clone());
                break;
            }
        }

        if let Some(passenger_in_taxi_key) = in_taxi_passenger {
            // Taxi is carrying a passenger
            let passenger_key = passenger_in_taxi_key.replace("in_taxi_", "");
            let goal_key = passenger_key.replace("passenger", "goal");
            let goal_pos = match self.goals.get(&goal_key) {
                Some(pos) => pos,
                None => return 0.0,
            };
            // Distance from taxi's position to passenger's goal
            total_cost += manhattan_distance(taxi_pos, goal_pos);
        } else {
            // Taxi is not carrying a passenger
            // Find the minimal distance to any passenger
            let mut min_pickup_cost = f64::INFINITY;
            for (key, pos) in positions {
                if key.starts_with("passenger") {
                    let pickup_cost = manhattan_distance(taxi_pos, pos);
                    if pickup_cost < min_pickup_cost {
                        min_pickup_cost = pickup_cost;
                    }
                }
            }
            total_cost += min_pickup_cost;
        }

        // Sum distances from undelivered passengers to their goals
        for (key, pos) in positions {
            if key.starts_with("passenger") {
                let goal_key = key.replace("passenger", "goal");
                let goal_pos = match self.goals.get(&goal_key) {
                    Some(pos) => pos,
                    None => continue,
                };
                total_cost += manhattan_distance(pos, goal_pos);
            }
        }

        total_cost
    }

}