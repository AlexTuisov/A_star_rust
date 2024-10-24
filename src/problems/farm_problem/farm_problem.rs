use crate::search::{node::Node, state::State, action::Action, state::Value, state::Position};
use std::collections::{BTreeMap, HashMap};
use crate::problems::problem::Problem;
use serde_json::{to_string, Value as JsonValue};
use std::collections::{HashMap as StdHashMap};
use crate::problems::taxi_problem::taxi_problem::TaxiProblem;


pub struct WeightedSumGoal {
    pub weights: BTreeMap<String, f64>,
    pub threshold: f64,
}

pub struct FarmProblem {
    pub farms: Vec<String>,               // List of farms
    pub x_values: BTreeMap<String, i32>,  // X values for each farm
    pub adjacencies: BTreeMap<String, Vec<String>>,  // Adjacencies between farms
    pub cost: i32,                        // Initial cost
    pub goal_thresholds: BTreeMap<String, i32>,      // Goal thresholds for each farm
    pub weighted_sum_goal: WeightedSumGoal,          // Weighted sum goal
}
impl FarmProblem {
    // Constructor that takes a path to a JSON file and loads the data
    pub fn new_from_json(path: &str) -> Self {
        // Load the data from the JSON file
        let file = std::fs::File::open(path).expect("File not found");
        let reader = std::io::BufReader::new(file);
        let json: serde_json::Value = serde_json::from_reader(reader).expect("Error reading JSON");

        // Parse farms
        let farms = json["farms"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| f.as_str().unwrap().to_string())
            .collect();

        // Parse x_values
        let x_values = json["initial_state"]["x_values"].as_object().unwrap();
        let mut parsed_x_values = BTreeMap::new();
        for (farm, x_val) in x_values {
            parsed_x_values.insert(farm.clone(), x_val.as_i64().unwrap() as i32);
        }

        // Parse adjacencies
        let adjacencies = json["initial_state"]["adjacencies"].as_object().unwrap();
        let mut parsed_adjacencies = BTreeMap::new();
        for (farm, neighbors) in adjacencies {
            let neighbors_vec = neighbors
                .as_array()
                .unwrap()
                .iter()
                .map(|n| n.as_str().unwrap().to_string())
                .collect();
            parsed_adjacencies.insert(farm.clone(), neighbors_vec);
        }

        // Parse cost
        let cost = json["initial_state"]["cost"].as_i64().unwrap() as i32;

        // Parse goal thresholds
        let goal_thresholds = json["goal"]["x_thresholds"].as_object().unwrap();
        let mut parsed_thresholds = BTreeMap::new();
        for (farm, threshold) in goal_thresholds {
            parsed_thresholds.insert(farm.clone(), threshold.as_i64().unwrap() as i32);
        }

        // Parse weighted sum goal
        let weights = json["goal"]["weighted_sum_goal"]["weights"].as_object().unwrap();
        let mut parsed_weights = BTreeMap::new();
        for (farm, weight) in weights {
            parsed_weights.insert(farm.clone(), weight.as_f64().unwrap());
        }
        let weighted_sum_goal = WeightedSumGoal {
            weights: parsed_weights,
            threshold: json["goal"]["weighted_sum_goal"]["threshold"].as_f64().unwrap(),
        };

        // Construct and return the FarmProblem instance
        FarmProblem {
            farms,
            x_values: parsed_x_values,
            adjacencies: parsed_adjacencies,
            cost,
            goal_thresholds: parsed_thresholds,
            weighted_sum_goal,
        }
    }

