#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Justification {
    Start,
    End,
}

impl Default for Justification {
    fn default() -> Self {
        Self::Start
    }
}
