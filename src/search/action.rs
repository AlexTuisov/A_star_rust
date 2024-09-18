#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub name: String,    // The name of the action
    pub cost: i32,       // The cost of the action
}

impl Action {
    pub fn new(name: String, cost: i32) -> Self {
        Action { name, cost }
    }
}