    pub fn create_initial_state(&self) -> State {
        let mut state = State::new();

        // Insert x_values into the state
        for (farm, x_val) in &self.x_values {
            state.insert_field(farm.clone(), Value::Int(*x_val));
        }

        // Insert adjacencies into the state
        let mut adjacency_map = BTreeMap::new();
        for (farm, neighbors) in &self.adjacencies {
            adjacency_map.insert(farm.clone(), neighbors.clone());
        }
        state.insert_field("adjacencies".to_string(), Value::MapToVecString(adjacency_map));

        // Insert goal thresholds into the state
        let mut threshold_map = BTreeMap::new();
        for (farm, threshold) in &self.goal_thresholds {
            threshold_map.insert(farm.clone(), *threshold);
        }
        state.insert_field("goal_thresholds".to_string(), Value::MapToInt(threshold_map));

        // Insert weighted sum goal into the state
        let mut weighted_sum_map = BTreeMap::new();
        for (farm, weight) in &self.weighted_sum_goal.weights {
            weighted_sum_map.insert(farm.clone(), weight.to_string()); // Convert float to string
        }
        state.insert_field("weighted_sum_goal".to_string(), Value::MapToString(weighted_sum_map));

        state.insert_field("goal_threshold".to_string(), Value::Int(self.weighted_sum_goal.threshold as i32));

        state
    }

}


impl Problem for FarmProblem {
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

        // Retrieve adjacencies from the state
        let adjacencies = match state.get_field("adjacencies") {
            Some(Value::MapToVecString(adjacencies)) => adjacencies,
            _ => return actions, // If adjacencies are missing or of the wrong type, return no actions
        };

