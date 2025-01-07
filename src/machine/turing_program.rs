use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::machine::instruction::Instruction;
use crate::machine::state::State;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TuringProgram {
    pub instructions: HashMap<(State, bool), Instruction>
}

impl TuringProgram {
    pub fn get(&self, state: State, read_bit: bool) -> Option<&Instruction> {
        self.instructions.get(&(state, read_bit))
    }
    
    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.insert(
            (instruction.current_state, instruction.read_bit),
            instruction
        );
    }
}