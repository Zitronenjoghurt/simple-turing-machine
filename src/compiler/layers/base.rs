use crate::compiler::layers::primitive::PrimitiveLayer;
use crate::enums::movement::Movement;
use crate::machine::state::State;

pub trait BaseLayer: PrimitiveLayer {
    /// Creates a chained loop which will repeat the a given build instruction n times.
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

    /// The current state will move in the given direction till it finds the given bit, then transition to the next state.
    /// If the bit is not found this results in an endless loop. Counters are not a thing on this primitive level yet.
    fn scan_simple(&mut self, read_bit: bool, direction: Movement, current_state: Option<State>, next_state: Option<State>) -> (State, State) {
        let check_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());
        
        let move_state = self.allocate_state();

        if read_bit {
            self.branch(Some(check_state), Some(end_state), Some(move_state));
        } else {
            self.branch(Some(check_state), Some(move_state), Some(end_state));
        }
        
        if direction == Movement::Right {
            self.move_right(Some(move_state), Some(check_state));
        } else if direction == Movement::Left {
            self.move_left(Some(move_state), Some(check_state));
        }
        
        (check_state, end_state)
    }
}