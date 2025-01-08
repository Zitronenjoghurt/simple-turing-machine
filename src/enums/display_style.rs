use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq, Eq)]
pub enum DisplayStyle {
    None,
    Formal,
    Visual,
    VisualFormal
}