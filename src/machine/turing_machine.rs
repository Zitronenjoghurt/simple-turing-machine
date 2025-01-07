use std::thread::sleep;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::enums::movement::Movement;
use crate::machine::instruction::Instruction;
use crate::machine::turing_program::TuringProgram;
use crate::machine::turing_tape::TuringTape;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TuringMachine {
    tape: TuringTape,
    head: usize,
    head_max: usize,
    state: usize, // Acts like a program counter
    program: TuringProgram,
    delay: Duration,
    debug_mode: bool,
    current_repeats: usize,
    max_repeat: usize, // Loop failsafe
}

impl TuringMachine {
    pub fn new(
        program: TuringProgram,
        tape_size_bytes: usize,
    ) -> Self {
        let head_max = (tape_size_bytes * 8) - 1;

        Self {
            tape: TuringTape::new(tape_size_bytes),
            head: 0,
            state: 0,
            head_max,
            program,
            delay: Duration::from_millis(0),
            debug_mode: false,
            current_repeats: 0,
            max_repeat: 100
        }
    }

    pub fn with_debug_mode(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self.debug_mode = true;
        self
    }

    fn clamp_head(&mut self) {
        if self.head > self.head_max {
            self.head = self.head_max;
        }
    }

    pub fn read(&self) -> bool {
        self.tape.read(self.head).unwrap()
    }

    pub fn set(&mut self) {
        self.tape.set(self.head).unwrap()
    }

    pub fn unset(&mut self) {
        self.tape.unset(self.head).unwrap()
    }

    pub fn move_right(&mut self) {
        self.head += 1;
        self.clamp_head();
    }

    pub fn move_left(&mut self) {
        self.head = self.head.saturating_sub(1);
    }

    pub fn program_step(&mut self) -> bool {
        let current_instruction = self.program.get(self.state, self.read());
        let instruction = match current_instruction {
            None => return false,
            Some(inst) => *inst,
        };

        if self.debug_mode {
            println!(
                "Head: {} | q={}, σ={} => q'={}, σ'={}, D={}",
                self.head,
                instruction.current_state,
                instruction.get_read_bit_number(),
                instruction.next_state,
                instruction.get_write_bit_number(),
                instruction.movement.get_code_string()
            );
        }

        let next_state = self.process_instruction(instruction);
        if self.state == next_state {
            self.current_repeats += 1;
        }

        if self.current_repeats >= self.max_repeat {
            panic!("Max loop count exceeded")
        }

        self.state = next_state;

        true
    }

    pub fn process_instruction(&mut self, instruction: Instruction) -> usize {
        if instruction.write_bit {
            self.set();
        } else {
            self.unset();
        }

        match instruction.movement {
            Movement::Left => self.move_left(),
            Movement::Right => self.move_right(),
            _ => {}
        };

        instruction.next_state
    }

    pub fn run_program(&mut self) {
        while self.program_step() {
            if self.debug_mode {
                sleep(self.delay);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_functionalities() {
        let mut tm = TuringMachine::new(
            TuringProgram::default(),
            10
        );

        assert_eq!(tm.head, 0);
        assert!(!tm.read());
        tm.set();
        assert!(tm.read());
        tm.unset();
        assert!(!tm.read());

        tm.move_right();
        assert_eq!(tm.head, 1);
        assert!(!tm.read());
        tm.set();
        assert!(tm.read());
        tm.move_left();
        assert_eq!(tm.head, 0);
        assert!(!tm.read());
        tm.set();

        tm.move_left();
        assert_eq!(tm.head, 0);
        assert!(tm.read());
    }

    #[test]
    fn test_run_program() {
        let instruction = Instruction::new(0, false, true)
            .with_movement(Movement::Right)
            .with_next_state(0);

        // A simple program which will turn every bit on the tape to 1 till it reaches the end
        let mut program = TuringProgram::default();
        program.add_instruction(instruction);

        let mut tm = TuringMachine::new(program, 2)
            .with_debug_mode(Duration::from_millis(0));
        
        tm.run_program();
        
        tm.head = 0;
        for _ in 0..=15 {
            assert!(tm.read());
            tm.move_right();
        }
    }
}