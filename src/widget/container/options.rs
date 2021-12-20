use super::{Alignment, Axis, Border, Insets, Justification};
use crate::widget::Widget;
use alloc::{boxed::Box, vec::Vec};
use embedded_graphics::{draw_target::DrawTarget, primitives::CornerRadii};

pub struct Options<Display>
where
    Display: DrawTarget,
{
    pub alignment: Alignment,
    pub axis: Axis,
    pub background_color: Option<Display::Color>,
    pub border: Option<Border<Display::Color>>,
    pub children: Vec<Box<dyn Widget<Display>>>,
    pub corner_radii: Option<CornerRadii>,
    pub height: Option<u32>,
    pub justification: Justification,
    pub padding: Insets,
    pub width: Option<u32>,
}

impl<Display> Default for Options<Display>
where
    Display: DrawTarget,
{
    fn default() -> Self {
        Self {
            alignment: Default::default(),
            axis: Default::default(),
            background_color: Default::default(),
            border: Default::default(),
            children: Default::default(),
            corner_radii: Default::default(),
            height: Default::default(),
            justification: Default::default(),
            padding: Default::default(),
            width: Default::default(),
        }
    }
}
