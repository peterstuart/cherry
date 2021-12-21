use embedded_graphics::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Insets {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

impl Insets {
    pub fn none() -> Self {
        Self {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }

    pub fn all(value: u32) -> Self {
        Self {
            left: value,
            right: value,
            top: value,
            bottom: value,
        }
    }

    pub fn horizontal(value: u32) -> Self {
        Self {
            left: value,
            right: value,
            top: 0,
            bottom: 0,
        }
    }

    pub fn vertical(value: u32) -> Self {
        Self {
            left: 0,
            right: 0,
            top: value,
            bottom: value,
        }
    }
}

impl Default for Insets {
    fn default() -> Self {
        Self::none()
    }
}

pub trait Inset {
    fn inset(&self, insets: Insets) -> Self;
    fn outset(&self, insets: Insets) -> Self;
}

impl Inset for Size {
    fn inset(&self, insets: Insets) -> Self {
        Self::new(
            self.width - insets.left - insets.right,
            self.height - insets.top - insets.bottom,
        )
    }

    fn outset(&self, insets: Insets) -> Self {
        Self::new(
            self.width + insets.left + insets.right,
            self.height + insets.top + insets.bottom,
        )
    }
}
