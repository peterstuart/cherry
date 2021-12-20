#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Alignment {
    Start,
    Center,
    End,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Start
    }
}
