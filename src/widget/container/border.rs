#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Border<Color> {
    pub color: Color,
    pub width: u32,
}
