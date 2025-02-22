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
    pub tape: TuringTape,
    pub head: usize,
    pub state: State, // Acts like a program counter
    pub program: TuringProgram,
    pub delay: Duration,
    pub debug_mode: bool,
    pub display_style: DisplayStyle,
}

impl TuringMachine {
    pub fn with_tape(mut self, tape: TuringTape) -> Self {
        self.tape = tape;
        self
    }

    pub fn with_program(mut self, program: TuringProgram) -> Self {
        self.program = program;
        self
    }

    pub fn with_debug_mode(mut self, display_style: DisplayStyle, delay: Duration) -> Self {
        self.display_style = display_style;
        self.delay = delay;
        self.debug_mode = true;
        self
    }

    pub fn reset_state_information(&mut self) {
        self.tape = TuringTape::default();
        self.state = State::default();
        self.head = 0;
    }
    
    pub fn reset_state_but_persist_tape(&mut self) {
        self.state = State::default();
    }
    
    pub fn set_program(&mut self, program: TuringProgram) {
        self.program = program;
    }

    pub fn set_tape(&mut self, tape: TuringTape) {
        self.tape = tape;
    }

    pub fn set_head(&mut self, head: usize) {
        self.head = head;
    }

    pub fn read(&mut self) -> bool {
        self.tape.read(self.head)
    }

    pub fn set(&mut self) {
        self.tape.set(self.head)
    }

    pub fn unset(&mut self) {
        self.tape.unset(self.head)
    }

    pub fn move_right(&mut self) {
        self.head += 1;
        self.tape.allocate_till_bit_index(self.head);
    }

    pub fn move_left(&mut self) {
        if self.head == 0 {
            self.head = 7;
            self.tape.allocate_left(1);
        } else {
            self.head -= 1;
        }
    }

    pub fn program_step(&mut self) -> bool {
        let current_bit = self.read();
        let current_instruction = self.program.get(self.state, current_bit);
        let instruction = match current_instruction {
            None => {
                if self.state.get() == usize::MAX {
                    return false;
                } else {
                    panic!("Dangling state 'q{} σ={}'", self.state.get(), self.read())
                }
            },
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
                    println!("{}", self.tape.get_string(Some(self.head)))
                },
                DisplayStyle::VisualFormal => {
                    println!(
                        "{} | Head: {} | {}",
                        self.tape.get_string(Some(self.head)),
                        self.head,
                        instruction.get_formal_string()
                    )
                },
                _ => {}
            }
        }
        
        self.state = self.process_instruction(instruction);
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

impl Default for TuringMachine {
    fn default() -> Self {
        Self {
            tape: TuringTape::default(),
            head: 0,
            state: State::default(),
            program: TuringProgram::default(),
            delay: Duration::from_millis(0),
            debug_mode: false,
            display_style: DisplayStyle::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_functionalities() {
        let mut tm = TuringMachine::default();

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
        assert_eq!(tm.head, 7);
        assert!(!tm.read());
    }

    #[test]
    fn test_run_program() {
        let instruction_0 = Instruction::new(State::new(0), false, false)
            .with_movement(Movement::Right)
            .with_next_state(State::new(0));

        let instruction_1 = Instruction::new(State::new(0), true, true)
            .with_movement(Movement::Stay)
            .with_next_state(State::new(usize::MAX));

        // A simple program which will search for the first 1 on the tape
        let mut program = TuringProgram::default();
        program.add_instruction(instruction_0);
        program.add_instruction(instruction_1);

        let mut tape = TuringTape::default();
        tape.set(13);

        let mut tm = TuringMachine::default().with_tape(tape);
        tm.set_program(program);
        tm.run_program();
        
        assert!(tm.read());
        assert_eq!(tm.head, 13);
    }
}