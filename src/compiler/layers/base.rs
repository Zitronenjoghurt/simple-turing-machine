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
    /// * `build_iteration`: A function which takes in the current iteration count, a start_state and end_state and returns a start_state and end_state.
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
        build_iteration: impl Fn(&mut Self, usize, Option<State>, Option<State>) -> (State, State),
    ) -> (State, State) {
        if n == 1 {
            return build_iteration(self, 0, start_state, end_state);
        }

        let mut prev_iter_end: Option<State> = None;
        let mut loop_start: Option<State> = None;
        let mut loop_end: Option<State> = None;

        for i in 0..n {
            let (iter_start, iter_end) = if i == 0 {
                build_iteration(self, i, start_state, None)
            } else if i == (n-1) {
                build_iteration(self, i, prev_iter_end, end_state)
            } else {
                build_iteration(self, i, prev_iter_end, None)
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
              |compiler, _, iter_start, iter_end| {
                  compiler.move_right(iter_start, iter_end)
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
            |compiler, _, iter_start, iter_end| {
                compiler.move_left(iter_start, iter_end)
            },
        );

        (start_loop_state, end_loop_state)
    }

    fn move_in_direction(&mut self, movement: Movement, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        match movement {
            Movement::Right => self.move_right(current_state, next_state),
            Movement::Left => self.move_left(current_state, next_state),
            Movement::Stay => self.idle(current_state, next_state),
        }
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
            self.branch_move(
                Some(start_state),
                Some(end_state),
                Some(start_state),
                final_movement,
                scan_movement
            );
        } else {
            self.branch_move(
                Some(start_state),
                Some(start_state),
                Some(end_state),
                scan_movement,
                final_movement
            );
        }
        
        (start_state, end_state)
    }

    fn branch_when(
        &mut self,
        target_bit: bool,
        next_movement: Movement,
        else_movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>,
        else_state: Option<State>
    ) -> (State, State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let next_state = next_state.unwrap_or_else(|| self.allocate_state());
        let else_state = else_state.unwrap_or_else(|| self.allocate_state());

        if target_bit {
            self.branch_move(
                Some(start_state),
                Some(next_state),
                Some(else_state),
                next_movement,
                else_movement
            );
        } else {
            self.branch_move(
                Some(start_state),
                Some(else_state),
                Some(next_state),
                else_movement,
                next_movement
            );
        }

        (start_state, next_state, else_state)
    }

    fn write_and_move(
        &mut self,
        target_bit: bool,
        movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>,
    ) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        if target_bit {
            match movement {
                Movement::Right => self.mark_and_move_right(Some(start_state), Some(end_state)),
                Movement::Left => self.mark_and_move_left(Some(start_state), Some(end_state)),
                Movement::Stay => self.mark(Some(start_state), Some(end_state)),
            }
        } else {
            match movement {
                Movement::Right => self.unmark_and_move_right(Some(start_state), Some(end_state)),
                Movement::Left => self.unmark_and_move_left(Some(start_state), Some(end_state)),
                Movement::Stay => self.unmark(Some(start_state), Some(end_state)),
            }
        }
    }

    fn or(
        &mut self,
        movement: Movement,
        final_movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>,
    ) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        let read_0 = self.allocate_state();
        let read_1 = self.allocate_state();
        let result_0 = self.allocate_state();
        let result_1 = self.allocate_state();

        self.branch_move(Some(start_state), Some(read_1), Some(read_0), movement, movement);
        self.branch_move(Some(read_0), Some(result_1), Some(result_0), movement, movement);
        self.move_in_direction(movement, Some(read_1), Some(result_1));
        self.write_and_move(false, final_movement, Some(result_0), Some(end_state));
        self.write_and_move(true, final_movement, Some(result_1), Some(end_state));

        (start_state, end_state)
    }

    fn and(
        &mut self,
        movement: Movement,
        final_movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>,
    ) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        let read_0 = self.allocate_state();
        let read_1 = self.allocate_state();
        let result_0 = self.allocate_state();
        let result_1 = self.allocate_state();

        self.branch_move(Some(start_state), Some(read_1), Some(read_0), movement, movement);
        self.move_in_direction(movement, Some(read_0), Some(result_0));
        self.branch_move(Some(read_1), Some(result_1), Some(result_0), movement, movement);
        self.write_and_move(false, final_movement, Some(result_0), Some(end_state));
        self.write_and_move(true, final_movement, Some(result_1), Some(end_state));

        (start_state, end_state)
    }

    fn xor(
        &mut self,
        movement: Movement,
        final_movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>,
    ) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        let read_0 = self.allocate_state();
        let read_1 = self.allocate_state();
        let result_0 = self.allocate_state();
        let result_1 = self.allocate_state();

        self.branch_move(Some(start_state), Some(read_1), Some(read_0), movement, movement);
        self.branch_move(Some(read_0), Some(result_1), Some(result_0), movement, movement);
        self.branch_move(Some(read_1), Some(result_0), Some(result_1), movement, movement);
        self.write_and_move(false, final_movement, Some(result_0), Some(end_state));
        self.write_and_move(true, final_movement, Some(result_1), Some(end_state));

        (start_state, end_state)
    }

    /// Layout on the tape:
    /// |prev_carry|n1_1st_bit|n2_1st_bit|result_1st_bit|next_carry/prev_carry|n1_2nd_bit|...
    /// To be chained together the final movement has to be a stay, that way the next_carry will be the prev_carry of the following add.
    fn add(
        &mut self,
        movement: Movement,
        final_movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>,
    ) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        let pc0 = self.allocate_state();
        let pc1 = self.allocate_state();
        let pc0_n0 = self.allocate_state();
        let pc0_n1 = self.allocate_state();
        let pc1_n0 = self.allocate_state();
        let pc1_n1 = self.allocate_state();
        let r0_c0 = self.allocate_state();
        let r0_c1 = self.allocate_state();
        let r1_c0 = self.allocate_state();
        let r1_c1 = self.allocate_state();
        let c0 = self.allocate_state();
        let c1 = self.allocate_state();

        self.branch_move(Some(start_state), Some(pc1), Some(pc0), movement, movement);
        self.branch_move(Some(pc0), Some(pc0_n1), Some(pc0_n0), movement, movement);
        self.branch_move(Some(pc1), Some(pc1_n1), Some(pc1_n0), movement, movement);
        self.branch_move(Some(pc0_n0), Some(r1_c0), Some(r0_c0), movement, movement);
        self.branch_move(Some(pc0_n1), Some(r0_c1), Some(r1_c0), movement, movement);
        self.branch_move(Some(pc1_n0), Some(r0_c1), Some(r1_c0), movement, movement);
        self.branch_move(Some(pc1_n1), Some(r1_c1), Some(r0_c1), movement, movement);
        self.write_and_move(false, movement, Some(r0_c0), Some(c0));
        self.write_and_move(false, movement, Some(r0_c1), Some(c1));
        self.write_and_move(true, movement, Some(r1_c0), Some(c0));
        self.write_and_move(true, movement, Some(r1_c1), Some(c1));
        self.write_and_move(false, final_movement, Some(c0), Some(end_state));
        self.write_and_move(true, final_movement, Some(c1), Some(end_state));

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
            |compiler, _, iter_start, iter_end| {
                let new_state = compiler.allocate_state();
                compiler.mark(iter_start, Some(new_state));
                compiler.unmark(Some(new_state), iter_end)
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

    #[test]
    fn test_branch_when() {
        let mut compiler = TuringCompiler::default();
        let branch_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.branch_when(
            false,
            Movement::Right,
            Movement::Left,
            Some(branch_state),
            Some(branch_state),
            Some(done),
        );

        let mut tape = TuringTape::default();
        tape.set(1);
        let mut tm = TuringMachine::default().with_program(compiler.get_program()).with_tape(tape);
        tm.program_step();
        assert_eq!(tm.head, 1);
        tm.program_step();
        assert_eq!(tm.head, 0);
    }

    #[test]
    fn test_or() {
        let mut compiler = TuringCompiler::default();

        let start_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.chained_loop(
            4,
            Some(start_state),
            Some(done),
            |compiler, _, iter_start, iter_end| {
                compiler.or(Movement::Right, Movement::Right, iter_start, iter_end)
            }
        );

        // Basically a truth table of the inputs, just unrolled on the tape
        let mut tape = TuringTape::default();
        tape.set(3);
        tape.set(7);
        tape.set(9);
        tape.set(10);
        let mut tm = TuringMachine::default().with_program(compiler.get_program()).with_tape(tape);
        tm.run_program();

        assert!(!tm.tape.read(0));
        assert!(!tm.tape.read(1));
        assert!(!tm.tape.read(2));
        assert!(tm.tape.read(3));
        assert!(!tm.tape.read(4));
        assert!(tm.tape.read(5));
        assert!(!tm.tape.read(6));
        assert!(tm.tape.read(7));
        assert!(tm.tape.read(8));
        assert!(tm.tape.read(9));
        assert!(tm.tape.read(10));
        assert!(tm.tape.read(11));
        assert_eq!(tm.head, 12);
    }

    #[test]
    fn test_and() {
        let mut compiler = TuringCompiler::default();

        let start_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.chained_loop(
            4,
            Some(start_state),
            Some(done),
            |compiler, _, iter_start, iter_end| {
                compiler.and(Movement::Right, Movement::Right, iter_start, iter_end)
            }
        );

        // Basically a truth table of the inputs, just unrolled on the tape
        let mut tape = TuringTape::default();
        tape.set(3);
        tape.set(7);
        tape.set(9);
        tape.set(10);
        let mut tm = TuringMachine::default().with_program(compiler.get_program()).with_tape(tape);
        tm.run_program();

        assert!(!tm.tape.read(0));
        assert!(!tm.tape.read(1));
        assert!(!tm.tape.read(2));
        assert!(tm.tape.read(3));
        assert!(!tm.tape.read(4));
        assert!(!tm.tape.read(5));
        assert!(!tm.tape.read(6));
        assert!(tm.tape.read(7));
        assert!(!tm.tape.read(8));
        assert!(tm.tape.read(9));
        assert!(tm.tape.read(10));
        assert!(tm.tape.read(11));
        assert_eq!(tm.head, 12);
    }

    #[test]
    fn test_xor() {
        let mut compiler = TuringCompiler::default();

        let start_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.chained_loop(
            4,
            Some(start_state),
            Some(done),
            |compiler, _, iter_start, iter_end| {
                compiler.xor(Movement::Right, Movement::Right, iter_start, iter_end)
            }
        );

        // Basically a truth table of the inputs, just unrolled on the tape
        let mut tape = TuringTape::default();
        tape.set(3);
        tape.set(7);
        tape.set(9);
        tape.set(10);
        let mut tm = TuringMachine::default().with_program(compiler.get_program()).with_tape(tape);
        tm.run_program();

        assert!(!tm.tape.read(0));
        assert!(!tm.tape.read(1));
        assert!(!tm.tape.read(2));
        assert!(tm.tape.read(3));
        assert!(!tm.tape.read(4));
        assert!(tm.tape.read(5));
        assert!(!tm.tape.read(6));
        assert!(tm.tape.read(7));
        assert!(tm.tape.read(8));
        assert!(tm.tape.read(9));
        assert!(tm.tape.read(10));
        assert!(!tm.tape.read(11));
        assert_eq!(tm.head, 12);
    }

    #[test]
    fn test_add() {
        let mut compiler = TuringCompiler::default();

        let start_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.chained_loop(
            8,
            Some(start_state),
            Some(done),
            |compiler, _, iter_start, iter_end| {
                compiler.add(Movement::Right, Movement::Right, iter_start, iter_end)
            }
        );

        // Basically a truth table of the inputs, just unrolled on the tape
        let mut tape = TuringTape::default();
        tape.set(5);
        tape.set(11);
        tape.set(15);
        tape.set(16);
        tape.set(22);
        tape.set(25);
        tape.set(27);
        tape.set(31);
        tape.set(32);
        tape.set(35);
        tape.set(36);
        tape.set(37);
        let mut tm = TuringMachine::default().with_program(compiler.get_program()).with_tape(tape);
        tm.run_program();

        assert!(!tm.tape.read(0));
        assert!(!tm.tape.read(1));
        assert!(!tm.tape.read(2));
        assert!(!tm.tape.read(3));
        assert!(!tm.tape.read(4));
        assert!(tm.tape.read(5));
        assert!(!tm.tape.read(6));
        assert!(!tm.tape.read(7));
        assert!(tm.tape.read(8));
        assert!(!tm.tape.read(9));
        assert!(!tm.tape.read(10));
        assert!(tm.tape.read(11));
        assert!(!tm.tape.read(12));
        assert!(tm.tape.read(13));
        assert!(!tm.tape.read(14));
        assert!(tm.tape.read(15));
        assert!(tm.tape.read(16));
        assert!(!tm.tape.read(17));
        assert!(!tm.tape.read(18));
        assert!(tm.tape.read(19));
        assert!(!tm.tape.read(20));
        assert!(!tm.tape.read(21));
        assert!(tm.tape.read(22));
        assert!(tm.tape.read(23));
        assert!(!tm.tape.read(24));
        assert!(tm.tape.read(25));
        assert!(!tm.tape.read(26));
        assert!(tm.tape.read(27));
        assert!(!tm.tape.read(28));
        assert!(tm.tape.read(29));
        assert!(!tm.tape.read(30));
        assert!(tm.tape.read(31));
        assert!(tm.tape.read(32));
        assert!(!tm.tape.read(33));
        assert!(tm.tape.read(34));
        assert!(tm.tape.read(35));
        assert!(tm.tape.read(36));
        assert!(tm.tape.read(37));
        assert!(tm.tape.read(38));
        assert!(tm.tape.read(39));
        assert_eq!(tm.head, 40);
    }
}