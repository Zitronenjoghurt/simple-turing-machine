pub struct TuringTape {
    tape: Vec<u8>,
    size_bytes: usize,
}

impl TuringTape {
    pub fn new(size_bytes: usize) -> Self {
        Self {
            tape: vec![0; size_bytes],
            size_bytes,
        }
    }

    fn get_byte_index_and_position(&self, bit_index: usize) -> Result<(usize, usize), Box<dyn std::error::Error>> {
        let byte_index = bit_index / 8;
        if byte_index >= self.size_bytes {
            return Err(format!("Bit index '{}' out of range for '{}' bytes", bit_index, self.size_bytes).into());
        }

        let position = bit_index % 8;
        Ok((byte_index, position))
    }

    pub fn read(&self, bit_index: usize) -> Result<bool, Box<dyn std::error::Error>> {
        let (byte_index, position) = self.get_byte_index_and_position(bit_index)?;
        let byte = self.tape[byte_index];
        Ok((byte & (1 << position)) != 0)
    }

    pub fn set(&mut self, bit_index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let (byte_index, position) = self.get_byte_index_and_position(bit_index)?;
        let byte = &mut self.tape[byte_index];
        *byte |= 1 << position;
        Ok(())
    }

    pub fn unset(&mut self, bit_index: usize) -> Result<(), Box<dyn std::error::Error>> {
        let (byte_index, position) = self.get_byte_index_and_position(bit_index)?;
        let byte = &mut self.tape[byte_index];
        *byte &= !(1 << position);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let tape = TuringTape::new(5);
        assert_eq!(tape.size_bytes, 5);
        assert_eq!(tape.tape, vec![0; 5]);
    }

    #[test]
    fn test_set_unset_read() {
        let mut tape = TuringTape::new(4);
        tape.set(7).unwrap();
        tape.set(13).unwrap();
        tape.set(19).unwrap();

        for i in 0..=31 {
            if (i == 7) || (i == 13) || (i == 19) {
                assert!(tape.read(i).unwrap());
            } else {
                assert!(!tape.read(i).unwrap());
            }
        }

        tape.unset(3).unwrap();
        assert!(!tape.read(3).unwrap());
    }

    #[test]
    fn test_out_of_bounds() {
        let mut tape = TuringTape::new(4);

        assert!(tape.read(31).is_ok());
        let read_result = tape.read(32);
        assert!(read_result.is_err());
        assert_eq!(
            read_result.unwrap_err().to_string(),
            "Bit index '32' out of range for '4' bytes"
        );

        assert!(tape.set(31).is_ok());
        let set_result = tape.set(37);
        assert!(set_result.is_err());
        assert_eq!(
            set_result.unwrap_err().to_string(),
            "Bit index '37' out of range for '4' bytes"
        );

        assert!(tape.set(31).is_ok());
        let unset_result = tape.unset(41);
        assert!(unset_result.is_err());
        assert_eq!(
            unset_result.unwrap_err().to_string(),
            "Bit index '41' out of range for '4' bytes"
        );
    }
}