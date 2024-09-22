use std::cell::RefCell;
use std::rc::{Rc, Weak};
use crate::search::{state::State, action::Action};


#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub state: State,
    pub parent: Option<usize>,
    pub children: Vec<usize>,
    pub action: Option<Action>,
    pub cost: i32,
}

impl Node {
    pub fn new_empty(state: State) -> Self {
        Node {
            state,
            parent: None,
            children: Vec::new(),
            action: None,
            cost: 0,

        }
    }
}

