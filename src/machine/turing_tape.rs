use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct TuringTape {
    tape: VecDeque<u8>,
}

impl TuringTape {
    pub fn allocate_till_bit_index(&mut self, bit_index: usize) {
        let byte_index = bit_index / 8;
        if byte_index >= self.tape.len() {
            self.allocate_right(byte_index - self.tape.len() + 1)
        }
    }

    pub fn get_byte_index_and_position(&mut self, bit_index: usize) -> (usize, usize) {
        let byte_index = bit_index / 8;
        if byte_index >= self.tape.len() {
            self.allocate_right(byte_index - self.tape.len() + 1)
        }

        let position = bit_index % 8;
        (byte_index, position)
    }

    pub fn read(&mut self, bit_index: usize) -> bool {
        let (byte_index, position) = self.get_byte_index_and_position(bit_index);
        let byte = self.tape[byte_index];
        (byte & (1 << position)) != 0
    }

    pub fn set(&mut self, bit_index: usize) {
        let (byte_index, position) = self.get_byte_index_and_position(bit_index);
        let byte = &mut self.tape[byte_index];
        *byte |= 1 << position;
    }

    pub fn unset(&mut self, bit_index: usize) {
        let (byte_index, position) = self.get_byte_index_and_position(bit_index);
        let byte = &mut self.tape[byte_index];
        *byte &= !(1 << position);
    }

    pub fn get_string(&mut self, marked_index: Option<usize>) -> String {
        let mut string = String::new();
        for i in 0..self.tape.len() * 8 {
            if Some(i) == marked_index {
                string.push('[');
            } else {
                string.push(' ');
            }

            if self.read(i) {
                string.push('1');
            } else {
                string.push('0');
            }

            if Some(i) == marked_index {
                string.push(']');
            } else {
                string.push(' ');
            }
        }
        string
    }

    pub fn allocate_left(&mut self, size: usize) {
        for _ in 0..size {
            self.tape.push_front(0);
        }
    }

    pub fn allocate_right(&mut self, size: usize) {
        for _ in 0..size {
            self.tape.push_back(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_unset_read() {
        let mut tape = TuringTape::default();
        tape.set(7);
        tape.set(13);
        tape.set(19);

        for i in 0..=31 {
            if (i == 7) || (i == 13) || (i == 19) {
                assert!(tape.read(i));
            } else {
                assert!(!tape.read(i));
            }
        }

        tape.unset(3);
        assert!(!tape.read(3));
    }
}