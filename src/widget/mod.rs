pub mod container;
pub mod text;

use alloc::boxed::Box;
use embedded_graphics::prelude::*;

pub trait Widget<Display: DrawTarget> {
    /// Returns the intrinsic size of the widget, based on its contents.
    fn intrinsic_size(&self) -> Size;

    /// Draws the widget with the provided `origin` and `size`. If no
    /// `size` is provided, the widget uses its [intrinsic size][Widget::intrinsic_size].
    fn draw(
        &self,
        display: &mut Display,
        origin: Point,
        size: Option<Size>,
    ) -> Result<(), Display::Error>;

    fn boxed(self) -> Box<dyn Widget<Display>>
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}
