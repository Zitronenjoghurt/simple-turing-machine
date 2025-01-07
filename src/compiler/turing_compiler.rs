use crate::compiler::layers::primitive::PrimitiveLayer;
use crate::compiler::layers::program_builder::ProgramBuilder;
use crate::machine::instruction::Instruction;
use crate::machine::state::State;
use crate::machine::turing_program::TuringProgram;

#[derive(Debug, Default)]
pub struct TuringCompiler {
    current_state: usize,
    program: TuringProgram
}

impl TuringCompiler {
    pub fn get_program(&self) -> TuringProgram {
        self.program.clone()
    }
}

impl ProgramBuilder for TuringCompiler {
    fn allocate_state(&mut self) -> State {
        let new_state = self.current_state;
        self.current_state += 1;
        State::new(new_state)
    }
    
    fn get_halt_state(&self) -> State {
        State::new(usize::MAX)
    }

    fn add_instruction(&mut self, instruction: Instruction) {
        self.program.add_instruction(instruction);
    }

    fn add_instructions(&mut self, instructions: &[Instruction]) {
        instructions.iter().for_each(|i| self.add_instruction(*i));
    }
}

impl PrimitiveLayer for TuringCompiler {}