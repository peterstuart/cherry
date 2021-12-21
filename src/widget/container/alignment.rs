#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Alignment {
    Stretch,
    Start,
    Center,
    End,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Stretch
    }
}
