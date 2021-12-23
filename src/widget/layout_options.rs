use super::container::Alignment;

#[derive(Clone, Copy, Default)]
pub struct LayoutOptions {
    pub alignment: Option<Alignment>,
    pub grow: u32,
}
