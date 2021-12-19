#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Justification {
    Start,
    Center,
    End,
    SpaceBetween,
}

impl Default for Justification {
    fn default() -> Self {
        Self::Start
    }
}
