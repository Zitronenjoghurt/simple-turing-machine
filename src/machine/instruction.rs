use serde::{Deserialize, Serialize};
use crate::enums::movement::Movement;
use crate::machine::state::State;

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone)]
pub struct Instruction {
    pub current_state: State,
    pub read_bit: bool,
    pub write_bit: bool,
    pub movement: Movement,
    pub next_state: State,
}

impl Instruction {
    pub fn new(current_state: State, read_bit: bool, write_bit: bool) -> Self {
        Self {
            current_state,
            read_bit,
            write_bit,
            movement: Movement::default(),
            next_state: current_state,
        }
    }
    
    pub fn with_movement(mut self, movement: Movement) -> Self {
        self.movement = movement;
        self
    }
    
    pub fn with_next_state(mut self, next_state: State) -> Self {
        self.next_state = next_state;
        self
    }
    
    pub fn get_read_bit_number(&self) -> usize {
        if self.read_bit {
            1
        } else {
            0
        }
    }
    
    pub fn get_write_bit_number(&self) -> usize {
        if self.write_bit {
            1
        } else {
            0
        }
    }

    pub fn get_formal_string(&self) -> String {
        format!(
            "(q={}, σ={}) => (q'={}, σ'={}, D={})",
            self.current_state.get(),
            self.get_read_bit_number(),
            self.next_state.get(),
            self.get_write_bit_number(),
            self.movement.get_code_string()
        )
    }
}