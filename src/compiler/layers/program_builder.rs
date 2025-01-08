use crate::machine::instruction::Instruction;
use crate::machine::state::State;

pub trait ProgramBuilder {
    fn allocate_state(&mut self) -> State;
    fn get_halt_state(&self) -> State;
    fn get_instruction(&self, state: State, read_bit: bool) -> Option<&Instruction>;
    fn add_instruction(&mut self, instruction: Instruction);

    fn allocate_states(&mut self, count: usize) -> Vec<State> {
        (0..count).map(|_| self.allocate_state()).collect()
    }
    
    fn add_instructions(&mut self, instructions: &[Instruction]) {
        instructions.iter().for_each(|i| self.add_instruction(*i));
    }
}