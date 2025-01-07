use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Movement {
    #[default]
    Stay,
    Left,
    Right
}

impl Movement {
    pub fn get_code_string(&self) -> &'static str {
        match self {
            Self::Stay => "S",
            Self::Left => "L",
            Self::Right => "R"
        }
    }
}