        // Retrieve x_values from the state
        for (farm, _) in &self.x_values {
            let x_value = match state.get_field(farm) {
                Some(Value::Int(v)) => *v,
                _ => continue, // Skip if x_value is missing or not an integer
            };

            // Check neighbors for possible actions
            if let Some(neighbors) = adjacencies.get(farm) {
                for neighbor in neighbors {
                    if x_value >= 4 {
                        let mut params = HashMap::new();
                        params.insert("farm0".to_string(), Value::Text(farm.clone()));
                        params.insert("farm1".to_string(), Value::Text(neighbor.clone()));

                        actions.push(Action::new("move-fast".to_string(), 1, params));
                    }

                    if x_value >= 1 {
                        let mut params = HashMap::new();
                        params.insert("farm0".to_string(), Value::Text(farm.clone()));
                        params.insert("farm1".to_string(), Value::Text(neighbor.clone()));

                        actions.push(Action::new("move-slow".to_string(), 1, params));
                    }
                }
            }
        }
        println!("{:?}", actions);
        actions
    }




    fn apply_action(&self, state: &State, action: &Action) -> State {
        let mut new_state = state.clone();

        // Retrieve the parameters from the action using the updated keys
        let farm0 = match action.parameters.get("farm0") {
            Some(Value::Text(farm)) => farm.clone(),
            _ => return new_state, // If farm0 is not present or not a string, return unchanged state
        };

        let farm1 = match action.parameters.get("farm1") {
            Some(Value::Text(farm)) => farm.clone(),
            _ => return new_state, // If farm1 is not present or not a string, return unchanged state
        };

        // Retrieve x_values for both farms
        let x_farm0 = match new_state.get_field(&farm0) {
            Some(Value::Int(v)) => *v,
            _ => return new_state, // If x value for farm0 is missing, return unchanged state
        };

        let x_farm1 = match new_state.get_field(&farm1) {
            Some(Value::Int(v)) => *v,
            _ => return new_state, // If x value for farm1 is missing, return unchanged state
        };

        // Apply effects based on action type, ensuring x_farm0 stays non-negative
        if action.name == "move-fast" && x_farm0 >= 4 {
            new_state.insert_field(farm0.clone(), Value::Int(x_farm0 - 4));
            new_state.insert_field(farm1.clone(), Value::Int(x_farm1 + 2));

            if let Some(Value::Int(cost)) = new_state.get_field("cost") {
                new_state.insert_field("cost".to_string(), Value::Int(cost + 1));
            }
        } else if action.name == "move-slow" && x_farm0 >= 1 {
            new_state.insert_field(farm0.clone(), Value::Int(x_farm0 - 1));
            new_state.insert_field(farm1.clone(), Value::Int(x_farm1 + 1));
        }

        new_state
    }




    fn is_goal_state(&self, state: &State) -> bool {
        // Retrieve goal thresholds from the state
        let goal_thresholds = match state.get_field("goal_thresholds") {
            Some(Value::MapToInt(thresholds)) => thresholds,
            _ => panic!("There was a problem: goal_thresholds missing or invalid"),
        };

        // Check if all x_values meet or exceed their respective thresholds
        for (farm, threshold) in goal_thresholds {
            let x_value = match state.get_field(farm) {
                Some(Value::Int(v)) => *v,
                _ => panic!("There was a problem: missing x_value for farm {}", farm),
            };
            if x_value < *threshold {
                return false; // If any farm's x_value is below the threshold, it's not a goal state
            }
        }

        // Retrieve weighted sum goal from the state
        let weighted_sum_goal = match state.get_field("weighted_sum_goal") {
            Some(Value::MapToString(weights)) => weights,
            _ => panic!("There was a problem: weighted_sum_goal missing or invalid"),
        };

        let mut total_weighted_sum = 0.0;
        for (farm, weight_str) in weighted_sum_goal {
            let weight = match weight_str.parse::<f64>() {
                Ok(w) => w,
                _ => panic!("There was a problem: parsing the weight for farm {}", farm),
            };

            let x_value = match state.get_field(farm) {
                Some(Value::Int(v)) => *v as f64,
                _ => panic!("There was a problem: missing x_value for farm {}", farm),
            };

            total_weighted_sum += weight * x_value;
        }

        // Retrieve the goal threshold for the weighted sum
        let goal_threshold = match state.get_field("goal_threshold") {
            Some(Value::Int(threshold)) => *threshold as f64,
            _ => panic!("There was a problem: goal_threshold missing or invalid"),
        };

        // Check if the total weighted sum meets or exceeds the goal threshold
        total_weighted_sum >= goal_threshold
    }

    fn heuristic(&self, state: &State) -> f64 {
        let mut heuristic_value = 0.0;

        // Retrieve goal thresholds
        let goal_thresholds = match state.get_field("goal_thresholds") {
            Some(Value::MapToInt(thresholds)) => thresholds,
            _ => {
                println!("Error: Goal thresholds are missing or invalid.");
                std::process::exit(1);
            }
        };

        // Calculate how far each farm's x_value is from the goal threshold
        for (farm, threshold) in goal_thresholds {
            let x_value = match state.get_field(farm) {
                Some(Value::Int(v)) => *v,
                _ => {
                    println!("Error: x_value for farm {} is missing or invalid.", farm);
                    std::process::exit(1);
                }
            };

            if x_value < *threshold {
                heuristic_value += (*threshold - x_value) as f64;
            }
        }

        // Retrieve weighted sum goal
        let weighted_sum_goal = match state.get_field("weighted_sum_goal") {
            Some(Value::MapToString(weights)) => weights,
            _ => {
                println!("Error: Weighted sum goal is missing or invalid.");
                std::process::exit(1);
            }
        };

        let mut total_weighted_sum = 0.0;
        for (farm, weight_str) in weighted_sum_goal {
            let weight = match weight_str.parse::<f64>() {
                Ok(w) => w,
                _ => {
                    println!("Error: Failed to parse weight for farm {}.", farm);
                    std::process::exit(1);
                }
            };

            let x_value = match state.get_field(farm) {
                Some(Value::Int(v)) => *v as f64,
                _ => {
                    println!("Error: x_value for farm {} is missing or invalid.", farm);
                    std::process::exit(1);
                }
            };

            total_weighted_sum += weight * x_value;
        }

        // Retrieve the goal threshold for the weighted sum
        let goal_threshold = match state.get_field("goal_threshold") {
            Some(Value::Int(threshold)) => *threshold as f64,
            _ => {
                println!("Error: Goal threshold for weighted sum is missing or invalid.");
                std::process::exit(1);
            }
        };

        // Calculate how far the weighted sum is from the goal threshold
        if total_weighted_sum < goal_threshold {
            heuristic_value += goal_threshold - total_weighted_sum;
        }
        // if heuristic_value < 150.0 {
        //     println!("{:?}", state);
        //     println!("Heuristic value: {}", heuristic_value);
        // }

        heuristic_value.max(0.0)
    }

}
