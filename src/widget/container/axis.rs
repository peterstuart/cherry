#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Default for Axis {
    fn default() -> Self {
        Self::Vertical
    }
}

impl Axis {
    pub fn opposite(&self) -> Axis {
        match self {
            Self::Horizontal => Self::Vertical,
            Self::Vertical => Self::Horizontal,
        }
    }
}
