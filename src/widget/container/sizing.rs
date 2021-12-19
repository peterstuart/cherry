#[derive(Clone, Copy, Eq, PartialEq)]
pub enum Sizing {
    Fit,
    Fill,
    Exact(u32),
}

impl Sizing {
    pub fn with(&self, content: u32, max: u32) -> u32 {
        match self {
            Self::Fit => content,
            Self::Fill => max,
            Self::Exact(size) => *size,
        }
    }
}

impl Default for Sizing {
    fn default() -> Self {
        Self::Fit
    }
}
