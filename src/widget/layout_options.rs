use super::container::Alignment;
use cherry_macros::Builder;

#[derive(Clone, Copy, Builder, Default)]
pub struct LayoutOptions {
    pub alignment: Option<Alignment>,
    pub grow: u32,
}

impl LayoutOptions {
    pub fn new() -> Self {
        Default::default()
    }
}
