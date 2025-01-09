#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pattern(Vec<bool>);

impl Pattern {
    pub fn new(pattern: Vec<bool>) -> Self {
        Self(pattern)
    }

    pub fn get(&self) -> &[bool] {
        &self.0
    }

    pub fn get_at(&self, index: usize) -> bool {
        self.0[index]
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}