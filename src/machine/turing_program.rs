use std::collections::HashMap;
use std::fmt::{Display, Formatter};
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

impl Display for TuringProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut sorted_instructions: Vec<&Instruction> = self.instructions.values().collect();
        sorted_instructions.sort_by_key(|instr| (instr.current_state.get(), instr.get_read_bit_number()));
        
        for (i, instruction) in sorted_instructions.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", instruction.get_formal_string())?;
        }
        Ok(())
    }
}