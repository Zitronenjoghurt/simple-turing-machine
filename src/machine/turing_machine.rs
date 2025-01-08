use std::thread::sleep;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::enums::display_style::DisplayStyle;
use crate::enums::movement::Movement;
use crate::machine::instruction::Instruction;
use crate::machine::state::State;
use crate::machine::turing_program::TuringProgram;
use crate::machine::turing_tape::TuringTape;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TuringMachine {
    tape: TuringTape,
    head: usize,
    head_max: usize,
    state: State, // Acts like a program counter
    program: TuringProgram,
    delay: Duration,
    debug_mode: bool,
    display_style: DisplayStyle,
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
            state: State::default(),
            head_max,
            program,
            delay: Duration::from_millis(0),
            debug_mode: false,
            display_style: DisplayStyle::None,
            current_repeats: 0,
            max_repeat: 100
        }
    }
    
    pub fn with_tape(mut self, tape: TuringTape) -> Self {
        self.tape = tape;
        self
    }

    pub fn with_debug_mode(mut self, display_style: DisplayStyle, delay: Duration) -> Self {
        self.display_style = display_style;
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
            match self.display_style {
                DisplayStyle::Formal => {
                    println!(
                        "Head: {} | {}",
                        self.head,
                        instruction.get_formal_string()
                    );
                },
                DisplayStyle::Visual => {
                    println!("{}", self.tape.to_string(Some(self.head)))
                },
                DisplayStyle::VisualFormal => {
                    println!(
                        "{} | Head: {} | {}",
                        self.tape.to_string(Some(self.head)),
                        self.head,
                        instruction.get_formal_string()
                    )
                },
                _ => {}
            }
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

    pub fn process_instruction(&mut self, instruction: Instruction) -> State {
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
        let instruction = Instruction::new(State::new(0), false, true)
            .with_movement(Movement::Right)
            .with_next_state(State::new(0));

        // A simple program which will turn every bit on the tape to 1 till it reaches the end
        let mut program = TuringProgram::default();
        program.add_instruction(instruction);

        let mut tm = TuringMachine::new(program, 2);
        tm.run_program();
        
        tm.head = 0;
        for _ in 0..=15 {
            assert!(tm.read());
            tm.move_right();
        }
    }
}