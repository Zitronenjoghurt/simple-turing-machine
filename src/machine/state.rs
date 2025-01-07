use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct State(usize);

impl State {
    pub fn new(value: usize) -> Self {
        State(value)
    }

    pub fn get(&self) -> usize {
        self.0
    }
}