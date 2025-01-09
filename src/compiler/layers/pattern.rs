use crate::compiler::layers::base::BaseLayer;
use crate::compiler::structures::pattern::Pattern;
use crate::enums::movement::Movement;
use crate::machine::state::State;

pub trait PatternLayer: BaseLayer {
    fn write_pattern(
        &mut self,
        pattern: Pattern,
        write_movement: Movement,
        final_movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>,
    ) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        self.chained_loop(
            pattern.len(),
            Some(start_state),
            Some(end_state),
            |compiler, i, iter_start, iter_end| {
                // Invert pattern when writing it from the left
                let current_target_bit = match write_movement {
                    Movement::Left => pattern.get_at(pattern.len() - i - 1),
                    _ => pattern.get_at(i)
                };
                if i < pattern.len() - 1 {
                    compiler.write_and_move(
                        current_target_bit,
                        write_movement,
                        iter_start,
                        iter_end,
                    )
                } else {
                    compiler.write_and_move(
                        current_target_bit,
                        final_movement,
                        iter_start,
                        iter_end,
                    )
                }
            }
        )
    }

    fn scan_pattern(
        &mut self,
        pattern: Pattern,
        scan_movement: Movement,
        final_movement: Movement,
        current_state: Option<State>,
        next_state: Option<State>,
    ) -> (State, State) {
        let start_state = current_state.unwrap_or_else(|| self.allocate_state());
        let end_state = next_state.unwrap_or_else(|| self.allocate_state());

        self.chained_loop(
            pattern.len(),
            Some(start_state),
            Some(end_state),
            |compiler, i, iter_start, iter_end| {
                // Invert pattern when scanning it from the left
                let current_target_bit = match scan_movement {
                    Movement::Left => pattern.get_at(pattern.len() - i - 1),
                    _ => pattern.get_at(i)
                };
                let (branch_state, match_state, _) =
                    if i < pattern.len() - 1 {
                        compiler.branch_when(
                            current_target_bit,
                            scan_movement,
                            scan_movement,
                            iter_start,
                            iter_end,
                            Some(start_state)
                        )
                    } else {
                        compiler.branch_when(
                            current_target_bit,
                            final_movement,
                            scan_movement,
                            iter_start,
                            iter_end,
                            Some(start_state)
                        )
                    };
                (branch_state, match_state)
            }
        );

        (start_state, end_state)
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::layers::primitive::PrimitiveLayer;
    use crate::compiler::layers::program_builder::ProgramBuilder;
    use crate::compiler::turing_compiler::TuringCompiler;
    use crate::machine::turing_machine::TuringMachine;
    use crate::machine::turing_tape::TuringTape;
    use super::*;

    #[test]
    fn test_write_pattern() {
        let pattern = Pattern::new(vec![true, true, false, false, false, true, false, true]);
        let mut tm = TuringMachine::default();

        // Writing left to right
        let mut compiler = TuringCompiler::default();

        let write_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.write_pattern(pattern.clone(), Movement::Right, Movement::Stay, Some(write_state), Some(done));

        tm.set_program(compiler.get_program());
        tm.run_program();
        assert_eq!(tm.head, 7);
        assert!(tm.tape.read(0));
        assert!(tm.tape.read(1));
        assert!(!tm.tape.read(2));
        assert!(!tm.tape.read(3));
        assert!(!tm.tape.read(4));
        assert!(tm.tape.read(5));
        assert!(!tm.tape.read(6));
        assert!(tm.tape.read(7));

        // Reset tm
        tm.reset_state_information();

        // Writing right to left
        let mut compiler = TuringCompiler::default();

        let write_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.write_pattern(pattern, Movement::Left, Movement::Stay, Some(write_state), Some(done));

        tm.set_program(compiler.get_program());
        tm.run_program();
        assert_eq!(tm.head, 1);
        assert!(tm.tape.read(1));
        assert!(tm.tape.read(2));
        assert!(!tm.tape.read(3));
        assert!(!tm.tape.read(4));
        assert!(!tm.tape.read(5));
        assert!(tm.tape.read(6));
        assert!(!tm.tape.read(7));
        assert!(tm.tape.read(8));
    }

    #[test]
    fn test_scan_pattern() {
        let pattern = Pattern::new(vec![true, true, false, false, false, true, false, true]);
        let mut tape = TuringTape::default();
        tape.set(4);
        tape.set(5);
        tape.set(9);
        tape.set(11);
        let mut tm = TuringMachine::default();

        // Scan left to right
        let mut compiler = TuringCompiler::default();

        let scan_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.scan_pattern(pattern.clone(), Movement::Right, Movement::Stay, Some(scan_state), Some(done));
        tm.set_tape(tape.clone());
        tm.set_program(compiler.get_program());
        tm.run_program();
        assert_eq!(tm.head, 11);

        // Reset tm
        tm.reset_state_information();

        // Scan right to left
        let mut compiler = TuringCompiler::default();

        let scan_state = compiler.allocate_state();
        let done = compiler.halt(None);

        compiler.scan_pattern(pattern.clone(), Movement::Left, Movement::Stay, Some(scan_state), Some(done));
        tm.set_tape(tape);
        tm.set_program(compiler.get_program());
        tm.set_head(15);
        tm.run_program();
        assert_eq!(tm.head, 4);
    }
}