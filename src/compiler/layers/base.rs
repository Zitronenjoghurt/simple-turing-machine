use crate::compiler::layers::primitive::PrimitiveLayer;
use crate::enums::movement::Movement;
use crate::machine::state::State;

pub trait BaseLayer: PrimitiveLayer {
    /// Creates a chained loop which will repeat a given build instruction n times.
    ///
    /// # Arguments
    ///
    /// * `n`: How many times the build instruction should be repeated.
    /// * `start_state`: The start state of the loop.
    /// * `end_state`: The end state of the loop.
    /// * `build_iteration`: A function which takes in a start_state and end_state and returns a start_state and end_state.
    ///
    /// returns: (State, State) = The start and end state of the loop
    ///
    /// # Examples
    /// Repeats move_right x times.
    /// ```
    /// let (start_loop_state, end_loop_state) = self.chained_loop(
    ///      x,
    ///      Some(start_state),
    ///      Some(end_state),
    ///      |compiler, iteration_start, iteration_end| {
    ///          compiler.move_right(iteration_start, iteration_end)
    ///      },
    /// );
    /// ```
    fn chained_loop(
        &mut self,
        n: usize,
        start_state: Option<State>,
        end_state: Option<State>,
        build_iteration: impl Fn(&mut Self, Option<State>, Option<State>) -> (State, State),
    ) -> (State, State) {
        if n == 1 {
            return build_iteration(self, start_state, end_state);
        }

        let mut prev_iter_end: Option<State> = None;
        let mut loop_start: Option<State> = None;
        let mut loop_end: Option<State> = None;

        for i in 0..n {
            let (iter_start, iter_end) = if i == 0 {
                build_iteration(self, start_state, None)
            } else if i == (n-1) {
                build_iteration(self, prev_iter_end, end_state)
            } else {
                build_iteration(self, prev_iter_end, None)
            };

            if i == 0 {
                loop_start = Some(iter_start);
            }
            if i == n-1 {
                loop_end = Some(iter_end);
            }

            prev_iter_end = Some(iter_end);
        };

        (loop_start.unwrap(), loop_end.unwrap())
    }

    /// The current state will move the head by x to the right, then transition to the next state.
    fn move_right_x(&mut self, x: usize, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

         let (start_loop_state, end_loop_state) = self.chained_loop(
              x,
              Some(start_state),
              Some(end_state),
              |compiler, iteration_start, iteration_end| {
                  compiler.move_right(iteration_start, iteration_end)
              },
         );

        (start_loop_state, end_loop_state)
    }

    /// The current state will move the head by x to the left, then transition to the next state.
    fn move_left_x(&mut self, x: usize, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        let (start_loop_state, end_loop_state) = self.chained_loop(
            x,
            Some(start_state),
            Some(end_state),
            |compiler, iteration_start, iteration_end| {
                compiler.move_left(iteration_start, iteration_end)
            },
        );

        (start_loop_state, end_loop_state)
    }

    /// The current state will move the head in the given direction till it finds the given bit, then transition to the next state.
    /// If the bit is not found this results in an endless loop. Counters are not a thing on this primitive level yet.
    fn scan_single(
        &mut self,
        target_bit: bool,
        scan_movement: Movement,
        final_movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>
    ) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        if target_bit {
            self.branch(
                Some(start_state),
                Some(end_state),
                Some(start_state),
                final_movement,
                scan_movement
            );
        } else {
            self.branch(
                Some(start_state),
                Some(start_state),
                Some(end_state),
                scan_movement,
                final_movement
            );
        }
        
        (start_state, end_state)
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::layers::program_builder::ProgramBuilder;
    use crate::compiler::turing_compiler::TuringCompiler;
    use crate::machine::turing_machine::TuringMachine;
    use crate::machine::turing_tape::TuringTape;
    use super::*;
    
    #[test]
    fn test_chained_loop() {
        let mut compiler = TuringCompiler::default();
        let start_state = compiler.allocate_state();
        let done = compiler.halt(None);
        
        compiler.chained_loop(
            3,
            Some(start_state),
            Some(done),
            |compiler, iteration_start, iteration_end| {
                let new_state = compiler.allocate_state();
                compiler.mark(iteration_start, Some(new_state));
                compiler.unmark(Some(new_state), iteration_end)
            }
        );
        
        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.program_step();
        assert!(tm.read());
        tm.program_step();
        assert!(!tm.read());
        tm.program_step();
        assert!(tm.read());
        tm.program_step();
        assert!(!tm.read());
        tm.program_step();
        assert!(tm.read());
        tm.program_step();
        assert!(!tm.read());
        tm.program_step();
        assert!(!tm.read());
    }
    
    #[test]
    fn test_move_right_x() {
        let mut compiler = TuringCompiler::default();
        let move_state = compiler.allocate_state();
        let done = compiler.halt(None);
        
        compiler.move_right_x(53, Some(move_state), Some(done));
        
        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.run_program();
        assert_eq!(tm.head, 53);
    }

    #[test]
    fn test_move_left_x() {
        let mut compiler = TuringCompiler::default();
        let move_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.move_left_x(5, Some(move_state), Some(done));

        let mut tm = TuringMachine::default().with_program(compiler.get_program());
        tm.run_program();
        assert_eq!(tm.head, 3);
    }
    
    #[test]
    fn test_scan_single() {
        let mut compiler = TuringCompiler::default();
        let scan_state = compiler.allocate_state();
        let done = compiler.halt(None);
        
        compiler.scan_single(true, Movement::Right, Movement::Stay, Some(scan_state), Some(done));
        
        let mut tape = TuringTape::default();
        tape.set(2763);
        let mut tm = TuringMachine::default().with_program(compiler.get_program()).with_tape(tape);
        tm.run_program();
        assert_eq!(tm.head, 2763);
    }
}