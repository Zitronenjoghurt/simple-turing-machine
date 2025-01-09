use crate::compiler::layers::program_builder::ProgramBuilder;
use crate::enums::movement::Movement;
use crate::machine::instruction::Instruction;
use crate::machine::state::State;

pub trait PrimitiveLayer: ProgramBuilder {
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

    /// The given state will transition to state_marked if the current bit is 1 else to state_unmarked.
    fn branch(
        &mut self,
        current_state: Option<State>,
        state_marked: Option<State>,
        state_unmarked: Option<State>,
        movement_marked: Movement,
        movement_unmarked: Movement
    ) -> (State, State, State) {
        let branch_state = current_state.unwrap_or_else(|| self.allocate_state());
        let state_marked = state_marked.unwrap_or_else(|| self.allocate_state());
        let state_unmarked = state_unmarked.unwrap_or_else(|| self.allocate_state());

        let instruction_0 = Instruction::new(branch_state, false, false)
            .with_next_state(state_unmarked)
            .with_movement(movement_unmarked);
        let instruction_1 = Instruction::new(branch_state, true, true)
            .with_next_state(state_marked)
            .with_movement(movement_marked);
        self.add_instructions(&[instruction_0, instruction_1]);

        (branch_state, state_marked, state_unmarked)
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

    fn mark_and_move_right(&mut self, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let current_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());

        let instruction_0 = Instruction::new(current_state, false, true)
            .with_movement(Movement::Right)
            .with_next_state(next_state);
        let instruction_1 = Instruction::new(current_state, true, true)
            .with_movement(Movement::Right)
            .with_next_state(next_state);
        self.add_instructions(&[instruction_0, instruction_1]);

        (current_state, next_state)
    }

    fn mark_and_move_left(&mut self, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let current_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());

        let instruction_0 = Instruction::new(current_state, false, true)
            .with_movement(Movement::Left)
            .with_next_state(next_state);
        let instruction_1 = Instruction::new(current_state, true, true)
            .with_movement(Movement::Left)
            .with_next_state(next_state);
        self.add_instructions(&[instruction_0, instruction_1]);

        (current_state, next_state)
    }

    fn unmark_and_move_right(&mut self, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let current_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());

        let instruction_0 = Instruction::new(current_state, false, false)
            .with_movement(Movement::Right)
            .with_next_state(next_state);
        let instruction_1 = Instruction::new(current_state, true, false)
            .with_movement(Movement::Right)
            .with_next_state(next_state);
        self.add_instructions(&[instruction_0, instruction_1]);

        (current_state, next_state)
    }

    fn unmark_and_move_left(&mut self, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let current_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());

        let instruction_0 = Instruction::new(current_state, false, false)
            .with_movement(Movement::Left)
            .with_next_state(next_state);
        let instruction_1 = Instruction::new(current_state, true, false)
            .with_movement(Movement::Left)
            .with_next_state(next_state);
        self.add_instructions(&[instruction_0, instruction_1]);

        (current_state, next_state)
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::turing_compiler::TuringCompiler;
    use crate::machine::turing_machine::TuringMachine;
    use crate::machine::turing_tape::TuringTape;
    use super::*;

    #[test]
    fn test_move_left() {
        let mut compiler = TuringCompiler::default();

        let move_state = compiler.allocate_state();
        let done = compiler.halt(None);
        compiler.move_left(Some(move_state), Some(done));

        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.run_program();

        assert_eq!(tm.head, 7);
    }

    #[test]
    fn test_move_right() {
        let mut compiler = TuringCompiler::default();

        let move_state = compiler.allocate_state();
        let done = compiler.halt(None);
        compiler.move_right(Some(move_state), Some(done));

        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.run_program();

        assert_eq!(tm.head, 1);
    }

    #[test]
    fn test_branch() {
        let mut compiler = TuringCompiler::default();

        let check_if_one = compiler.allocate_state();
        let move_left = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.branch(
            Some(check_if_one),
            Some(move_left),
            Some(check_if_one),
            Movement::Left,
            Movement::Right,
        );

        compiler.move_left(Some(move_left), Some(done));

        let mut tape = TuringTape::default();
        tape.set(5);

        let mut tm = TuringMachine::default()
            .with_program(compiler.get_program())
            .with_tape(tape);
        tm.program_step();
        assert_eq!(tm.head, 1);
        tm.program_step();
        assert_eq!(tm.head, 2);
        tm.program_step();
        assert_eq!(tm.head, 3);
        tm.program_step();
        assert_eq!(tm.head, 4);
        tm.program_step();
        assert_eq!(tm.head, 5);
        tm.program_step();
        assert_eq!(tm.head, 4);
        tm.program_step();
        assert_eq!(tm.head, 3);
    }

    #[test]
    fn test_mark() {
        let mut compiler = TuringCompiler::default();
        let mark_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.mark(Some(mark_state), Some(done));

        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.run_program();
        assert_eq!(tm.head, 0);
        assert!(tm.read());
    }
    
    #[test]
    fn test_unmark() {
        let mut compiler = TuringCompiler::default();
        let mark_state = compiler.allocate_state();
        let unmark_state = compiler.allocate_state();
        let done = compiler.halt(None);
        
        compiler.mark(Some(mark_state), Some(unmark_state));
        compiler.unmark(Some(unmark_state), Some(done));
        
        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.run_program();
        assert_eq!(tm.head, 0);
        assert!(!tm.read());
    }

    #[test]
    fn test_mark_and_move_right() {
        let mut compiler = TuringCompiler::default();
        
        let mark_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.mark_and_move_right(Some(mark_state), Some(done));
        
        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.run_program();
        assert!(tm.tape.read(0));
        assert!(!tm.tape.read(1));
        assert_eq!(tm.head, 1);
    }

    #[test]
    fn test_mark_and_move_left() {
        let mut compiler = TuringCompiler::default();
        
        let mark_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.mark_and_move_left(Some(mark_state), Some(done));
        
        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.run_program();
        // After moving left, more space left is allocated which shifts the previously marked index right by 8
        assert!(!tm.tape.read(7));
        assert!(tm.tape.read(8));
        assert_eq!(tm.head, 7);
    }
    
    #[test]
    fn test_unmark_and_move_right() {
        let mut compiler = TuringCompiler::default();
        
        let mark_state = compiler.allocate_state();
        let done = compiler.halt(None);
        
        compiler.unmark_and_move_right(Some(mark_state), Some(done));
        
        let mut tape = TuringTape::default();
        tape.set(0);
        let mut tm = TuringMachine::default().with_program(compiler.get_program()).with_tape(tape);
        assert!(tm.tape.read(0));
        tm.run_program();
        assert!(!tm.tape.read(0));
        assert!(!tm.tape.read(1));
        assert_eq!(tm.head, 1);
    }

    #[test]
    fn test_unmark_and_move_left() {
        let mut compiler = TuringCompiler::default();

        let mark_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.unmark_and_move_left(Some(mark_state), Some(done));

        let mut tape = TuringTape::default();
        tape.set(0);
        let mut tm = TuringMachine::default().with_program(compiler.get_program()).with_tape(tape);
        assert!(tm.tape.read(0));
        tm.run_program();
        assert!(!tm.tape.read(7));
        assert!(!tm.tape.read(8));
        assert_eq!(tm.head, 7);
    }
}