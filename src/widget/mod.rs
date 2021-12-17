pub mod container;
pub mod text;

use alloc::boxed::Box;
use embedded_graphics::prelude::*;

pub trait Widget<Display: DrawTarget> {
    fn draw(
        &self,
        display: &mut Display,
        origin: Point,
        max_size: Size,
    ) -> Result<Size, Display::Error>;

    fn boxed(self) -> Box<dyn Widget<Display>>
    where
        Self: 'static + Sized,
    {
        Box::new(self)
    }
}
