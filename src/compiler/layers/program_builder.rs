use crate::machine::instruction::Instruction;
use crate::machine::state::State;

pub trait ProgramBuilder {
    fn allocate_state(&mut self) -> State;
    fn get_halt_state(&self) -> State;
    fn add_instruction(&mut self, instruction: Instruction);
    fn add_instructions(&mut self, instructions: &[Instruction]);
}