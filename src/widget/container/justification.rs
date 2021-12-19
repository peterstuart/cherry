#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Justification {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for Justification {
    fn default() -> Self {
        Self::Start
    }
}
