use crate::compiler::layers::program_builder::ProgramBuilder;
use crate::enums::movement::Movement;
use crate::machine::instruction::Instruction;
use crate::machine::state::State;

pub trait PrimitiveLayer: ProgramBuilder {
    /// The given state will transition to state_marked if the current bit is 1 else to state_unmarked.
    fn branch(&mut self, current_state: Option<State>, state_marked: Option<State>, state_unmarked: Option<State>) -> State {
        let branch_state = current_state.unwrap_or_else(|| self.allocate_state());
        let state_marked = state_marked.unwrap_or_else(|| self.allocate_state());
        let state_unmarked = state_unmarked.unwrap_or_else(|| self.allocate_state());

        let instruction_0 = Instruction::new(branch_state, false, false)
            .with_next_state(state_unmarked);
        let instruction_1 = Instruction::new(branch_state, true, true)
            .with_next_state(state_marked);
        self.add_instructions(&[instruction_0, instruction_1]);
        
        branch_state
    }

    /// The given state will move the head left, then transition to the next state.
    fn move_left(&mut self, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let move_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());
        
        let instruction_0 = Instruction::new(move_state, false, false)
            .with_next_state(next_state)
            .with_movement(Movement::Left);
        let instruction_1 = Instruction::new(move_state, true, true)
            .with_next_state(next_state)
            .with_movement(Movement::Left);
        self.add_instructions(&[instruction_0, instruction_1]);
        
        (move_state, next_state)
    }

    /// The given state will move the head right, then transition to the next state.
    fn move_right(&mut self, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let move_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());

        let instruction_0 = Instruction::new(move_state, false, false)
            .with_next_state(next_state)
            .with_movement(Movement::Right);
        let instruction_1 = Instruction::new(move_state, true, true)
            .with_next_state(next_state)
            .with_movement(Movement::Right);
        self.add_instructions(&[instruction_0, instruction_1]);
        
        (move_state, next_state)
    }

    /// The given state will mark the current bit, then transition to the next state.
    fn mark(&mut self, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let marking_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());
        
        let instruction_0 = Instruction::new(marking_state, false, true)
            .with_next_state(next_state);
        let instruction_1 = Instruction::new(marking_state, true, true)
            .with_next_state(next_state);
        self.add_instructions(&[instruction_0, instruction_1]);
        
        (marking_state, next_state)
    }

    /// The given state will unmark the current bit, then transition to the next state.
    fn unmark(&mut self, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let unmarking_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());

        let instruction_0 = Instruction::new(unmarking_state, false, false)
            .with_next_state(next_state);
        let instruction_1 = Instruction::new(unmarking_state, true, false)
            .with_next_state(next_state);
        self.add_instructions(&[instruction_0, instruction_1]);

        (unmarking_state, next_state)
    }
    
    /// The given state will transition to halt.
    fn halt(&mut self, current_state: Option<State>) -> State {
        let new_state = current_state.unwrap_or_else(|| self.allocate_state());
        let halt_state = self.get_halt_state();
        
        let instruction_0 = Instruction::new(new_state, false, false)
            .with_next_state(halt_state);
        let instruction_1 = Instruction::new(new_state, true, true)
            .with_next_state(halt_state);
        self.add_instructions(&[instruction_0, instruction_1]);
        
        new_state
    }
}